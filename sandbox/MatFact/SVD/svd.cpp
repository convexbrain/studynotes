
#include "svd.h"
#include "osj_svd.h"
#include "osj_svd_mt.h"

unique_ptr<SVD_IF> SVD_Factory::create_OSJ_SVD(uint32_t rows, uint32_t cols) {
	return unique_ptr<SVD_IF>(new OSJ_SVD(rows, cols));
}

unique_ptr<SVD_IF> SVD_Factory::create_OSJ_SVD_MT(uint32_t rows, uint32_t cols, uint32_t th_num) {
	return unique_ptr<SVD_IF>(new OSJ_SVD_MT(rows, cols, th_num));
}
