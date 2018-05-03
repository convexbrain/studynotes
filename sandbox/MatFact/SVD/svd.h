#ifndef _SVD_H_
#define _SVD_H_

#include <cstdint>
#include <cstdbool>
#include <cstddef>

#include "../../../submodules/eigen-git-mirror/Eigen/Dense"
using Eigen::MatrixXd;
using Eigen::VectorXd;
typedef const Eigen::Ref<const MatrixXd> MatrixXd_IN;
typedef Eigen::Ref<MatrixXd> MatrixXd_IO;

#include <ostream>
using std::ostream;
using std::endl;

class IF_SVD {
public:
	explicit IF_SVD(MatrixXd_IN G) {}
	virtual ~IF_SVD() {}

	virtual bool decomp(void) = 0;
	virtual double test(MatrixXd_IN G, ostream &out) = 0;
};


class OSJ_SVD : public IF_SVD {
private:
	const double m_tol = DBL_EPSILON;
	const double m_thr = DBL_MIN;

	bool m_tr;
	MatrixXd m_U;
	VectorXd m_S;
	MatrixXd m_V;

public:
	explicit OSJ_SVD(MatrixXd_IN G);
	virtual ~OSJ_SVD() {}

	virtual bool decomp(void);
	virtual double test(MatrixXd_IN G, ostream &out);
};

#endif
