
#include "emgm.h"

#include <assert.h>
#include <math.h>
#include <gsl/gsl_matrix.h>
#include <gsl/gsl_permutation.h>
#include <gsl/gsl_linalg.h>

typedef double SCALAR;

#define EPSILON (1.0/65536.0)

struct Data {
	int K;
	int D;
	SCALAR *x;
	SCALAR *E;
	
	Data(int k, int d): K(k), D(d) {
		x = new SCALAR[D];
		E = new SCALAR[K];
	}
	
	~Data() {
		delete[] x;
		delete[] E;
	}
};

struct Theta {
	int D;
	SCALAR Phi;
	SCALAR *Mu;
	SCALAR *invS;
	SCALAR detS;
	
	Theta(int d): D(d) {
		Mu = new SCALAR[D];
		invS = new SCALAR[D*D];
	}
	
	void init(int k, SCALAR *x) {
		Phi = 1.0 / (SCALAR)k;
		for(int id = 0; id < D; id++) {
			Mu[id] = x[id];
			for(int id2 = 0; id2 < D; id2++) {
				if(id == id2) {
					invS[id*D+id2] = 1.0;
				} else {
					invS[id*D+id2] = 0.0;
				}
			}
		}
		detS = 1.0;
	}
	
	SCALAR N(SCALAR *x) {
		SCALAR r = 0.0;
		
		for(int id0 = 0; id0 < D; id0++) {
			SCALAR r1 = 0.0;
			
			for(int id1 = 0; id1 < D; id1++) {
				r1 += invS[id1*D+id0] * (x[id1] - Mu[id1]);
			}
			
			r += r1 * (x[id0] - Mu[id0]);
		}
		
		return exp(-1.0/2.0 * r) / pow(2.0*M_PI, D) / sqrt(detS);
	}
	
	~Theta() {
		delete[] invS;
		delete[] Mu;
	}
};

EMGM::EMGM(int k, int d, int n): K(k), D(d), N(n) {
	assert(K > 0);
	assert(D > 0);
	assert(N > 0);
	assert(K < N);
	
	pData = new Data*[N];
	for(int in = 0; in < N; in++) {
		pData[in] = new Data(K, D);
	}
	
	pTheta = new Theta*[K];
	for(int ik = 0; ik < K; ik++) {
		pTheta[ik] = new Theta(D);
	}
	
	sumXE = new SCALAR[D];
	sumXXE = new SCALAR[D*D];
}
	
EMGM::~EMGM() {
	for(int in = 0; in < N; in++) {
		delete pData[in];
	}
	delete[] pData;
	
	for(int ik = 0; ik < K; ik++) {
		delete pTheta[ik];
	}
	delete[] pTheta;
	
	delete[] sumXE;
	delete[] sumXXE;
}

void EMGM::setData(int in, SCALAR *x) {
	for(int id = 0; id < D; id++) {
		pData[in]->x[id] = x[id];
	}
	if(in < K) {
		int ik = in;
		pTheta[ik]->init(K, x);
	}
}

SCALAR EMGM::Estep(void)
{
	SCALAR logp = 0.0;
	
	for(int in = 0; in < N; in++) {
		SCALAR sum = 0.0;
		
		for(int ik = 0; ik < K; ik++) {
			pData[in]->E[ik] = pTheta[ik]->Phi * pTheta[ik]->N(pData[in]->x);
			sum += pTheta[ik]->Phi * pTheta[ik]->N(pData[in]->x);
		}
		logp += log(sum);
		
		for(int ik = 0; ik < K; ik++) {
			pData[in]->E[ik] /= sum;
		}
	}
	
	return logp;
}

void EMGM::Mstep(void)
{
	for(int ik = 0; ik < K; ik++) {
		sum1E = 0.0;
		for(int id = 0; id < D; id++) {
			sumXE[id] = 0.0;
		}
		for(int idd = 0; idd < D*D; idd++) {
			sumXXE[idd] = 0.0;
		}
		
		for(int in = 0; in < N; in++) {
			sum1E += pData[in]->E[ik];
			for(int id = 0; id < D; id++) {
				sumXE[id] += pData[in]->x[id] * pData[in]->E[ik];
				for(int id2 = 0; id2 < D; id2++) {
					sumXXE[id*D+id2] += (pData[in]->x[id] - pTheta[ik]->Mu[id]) *
					                    (pData[in]->x[id2] - pTheta[ik]->Mu[id2]) *
					                    pData[in]->E[ik];
				}
			}
		}
		
		pTheta[ik]->Phi = sum1E / (SCALAR)N;
		for(int id = 0; id < D; id++) {
			pTheta[ik]->Mu[id] = sumXE[id] / sum1E;
		}
		pTheta[ik]->detS = matrix_det_inv(sumXXE) / sum1E;
		for(int idd = 0; idd < D*D; idd++) {
			pTheta[ik]->invS[idd] = sumXXE[idd] * sum1E;
		}
	}
}

int EMGM::learn(int itMax)
{
	SCALAR old_logp = 0.0;
	int it;
	
	for(it = 0; it != itMax; it++) {
		SCALAR new_logp = Estep();
		
		// Convergence check
		//fprintf(stderr, "%d %f\n", it, new_logp);
		if((new_logp > old_logp) && (new_logp - old_logp < EPSILON)) {
			break;
		}
		old_logp = new_logp;
		
		Mstep();
	}
	
	return it;
}

SCALAR EMGM::matrix_det_inv(SCALAR *m)
{
	gsl_matrix_view m_in = gsl_matrix_view_array(m, D, D);
	gsl_matrix *lu = gsl_matrix_alloc(D, D);
	gsl_permutation *p = gsl_permutation_alloc(D);
	int s;
	
	gsl_matrix_memcpy(lu, &m_in.matrix);
	gsl_linalg_LU_decomp(lu, p, &s);
	SCALAR det = gsl_linalg_LU_det(lu, s);
	gsl_linalg_LU_invert(lu, p, &m_in.matrix);
	gsl_permutation_free (p);
	gsl_matrix_free (lu);
	
	return det;
}

int EMGM::classify(SCALAR *x)
{
	SCALAR maxp = -1.0;
	int maxk = -1;
	
	for(int ik = 0; ik < K; ik++) {
		SCALAR p = pTheta[ik]->Phi * pTheta[ik]->N(x);
		if(p > maxp) {
			maxp = p;
			maxk = ik;
		}
	}
	
	return maxk;
}

void EMGM::dprint(FILE *fp_o)
{
	for(int ik = 0; ik < K; ik++) {
		for(int in = 0; in < N; in++) {
			if (classify(pData[in]->x) == ik) {
				for(int id = 0; id < D; id++) {
					fprintf(fp_o, "%f ", pData[in]->x[id]);
				}
				
				fprintf(fp_o, "%f ", pTheta[ik]->N(pData[in]->x));
				
				SCALAR p = 0.0;
				for(int ikk = 0; ikk < K; ikk++) {
					p += pTheta[ikk]->Phi * pTheta[ikk]->N(pData[in]->x);
				}
				p = pTheta[ik]->Phi * pTheta[ik]->N(pData[in]->x) / p;
				fprintf(fp_o, "%f ", p);
				
				fprintf(fp_o, "\n");
			}
		}
		
		fprintf(fp_o, "\n");
		fprintf(fp_o, "\n");
	}
	
	SCALAR x[2];
	int div = 32;
	for(int ix0 = 0; ix0 < div; ix0++) {
		x[0] = (SCALAR)ix0 / (SCALAR)div * (1.4 + 0.4) - 0.4;
		for(int ix1 = 0; ix1 < div; ix1++) {
			x[1] = (SCALAR)ix1 / (SCALAR)div * (1.4 + 0.4) - 0.4;
			
			for(int id = 0; id < 2; id++) {
				fprintf(fp_o, "%f ", x[id]);
			}
			
			int k = classify(x);
			fprintf(fp_o, "%f ", pTheta[k]->N(x));
			
			SCALAR p = 0.0;
			for(int ik = 0; ik < K; ik++) {
				p += pTheta[ik]->Phi * pTheta[ik]->N(x);
			}
			p = pTheta[k]->Phi * pTheta[k]->N(x) / p;
			fprintf(fp_o, "%f ", p);
			
			fprintf(fp_o, "\n");
		}
		
		fprintf(fp_o, "\n");
	}
}

