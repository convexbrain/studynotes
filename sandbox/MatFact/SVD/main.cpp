
#include "svd.h"

#include <chrono>
#include <iostream>
using std::cout;
using std::cerr;
using std::endl;

//

static std::stringstream nullout;

void test_simple(uint32_t rows, uint32_t cols, uint32_t mt)
{
	MatrixXd G(rows, cols);
	G.setRandom();

	unique_ptr<SVD_IF> pSVD;
	if (mt > 0) pSVD = SVD_Factory::create_OSJ_SVD_MT(rows, cols, mt);
	else pSVD = SVD_Factory::create_OSJ_SVD(rows, cols);

	pSVD->decomp(G);
	pSVD->selftest(G, cout);
}

void test_fit(uint32_t points, uint32_t mt, bool doTest)
{
	MatrixXd P(points, 2);
	P.setRandom();
	P.row(0).setOnes();
	// 最初の点は(1,1)、その他はランダム

	MatrixXd G(points, 3);
	G.col(0) = P.col(0).cwiseProduct(P.col(0));
	G.col(1) = P.col(0);
	G.col(2).setOnes();

	VectorXd h(points);
	h = P.col(1);

	VectorXd a(3);

	unique_ptr<SVD_IF> pSVD;
	if (mt > 0) pSVD = SVD_Factory::create_OSJ_SVD_MT(points, 3, mt);
	else pSVD = SVD_Factory::create_OSJ_SVD(points, 3);

	pSVD->decomp(G);
	if (doTest) {
		bool r = pSVD->selftest(G, cout);
		cout << "selftest" << endl << r << endl;
	}
	pSVD->solve(a, h);

	cout << "x, y, ax, a0, a1, a2" << endl;
	for (uint32_t i = 0; i < points; i++) {
		double ax = a(0) * P(i, 0) * P(i, 0) + a(1) * P(i, 0) + a(2);
		cout << P(i, 0) << ", " << P(i, 1) << ", " << ax << ", ";
		if (0 == i) {
			cout << a(0) << ", " << a(1) << ", " << a(2);
		}
		cout << endl;
	}
}

void test_comp(uint32_t num_max, uint32_t sz_max, uint32_t r_max, uint32_t mt, bool doTest)
{
	VectorXd rc(2);

	cout << "num, rows, cols, time," << endl;
	for (uint32_t i = 0; i < num_max; i++) {
		cout << i << ", ";

		rc.setRandom();
		uint32_t sz = (uint32_t)((rc(0) + 1.0) * 0.5 * sz_max) + 1;
		uint32_t r = (uint32_t)((rc(1) + 1.0) * 0.5 * r_max) + 1;
		uint32_t c = (sz / r) + 1;
		cout << r << ", " << c << ", ";

		MatrixXd G(r, c);
		G.setRandom();

		unique_ptr<SVD_IF> pSVD;
		if (mt > 0) pSVD = SVD_Factory::create_OSJ_SVD_MT(r, c, mt);
		else pSVD = SVD_Factory::create_OSJ_SVD(r, c);

		auto start = std::chrono::system_clock::now();
		pSVD->decomp(G);
		auto end = std::chrono::system_clock::now();
		auto period = std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count();
		cout << period << ", ";

		if (doTest) {
			bool r = pSVD->selftest(G, nullout);
			cout << r;
		}
		cout << endl;
	}
}

int main(int argc, char ** argv)
{
	// CPU負荷率テスト（スレッドなし）
	//test_simple(1000, 1000, 0);

	// CPU負荷率テスト（8スレッド）
	//test_simple(1000, 1000, 8);

	// 計算量テスト（スレッドなし）
	//test_comp(100, 360000, 600, 0, false);

	// 計算量テスト（1スレッド）
	//test_comp(100, 360000, 600, 1, false);

	// 計算量テスト（8スレッド）
	test_comp(100, 360000, 600, 8, false);


	// 二次多項式フィッティング（3点：正則正方）
	//test_fit(3, 0, false);

	// 二次多項式フィッティング（10点：優決定系）
	//test_fit(10, 0, false);

	// 二次多項式フィッティング（1点：劣決定系）
	//test_fit(1, 0, false);


	//test_simple(10, 10, 8);
	//test_fit(3, 0, true);
	//test_comp(10, 3600, 60, 8, true);

	cerr << "Hit Any Key" << endl;
	getchar();
}
