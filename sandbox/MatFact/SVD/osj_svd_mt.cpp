
#include "osj_svd_mt.h"

using std::endl;

//

OSJ_SVD_MT::OSJ_SVD_MT(uint32_t rows, uint32_t cols, uint32_t th_num) : SVD_IF(rows, cols)
{
	if (rows < cols) {
		m_tr = true;
		m_U = MatrixXd(cols, rows);
	}
	else {
		m_tr = false;
		m_U = MatrixXd(rows, cols);
	}

	m_S = VectorXd(m_U.cols());
	m_S.setZero();

	m_V = MatrixXd(m_U.cols(), m_U.cols());
	m_V.setIdentity();

	m_thNum = (th_num >= 1) ? th_num : 1;

	m_vecThread.resize(m_thNum);
	m_vecMsgToSlave.resize(m_thNum);
	m_vecMsgToMaster.resize(m_thNum);
}

void OSJ_SVD_MT::thread_work(OSJ_SVD_MT *pThis, uint32_t th_id)
{
	pThis->work(th_id);
}

void OSJ_SVD_MT::work(uint32_t th_id)
{
	MsgBox<bool> *pMsgtoSlave = &(m_vecMsgToSlave[th_id]);
	MsgBox<bool> *pMsgtoMaster = &(m_vecMsgToMaster[th_id]);

	bool global_conv = false;
	bool local_conv = true;

	while (!global_conv) {
		ColPair cp;
		{
			unique_lock<mutex> lk(m_mtxLsColPair);

			cp = m_lsColPair.front();
			if (!cp.term) {
				m_lsColPair.pop_front();
				m_lsColPair.push_back(cp);
			}
			//std::cout << th_id << ":" << cp.col1 << "," << cp.col2 << "," << cp.term << endl;
		}

		switch (cp.term) {
		case CPTERM_PAR:
			pMsgtoMaster->put(false);
			pMsgtoSlave->get();
			break;
		case CPTERM_LOOP:
			pMsgtoMaster->put(local_conv);
			global_conv = pMsgtoSlave->get();
			local_conv = true;
			break;
		case CPTERM_NONE:
		default:
			{
				// TODO
				uint32_t m = m_U.rows();
				uint32_t n = m_U.cols();
				uint32_t i = cp.col1;
				uint32_t j = cp.col2;

				double a = m_U.col(i).squaredNorm();
				double b = m_U.col(j).squaredNorm();
				double c = m_U.col(i).dot(m_U.col(j));

				if (c * c > m_tol * m_tol * a * b) local_conv = false;

				if ((c < -m_thr) || (m_thr < c)) {
					double zeta = (b - a) / (2.0 * c);
					double t;
					if (zeta > 0) t = 1.0 / (zeta + sqrt(1 + zeta * zeta));
					else          t = -1.0 / (-zeta + sqrt(1 + zeta * zeta));
					double cs = 1.0 / sqrt(1.0 + t * t);
					double sn = cs * t;

					for (uint32_t k = 0; k < m; k++) {
						double tmp = m_U(k, i);
						m_U(k, i) = cs * tmp - sn * m_U(k, j);
						m_U(k, j) = sn * tmp + cs * m_U(k, j);
					}

					for (uint32_t k = 0; k < n; k++) {
						double tmp = m_V(k, i);
						m_V(k, i) = cs * tmp - sn * m_V(k, j);
						m_V(k, j) = sn * tmp + cs * m_V(k, j);
					}
				}
			}
			//std::this_thread::sleep_for(std::chrono::seconds(1));
			// TODO: process
			//local_conv = false;
			break;
		}
	}
}

void OSJ_SVD_MT::do_decomp(MatrixXd_IN G)
{
	uint32_t m = m_U.rows();
	uint32_t n = m_U.cols();

	if (m_tr) {
		m_U = G.transpose();
	}
	else {
		m_U = G;
	}

	{
		vector<bool> isUsedCol;
		isUsedCol.resize(n);
		for (uint32_t i = 0; i < n; i++) {
			isUsedCol[i] = false;
		}

		m_lsColPair.clear();
		for (uint32_t i = 0; i < n - 1; i++) {
			for (uint32_t j = i + 1; j < n; j++) {
				m_lsColPair.push_back({ i, j, CPTERM_NONE });
			}
		}
		m_lsColPair.push_back({ 0, 0, CPTERM_LOOP });

		auto cp = m_lsColPair.begin();
		while (1) {
			if (cp->term) {
				if (cp == m_lsColPair.begin()) {
					m_lsColPair.push_back({ 0, 0, CPTERM_LOOP });
					break;
				}
				else {
					m_lsColPair.push_back({ 0, 0, CPTERM_PAR });
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

	{
		for (uint32_t i = 0; i < m_thNum; i++) {
			m_vecThread[i] = thread(thread_work, this, i);
		}

		while (1) {
			bool global_conv = true;

			for (uint32_t i = 0; i < m_thNum; i++) {
				bool local_conv = m_vecMsgToMaster[i].get();
				global_conv = (global_conv && local_conv);
			}

			ColPair cp = m_lsColPair.front();
			m_lsColPair.pop_front();
			m_lsColPair.push_back(cp);

			//std::cout << "master:" << cp.term << endl;
			//std::this_thread::sleep_for(std::chrono::seconds(1));

			for (uint32_t i = 0; i < m_thNum; i++) {
				m_vecMsgToSlave[i].put(global_conv);
			}

			if ((CPTERM_LOOP == cp.term) && global_conv) {
				break;
			}
		}

		for (uint32_t i = 0; i < m_thNum; i++) {
			m_vecThread[i].join();
		}

		// TODO
		for (uint32_t i = 0; i < n; i++) {
			double s = m_U.col(i).norm();
			m_S(i) = s;

			if ((-m_thr < s) && (s < m_thr)) continue;

			m_U.col(i).normalize();
		}
	}
}

bool OSJ_SVD_MT::do_selftest(MatrixXd_IN G, ostream &out)
{
	MatrixXd Gr;
	if (m_tr) Gr = m_V * m_S.asDiagonal() * m_U.transpose();
	else Gr = m_U * m_S.asDiagonal() * m_V.transpose();

	MatrixXd UtU;
	UtU = m_U.transpose() * m_U;
	MatrixXd VVt;
	VVt = m_V * m_V.transpose();

	out << endl;
	out << "--- G" << endl << G << endl;
	out << endl;
	out << "--- tr" << endl << m_tr << endl;
	out << "--- U" << endl << m_U << endl;
	out << "--- S" << endl << m_S << endl;
	out << "--- V" << endl << m_V << endl;
	out << endl;
	out << "--- G reconstructed" << endl << Gr << endl;
	out << "--- U' * U" << endl << UtU << endl;
	out << "--- V * V'" << endl << VVt << endl;

	Gr -= G;
	double diff = Gr.norm();
	out << "--- diff" << endl << diff << endl;

	MatrixXd I;
	I = MatrixXd(UtU.rows(), UtU.cols());
	I.setIdentity();
	I -= UtU;
	double difU = I.norm();
	out << "--- difU" << endl << difU << endl;

	I = MatrixXd(VVt.rows(), VVt.cols());
	I.setIdentity();
	I -= VVt;
	double difV = I.norm();
	out << "--- difV" << endl << difV << endl;

	VectorXd Sinv = m_S.cwiseInverse();
	for (Eigen::Index i = 0; i < Sinv.size(); i++) {
		if ((-m_tol < m_S(i)) && (m_S(i) < m_tol)) Sinv(i) = 0;
	}
	out << "--- Sinv" << endl << Sinv << endl;

	MatrixXd IG;
	if (m_tr) IG = m_U * Sinv.asDiagonal() * m_V.transpose() * G;
	else IG = m_V * Sinv.asDiagonal() * m_U.transpose() * G;
	out << "--- IG" << endl << IG << endl;
	out << "--- G * IG" << endl << G * IG << endl;

	return diff < 1e-10; // TODO
}
