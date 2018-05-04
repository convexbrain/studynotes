#ifndef _SVD_H_
#define _SVD_H_

#include <cstdint>
#include <cstdbool>
#include <cstddef>

#include "Eigen"
using Eigen::MatrixXd;
using Eigen::VectorXd;
typedef const Eigen::Ref<const MatrixXd> MatrixXd_IN;
typedef const Eigen::Ref<const VectorXd> VectorXd_IN;
typedef Eigen::Ref<MatrixXd> MatrixXd_IO;
typedef Eigen::Ref<VectorXd> VectorXd_IO;

#include <ostream>
using std::ostream;

#include <memory>
using std::unique_ptr;

//

class SVD_IF {
private:
	bool good(MatrixXd_IN G) {
		if ((0 == G.rows()) || (0 == G.cols()) ||
			(m_rows != G.rows()) || (m_cols != G.cols())
			) {
			return false;
		}
		return true;
	}

protected:
	uint32_t m_rows;
	uint32_t m_cols;
	bool m_decomped;

	virtual void do_decomp(MatrixXd_IN G) = 0;
	virtual bool do_selftest(MatrixXd_IN G, ostream &out) = 0;

public:
	explicit SVD_IF(uint32_t rows, uint32_t cols) :
		m_rows(rows), m_cols(cols), m_decomped(false) { /*std::cout << "SVD_IF()" << std::endl;*/ }
	virtual ~SVD_IF() { /*std::cout << "~SVD_IF()" << std::endl;*/ }

	bool decomp(MatrixXd_IN G)
	{
		if (!good(G)) return false;
		do_decomp(G);
		m_decomped = true;
		return true;
	}

	bool selftest(MatrixXd_IN G, ostream &out)
	{
		if (!good(G) || !m_decomped) return false;
		return do_selftest(G, out);
	}

	// bool solve(VectorXd_IO x, VectorXd_IN h) = 0; // TODO
};

//

class SVD_Factory {
private:
	SVD_Factory() {}
	~SVD_Factory() {}
public:
	static unique_ptr<SVD_IF> create_OSJ_SVD(uint32_t rows, uint32_t cols);
	static unique_ptr<SVD_IF> create_OSJ_SVD_MT(uint32_t rows, uint32_t cols, uint32_t th_num = 1);
};

#endif
