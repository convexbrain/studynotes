
#include "svd.h"

#include <iostream> // for debug
#include <list>
#include <vector>
#include <thread>
#include <future>
#include <condition_variable>
using std::list;
using std::vector;
using std::thread;
using std::mutex;
using std::unique_lock;
using std::condition_variable;

enum ColPairTerm {
	CPTERM_NONE = 0,
	CPTERM_PAR,
	CPTERM_LOOP
};

struct ColPair {
	uint32_t col1;
	uint32_t col2;
	ColPairTerm term;
};

template <class T>
class MsgBox {
protected:
	mutex m_mtx;
	condition_variable m_cond_empty;
	condition_variable m_cond_full;
	T m_msg;
	bool m_valid;

public:
	MsgBox() : m_valid(false) {}
	~MsgBox() {}

	void put(T msg)
	{
		{
			unique_lock<mutex> lk(m_mtx);
			m_cond_empty.wait(lk, [this] { return !m_valid; });
			m_valid = true;
			m_msg = msg;
		}
		m_cond_full.notify_one();
	}

	T get(void)
	{
		T msg;
		{
			unique_lock<mutex> lk(m_mtx);
			m_cond_full.wait(lk, [this] { return m_valid; });
			msg = m_msg;
			m_valid = false;
		}
		m_cond_empty.notify_one();
		return msg;
	}
};

OSJ_SVD_MT::OSJ_SVD_MT(MatrixXd_IN G) : IF_SVD(G)
{
	if (G.rows() < G.cols()) {
		m_tr = true;
		m_U = G.transpose();
	}
	else {
		m_tr = false;
		m_U = G;
	}

	m_S = VectorXd(m_U.cols());
	m_S.setZero();

	m_V = MatrixXd(m_U.cols(), m_U.cols());
	m_V.setIdentity();
}

list<ColPair> lsColPair;
mutex mtxLsColPair;
const uint32_t th_num = 3;
thread th[th_num];
MsgBox<bool> msgToSlave[th_num];
MsgBox<bool> msgToMaster[th_num];

void work(uint32_t id, MsgBox<bool> *pMsgtoSlave, MsgBox<bool> *pMsgtoMaster)
{
	bool global_conv = false;
	bool local_conv = true;

	while (!global_conv) {
		ColPair cp;
		{
			unique_lock<mutex> lk(mtxLsColPair);

			cp = lsColPair.front();
			if (!cp.term) {
				lsColPair.pop_front();
				lsColPair.push_back(cp);
			}
			std::cout << id << ":" << cp.col1 << "," << cp.col2 << "," << cp.term << endl;
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
			std::this_thread::sleep_for(std::chrono::seconds(1));
			// TODO: process
			//local_conv = false;
			break;
		}
	}
}

bool OSJ_SVD_MT::decomp(void)
{
	uint32_t m = m_U.rows();
	uint32_t n = m_U.cols();

	{
		vector<bool> isUsedCol;
		isUsedCol.resize(n);
		for (uint32_t i = 0; i < n; i++) {
			isUsedCol[i] = false;
		}

		for (uint32_t i = 0; i < n - 1; i++) {
			for (uint32_t j = i + 1; j < n; j++) {
				lsColPair.push_back({ i, j, CPTERM_NONE });
			}
		}
		lsColPair.push_back({ 0, 0, CPTERM_LOOP });

		auto cp = lsColPair.begin();
		while (1) {
			if (cp->term) {
				if (cp == lsColPair.begin()) {
					lsColPair.push_back({ 0, 0, CPTERM_LOOP });
					break;
				}
				else {
					lsColPair.push_back({ 0, 0, CPTERM_PAR });
					for (uint32_t i = 0; i < n; i++) {
						isUsedCol[i] = false;
					}
					cp = lsColPair.begin();
				}
			}
			else if (!isUsedCol[cp->col1] && !isUsedCol[cp->col2]) {
				isUsedCol[cp->col1] = true;
				isUsedCol[cp->col2] = true;

				lsColPair.push_back(*cp);
				cp = lsColPair.erase(cp);
			}
			else {
				cp++;
			}
		}
		lsColPair.pop_front(); // remove unnecessary CPTERM_LOOP
	}

#if 0
	std::for_each(lsColPair.begin(), lsColPair.end(), [](ColPair cp){
		std::cout << cp.col1 << "," << cp.col2 << "," << cp.term << endl;
	});
#endif

	{
		for (uint32_t i = 0; i < th_num; i++) {
			th[i] = thread(work, i, &msgToSlave[i], &msgToMaster[i]);
		}

		while (1) {
			bool global_conv = true;

			for (uint32_t i = 0; i < th_num; i++) {
				bool local_conv = msgToMaster[i].get();
				global_conv = (global_conv && local_conv);
			}

			ColPair cp = lsColPair.front();
			lsColPair.pop_front();
			lsColPair.push_back(cp);

			std::cout << "master:" << cp.term << endl;
			std::this_thread::sleep_for(std::chrono::seconds(1));

			for (uint32_t i = 0; i < th_num; i++) {
				msgToSlave[i].put(global_conv);
			}

			if ((CPTERM_LOOP == cp.term) && global_conv) {
				break;
			}
		}

		for (uint32_t i = 0; i < th_num; i++) {
			th[i].join();
		}
	}

	return true;
}

#if 0
bool OSJ_SVD_MT::decomp(void)
{
	uint32_t m = m_U.rows();
	uint32_t n = m_U.cols();

	if ((m == 0) || (n == 0)) {
		return false;
	}

	bool converged;
	do {
		converged = true;

		for (uint32_t i = 0; i < n - 1; i++) {
			for (uint32_t j = i + 1; j < n; j++) {
				double a = m_U.col(i).squaredNorm();
				double b = m_U.col(j).squaredNorm();
				double c = m_U.col(i).dot(m_U.col(j));

				if (c * c > m_tol * m_tol * a * b) converged = false;

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
		}

	} while (!converged);

	for (uint32_t i = 0; i < n; i++) {
		double s = m_U.col(i).norm();
		m_S(i) = s;

		if ((-m_thr < s) && (s < m_thr)) continue;

		m_U.col(i).normalize();
	}

	return true;
}
#endif

double OSJ_SVD_MT::test(MatrixXd_IN G, ostream &out)
{
	if ((m_U.rows() == 0) || (m_U.cols() == 0)) {
		return true;
	}

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

	return diff;
}
