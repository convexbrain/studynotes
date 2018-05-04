#ifndef _OSJ_SVD_MT_H_
#define _OSJ_SVD_MT_H_

#include "osj_svd.h"

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
	CPTERM_SYNC,
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

class OSJ_SVD_MT : public OSJ_SVD {
private:
	list<ColPair> m_lsColPair;
	mutex m_mtxLsColPair;
	uint32_t m_thNum;
	vector<thread> m_vecThread;
	vector< MsgBox<bool> > m_vecMsgToSlave;
	vector< MsgBox<bool> > m_vecMsgToMaster;

	static void thread_work(OSJ_SVD_MT *pThis, uint32_t th_id) { pThis->work(th_id); }
	void work(uint32_t th_id);

	void makeLsColPair(void);

protected:
	virtual void do_decomp(MatrixXd_IN G);
	virtual bool do_selftest(MatrixXd_IN G, ostream &out) { return OSJ_SVD::do_selftest(G, out); }

public:
	explicit OSJ_SVD_MT(uint32_t rows, uint32_t cols, uint32_t th_num = 1);
	virtual ~OSJ_SVD_MT() { /*std::cout << "~OSJ_SVD_MT()" << std::endl;*/ }
};

#endif
