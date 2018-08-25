#ifndef _OSJ_SVD_H_
#define _OSJ_SVD_H_

#include "svd.h"
#include <cfloat>

//

class OSJ_SVD : public SVD_IF {
protected:
	const double m_tol_cnv2 = DBL_EPSILON * DBL_EPSILON;
	const double m_tol_div0 = DBL_MIN;
	const double m_tol_sinv = DBL_EPSILON;
	const double m_tol_rmse = 1.0 / (1LL << 32);

	bool m_tr;
	MatrixXd m_U;
	VectorXd m_S;
	MatrixXd m_V;

	void initMats(MatrixXd_IN G);
	bool applyJacobiRot(uint32_t c1, uint32_t c2);
	void normSingular(void);

protected:
	virtual void do_decomp(MatrixXd_IN G);
	virtual bool do_selftest(MatrixXd_IN G, ostream &out);
	virtual void do_solve(VectorXd_IO x, VectorXd_IN h);

public:
	explicit OSJ_SVD(uint32_t rows, uint32_t cols);
	virtual ~OSJ_SVD() { /*std::cout << "~OSJ_SVD()" << std::endl;*/ }
};

#endif
