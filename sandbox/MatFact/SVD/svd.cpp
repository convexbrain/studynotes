#include <cstdint>
#include <cstdbool>
#include <cstddef>

#include <cmath>
#include <cfloat>
#include <cstdio>

#include <iostream>
#include <chrono>

#include <Eigen/Dense>

using std::cout;
using std::cerr;
using std::endl;
using std::ostream;
using std::stringstream;

using Eigen::MatrixXd;
using Eigen::VectorXd;
typedef const Eigen::Ref<const MatrixXd> MatrixXd_IN;
typedef Eigen::Ref<MatrixXd> MatrixXd_IO;

class OSJ_SVD {
private:
	const double m_tol2 = DBL_EPSILON * DBL_EPSILON;
	const double m_thr = DBL_MIN;
	std::stringstream m_nullout;

	bool m_tr;
	MatrixXd m_U;
	VectorXd m_S;
	MatrixXd m_V;

	ostream *m_pDout;

public:
	OSJ_SVD(MatrixXd_IN G, ostream *out = NULL) :
		m_tr(false), m_pDout(out)
	{
		if (out == NULL) m_pDout = &m_nullout;

		if (G.rows() < G.cols()) {
			m_tr = true;
			m_U = G.transpose();
		}
		else {
			m_U = G;
		}

		m_S = VectorXd(m_U.cols());
		m_S.setZero();

		m_V = MatrixXd(m_U.cols(), m_U.cols());
		m_V.setIdentity();
	}

	bool decomp(void)
	{
		uint32_t m = m_U.rows();
		uint32_t n = m_U.cols();

		if ((m == 0) || (n == 0)) {
			return false;
		}

		bool converged;
		do {
			*m_pDout << "===" << endl;

			converged = true;

			for (uint32_t i = 0; i < n - 1; i++) {
				for (uint32_t j = i + 1; j < n; j++) {
					*m_pDout << "i:" << i << ", ";
					*m_pDout << "j:" << j << ", ";

					double a = m_U.col(i).squaredNorm();
					double b = m_U.col(j).squaredNorm();
					double c = m_U.col(i).dot(m_U.col(j));

					*m_pDout << "a:" << a << ", ";
					*m_pDout << "b:" << b << ", ";
					*m_pDout << "c:" << c << ", ";

					if (c * c > m_tol2 * a * b) converged = false;

					if ((c < -m_thr) || (m_thr < c)) {
						double zeta = (b - a) / (2.0 * c);
						double t;
						if (zeta > 0) t = 1.0 / (zeta + sqrt(1 + zeta * zeta));
						else          t = -1.0 / (-zeta + sqrt(1 + zeta * zeta));
						double cs = 1.0 / sqrt(1.0 + t * t);
						double sn = cs * t;

						*m_pDout << "zeta:" << zeta << ", ";
						*m_pDout << "t:" << t << ", ";
						*m_pDout << "cs:" << cs << ", ";
						*m_pDout << "sn:" << sn << ", ";

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

					*m_pDout << endl;
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

	double test(MatrixXd_IN G)
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

		*m_pDout << endl;
		*m_pDout << "--- G" << endl << G << endl;
		*m_pDout << endl;
		*m_pDout << "--- tr" << endl << m_tr << endl;
		*m_pDout << "--- U" << endl << m_U << endl;
		*m_pDout << "--- S" << endl << m_S << endl;
		*m_pDout << "--- V" << endl << m_V << endl;
		*m_pDout << endl;
		*m_pDout << "--- G reconstructed" << endl << Gr << endl;
		*m_pDout << "--- U' * U" << endl << UtU << endl;
		*m_pDout << "--- V * V'" << endl << VVt << endl;

		Gr -= G;
		double diff = Gr.norm();
		*m_pDout << "--- diff" << endl << diff << endl;

		MatrixXd I;
		I = MatrixXd(UtU.rows(), UtU.cols());
		I.setIdentity();
		I -= UtU;
		double difU = I.norm();
		*m_pDout << "--- difU" << endl << difU << endl;

		I = MatrixXd(VVt.rows(), VVt.cols());
		I.setIdentity();
		I -= VVt;
		double difV = I.norm();
		*m_pDout << "--- difV" << endl << difV << endl;

		return diff;
	}
};

void test1(void)
{
	MatrixXd G(3, 3);
	//G.setIdentity();
	G.setRandom();

	OSJ_SVD d(G, &cout);
	d.decomp();

	d.test(G);
}

void test2(void)
{
	VectorXd rc(2);

	cout << "num, rows, cols, period, diff" << endl;
	for (uint32_t i = 0; i < 100; i++) {
		cout << i << ", ";

		rc.setRandom();
		uint32_t r = (uint32_t)((rc(0) + 1.0) * 0.5 * 500) + 1;
		uint32_t c = (uint32_t)((rc(1) + 1.0) * 0.5 * 500) + 1;
		cout << r << ", " << c << ", ";

		MatrixXd G(r, c);
		//G.setIdentity();
		G.setRandom();

		OSJ_SVD d(G);
		auto start = std::chrono::system_clock::now();
		d.decomp();
		auto end = std::chrono::system_clock::now();
		auto period = std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count();
		cout << period << ", ";

		double diff = d.test(G);
		cout << diff << endl;
	}
}

int main(int argc, char ** argv)
{
#if 0
	test1();

	cerr << "Hit Any Key" << endl;
	getchar();
#endif
#if 1
	test2();

	cerr << "Hit Any Key" << endl;
	getchar();
#endif
}
