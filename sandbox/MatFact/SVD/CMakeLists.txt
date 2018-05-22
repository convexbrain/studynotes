cmake_minimum_required(VERSION 2.8)

project(SVD CXX)

include_directories(
    ../../../submodules/eigen-git-mirror/Eigen
)

add_executable(SVD
	main.cpp
    svd.cpp
    osj_svd.cpp
    osj_svd_mt.cpp
)

if(MSVC)
else()
    add_definitions(-std=c++11 -Wall -msse2 -O2)
    target_link_libraries(SVD
        pthread
    )
endif()
