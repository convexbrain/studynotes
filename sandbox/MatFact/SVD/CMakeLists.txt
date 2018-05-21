cmake_minimum_required(VERSION 2.8)

project(SVD CXX)

include_directories("../../../submodules/eigen-git-mirror/Eigen")

add_executable(SVD
	main.cpp
    svd.cpp
    osj_svd.cpp
    osj_svd_mt.cpp
)
