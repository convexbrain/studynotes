#include <cstdint>
#include <cstdbool>
#include <cstddef>

#include <cmath>
#include <cfloat>
#include <cstdio>
#include <iostream>

#include <Eigen/Dense>

using std::cout;
using std::endl;
using Eigen::MatrixXd;
using Eigen::VectorXd;

#if 1
#define dout cout
#else
std::stringstream dout;
#endif

int main(int argc, char ** argv)
{
	MatrixXd G(5, 5);
	//G.setIdentity();
	G.setRandom();

	bool tr;
	MatrixXd U;
	VectorXd S;
	MatrixXd V;
	{
		const double tol2 = DBL_EPSILON * DBL_EPSILON;
		const double thr = DBL_MIN;

		if (G.rows() < G.cols()) {
			tr = true;
			U = G.transpose();
		}
		else {
			tr = false;
			U = G;
		}

		uint32_t m = U.rows();
		uint32_t n = U.cols();

		S = VectorXd(n);
		S.setZero();

		V = MatrixXd(n, n);
		V.setIdentity();

		bool converged;
		do {
			dout << "===" << endl;

			converged = true;

			for (uint32_t i = 0; i < n - 1; i++) {
				for (uint32_t j = i + 1; j < n; j++) {
					dout << "i:" << i << ", ";
					dout << "j:" << j << ", ";

					double a = U.col(i).squaredNorm();
					double b = U.col(j).squaredNorm();
					double c = U.col(i).dot(U.col(j));

					dout << "a:" << a << ", ";
					dout << "b:" << b << ", ";
					dout << "c:" << c << ", ";

					if (c * c > tol2 * a * b) converged = false;

					if ((c < -thr) || (thr < c)) {
						double zeta = (b - a) / (2.0 * c);
						double t;
						if (zeta > 0) t = 1.0 / (zeta + sqrt(1 + zeta * zeta));
						else          t = -1.0 / (-zeta + sqrt(1 + zeta * zeta));
						double cs = 1.0 / sqrt(1.0 + t * t);
						double sn = cs * t;

						dout << "zeta:" << zeta << ", ";
						dout << "t:" << t << ", ";
						dout << "cs:" << cs << ", ";
						dout << "sn:" << sn << ", ";

						for (uint32_t k = 0; k < m; k++) {
							double tmp = U(k, i);
							U(k, i) = cs * tmp - sn * U(k, j);
							U(k, j) = sn * tmp + cs * U(k, j);
						}

						for (uint32_t k = 0; k < n; k++) {
							double tmp = V(k, i);
							V(k, i) = cs * tmp - sn * V(k, j);
							V(k, j) = sn * tmp + cs * V(k, j);
						}
					}

					dout << endl;
				}
			}

		} while (!converged);

		for (uint32_t i = 0; i < n; i++) {
			double s = U.col(i).norm();
			S(i) = s;

			if ((-thr < s) && (s < thr)) continue;

			U.col(i).normalize();
		}
	}

	MatrixXd Gr;
	if (tr) Gr = V * S.asDiagonal() * U.transpose();
	else Gr = U * S.asDiagonal() * V.transpose();

	cout << endl;
	cout << "--- G" << endl << G << endl;
	cout << endl;
	if (tr) cout << "SVD of G -> V * diag(S) * U'" << endl;
	else cout << "SVD of G -> U * diag(S) * V'" << endl;
	cout << endl;
	cout << "--- U" << endl << U << endl;
	cout << "--- S" << endl << S << endl;
	cout << "--- V" << endl << V << endl;
	cout << endl;
	cout << "--- G reconstructed" << endl << Gr << endl;
	cout << "--- U' * U" << endl << U.transpose() * U << endl;
	cout << "--- V' * V" << endl << V.transpose() * V << endl;
	getchar();
}