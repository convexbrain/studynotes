#include <cstdint>
#include <cstdbool>
#include <cstddef>

#include <cmath>
#include <cfloat>
#include <cstdio>

#include <iostream>
#include <chrono>

#include "../../../submodules/eigen-git-mirror/Eigen/Dense"

using std::cout;
using std::cerr;
using std::endl;
using std::ostream;
using std::stringstream;

using Eigen::MatrixXd;
using Eigen::VectorXd;
typedef const Eigen::Ref<const MatrixXd> MatrixXd_IN;
typedef Eigen::Ref<MatrixXd> MatrixXd_IO;

static std::stringstream nullout;

class OSJ_SVD {
private:
	const double m_tol = DBL_EPSILON;
	const double m_thr = DBL_MIN;

	bool m_tr;
	MatrixXd m_U;
	VectorXd m_S;
	MatrixXd m_V;

public:
	OSJ_SVD(MatrixXd_IN G)
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

	bool decomp(void)
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

	double test(MatrixXd_IN G, ostream &out)
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
};

void test1(void)
{
	MatrixXd G(4, 6);
	//G.setIdentity();
	G.setRandom();

	//G.row(1) = G.row(0) * G(0, 0);
	//G.row(3) = G.row(2) * G(0, 1);

	OSJ_SVD d(G);
	d.decomp();

	d.test(G, cout);
}

void test2(bool doTest)
{
	VectorXd rc(2);

	cout << "num, rows, cols, period, diff" << endl;
	for (uint32_t i = 0; i < 100; i++) {
		cout << i << ", ";

		rc.setRandom();
		uint32_t sz = (uint32_t)((rc(0) + 1.0) * 0.5 * 300000) + 1;
		uint32_t r = (uint32_t)((rc(1) + 1.0) * 0.5 * 500) + 1;
		uint32_t c = (sz / r) + 1;
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

		if (doTest) {
			double diff = d.test(G, nullout);
			cout << diff;
		}
		cout << endl;
	}
}

int main(int argc, char ** argv)
{
	//test1();
	test2(false);

	cerr << "Hit Any Key" << endl;
	getchar();
}
