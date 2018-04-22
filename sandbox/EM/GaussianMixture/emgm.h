#ifndef __EMGM_H__
#define __EMGM_H__

#include <stdio.h>

typedef double SCALAR;

#define EPSILON (1.0/65536.0)

struct Data;
struct Theta;

class EMGM {
	int K;
	int D;
	int N;
	Data **pData;
	Theta **pTheta;
	SCALAR sum1E;
	SCALAR *sumXE;
	SCALAR *sumXXE;
	
	SCALAR matrix_det_inv(SCALAR *m);
	SCALAR Estep(void);
	void Mstep(void);
	
public:
	EMGM(int k, int d, int n);
	~EMGM();
	void setData(int in, SCALAR *x);
	int learn(int itMax);
	void dprint(FILE *fp_o);
	int classify(SCALAR *x);
};

#endif // end of #ifndef __EMGM_H__
