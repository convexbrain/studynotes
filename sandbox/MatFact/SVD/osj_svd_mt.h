#ifndef _OSJ_SVD_MT_H_
#define _OSJ_SVD_MT_H_

#include "svd.h"

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

#include <iostream> // for debug

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
	MsgBox() : m_valid(false) { /*std::cout << "MsgBox" << std::endl;*/ }
	MsgBox(const MsgBox<T> &cp) : MsgBox() { /*std::cout << "MsgBox(cp)" << std::endl;*/ }
	~MsgBox() { /*std::cout << "~MsgBox" << std::endl;*/ }

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

class OSJ_SVD_MT : public IF_SVD {
private:
	const double m_tol = DBL_EPSILON;
	const double m_thr = DBL_MIN;

	bool m_tr;
	MatrixXd m_U;
	VectorXd m_S;
	MatrixXd m_V;

	list<ColPair> m_lsColPair;
	mutex m_mtxLsColPair;
	uint32_t m_thNum;
	vector<thread> m_vecThread;
	vector< MsgBox<bool> > m_vecMsgToSlave;
	vector< MsgBox<bool> > m_vecMsgToMaster;

	static void thread_work(OSJ_SVD_MT *pThis, uint32_t th_id);
	void work(uint32_t th_id);

public:
	explicit OSJ_SVD_MT(MatrixXd_IN G, uint32_t th_num = 1);
	virtual ~OSJ_SVD_MT() {}

	virtual bool decomp(void);
	virtual double test(MatrixXd_IN G, ostream &out);
};

#endif
