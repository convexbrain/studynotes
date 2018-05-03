
#include "svd.h"
#include "osj_svd_mt.h"

#include <chrono>
#include <iostream>
using std::cout;
using std::cerr;
static std::stringstream nullout;

void test1(uint32_t rows, uint32_t cols)
{
	MatrixXd G(rows, cols);
	//G.setIdentity();
	G.setRandom();

	//G.row(1) = G.row(0) * G(0, 0);
	//G.row(3) = G.row(2) * G(0, 1);

	{
		//IF_SVD *pSVD = new OSJ_SVD(G);
		IF_SVD *pSVD = new OSJ_SVD_MT(G, 8);

		pSVD->decomp();
		pSVD->test(G, cout);

		delete pSVD;
	}
}

void test2(uint32_t num_max, uint32_t sz_max, uint32_t r_max, bool doTest)
{
	VectorXd rc(2);

	cout << "num, rows, cols, period, diff" << endl;
	for (uint32_t i = 0; i < num_max; i++) {
		cout << i << ", ";

		rc.setRandom();
		uint32_t sz = (uint32_t)((rc(0) + 1.0) * 0.5 * sz_max) + 1;
		uint32_t r = (uint32_t)((rc(1) + 1.0) * 0.5 * r_max) + 1;
		uint32_t c = (sz / r) + 1;
		cout << r << ", " << c << ", ";

		MatrixXd G(r, c);
		//G.setIdentity();
		G.setRandom();

		{
			//IF_SVD *pSVD = new OSJ_SVD(G);
			IF_SVD *pSVD = new OSJ_SVD_MT(G, 8);

			auto start = std::chrono::system_clock::now();
			pSVD->decomp();
			auto end = std::chrono::system_clock::now();
			auto period = std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count();
			cout << period << ", ";

			if (doTest) {
				double diff = pSVD->test(G, nullout);
				cout << diff;
			}
			cout << endl;

			delete pSVD;
		}
	}
}

int main(int argc, char ** argv)
{
	//test1(3, 3);
	//test1(10, 10);
	//test2(10, 3000, 50, true);
	test2(100, 300000, 500, false);

	cerr << "Hit Any Key" << endl;
	getchar();
}
