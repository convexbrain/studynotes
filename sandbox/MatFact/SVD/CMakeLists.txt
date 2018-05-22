cmake_minimum_required(VERSION 2.8)

project(SVD CXX)

if(MSVC)
    add_definitions()
else()
    add_definitions(-std=c++11 -Wall -msse2 -O2)
endif()

include_directories(
    ../../../submodules/eigen-git-mirror/Eigen
)

add_executable(SVD
	main.cpp
    svd.cpp
    osj_svd.cpp
    osj_svd_mt.cpp
)
