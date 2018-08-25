
#include "osj_svd_mt.h"

using std::endl;

//

OSJ_SVD_MT::OSJ_SVD_MT(uint32_t rows, uint32_t cols, uint32_t th_num) : OSJ_SVD(rows, cols)
{
	m_thNum = (th_num >= 1) ? th_num : 1;

	m_vecThread.resize(m_thNum);
	m_vecMsgToSlave.resize(m_thNum);
	m_vecMsgToMaster.resize(m_thNum);
}

void OSJ_SVD_MT::work(uint32_t th_id)
{
	MsgBox<bool> *pMsgtoSlave = &(m_vecMsgToSlave[th_id]);
	MsgBox<bool> *pMsgtoMaster = &(m_vecMsgToMaster[th_id]);

	bool converged_all = false;
	bool converged_local = true;

	while (!converged_all) {
		ColPair cp;
		{
			unique_lock<mutex> lk(m_mtxLsColPair);

			cp = *m_pColPair;
			if (!cp.term) {
				m_pColPair++;
			}
		}

		switch (cp.term) {
		case CPTERM_SYNC:
			pMsgtoMaster->put(false);
			pMsgtoSlave->get();
			break;
		case CPTERM_LOOP:
			pMsgtoMaster->put(converged_local);
			converged_all = pMsgtoSlave->get();
			converged_local = true;
			break;
		case CPTERM_NONE:
		default:
			if (!applyJacobiRot(cp.col1, cp.col2)) converged_local = false;
			break;
		}
	}
}

void OSJ_SVD_MT::makeLsColPair(void)
{
	uint32_t n = m_U.cols();

	// initialize isUsedCol
	vector<bool> isUsedCol;
	isUsedCol.resize(n);
	for (uint32_t i = 0; i < n; i++) {
		isUsedCol[i] = false;
	}

	// initialize m_lsColPair
	m_lsColPair.clear();
	for (uint32_t i = 0; i < n - 1; i++) {
		for (uint32_t j = i + 1; j < n; j++) {
			m_lsColPair.push_back({ i, j, CPTERM_NONE });
		}
	}
	m_lsColPair.push_back({ 0, 0, CPTERM_LOOP });

	// construct m_lsColPair
	auto cp = m_lsColPair.begin();
	while (1) {
		if (cp->term) {
			if (cp == m_lsColPair.begin()) {
				m_lsColPair.push_back({ 0, 0, CPTERM_LOOP });
				break;
			}
			else {
				m_lsColPair.push_back({ 0, 0, CPTERM_SYNC });
				for (uint32_t i = 0; i < n; i++) {
					isUsedCol[i] = false;
				}
				cp = m_lsColPair.begin();
			}
		}
		else if (!isUsedCol[cp->col1] && !isUsedCol[cp->col2]) {
			isUsedCol[cp->col1] = true;
			isUsedCol[cp->col2] = true;

			m_lsColPair.push_back(*cp);
			cp = m_lsColPair.erase(cp);
		}
		else {
			cp++;
		}
	}
	m_lsColPair.pop_front(); // remove unnecessary CPTERM_LOOP
}


void OSJ_SVD_MT::do_decomp(MatrixXd_IN G)
{
	initMats(G);

	// make concurrent column pairs
	makeLsColPair();
	m_pColPair = m_lsColPair.begin();

	// run Slave threads
	for (uint32_t i = 0; i < m_thNum; i++) {
		m_vecThread[i] = thread(thread_work, this, i);
	}

	m_iters = 0;
	while (1) {
		bool converged_all = true;

		// wait for Slave threads SYNC/LOOP
		for (uint32_t i = 0; i < m_thNum; i++) {
			bool local_conv = m_vecMsgToMaster[i].get();
			converged_all = (converged_all && local_conv);
		}

		ColPairTerm term = m_pColPair->term;
		m_pColPair++;

		if (CPTERM_LOOP == term) {
			m_pColPair = m_lsColPair.begin();
			m_iters++;
		}

		// continue Slave threads
		for (uint32_t i = 0; i < m_thNum; i++) {
			m_vecMsgToSlave[i].put(converged_all);
		}

		if ((CPTERM_LOOP == term) && converged_all) {
			break;
		}
	}

	// join Slave threads
	for (uint32_t i = 0; i < m_thNum; i++) {
		m_vecThread[i].join();
	}

	normSingular();
}
