
#include "svm.h"

#include <algorithm>
#include <vector>
#include <math.h>

static SCALAR product(SCALAR *x0, SCALAR *x1, int dim)
{
	SCALAR r = 0.0;
	
	for(int i = 0; i < dim; i++) {
		r += *x0++ * *x1++;
	}
	
	return r;
}

static SCALAR polynomial(SCALAR *x0, SCALAR *x1, int dim, SCALAR c, int d)
{
	SCALAR r = 0.0;
	
	for(int i = 0; i < dim; i++) {
		r += *x0++ * *x1++;
	}
	
	SCALAR rr = 1.0;
	
	for(int i = 0; i < d; i++) {
		rr *= (r + c);
	}
	
	return rr;
}

static SCALAR gaussian(SCALAR *x0, SCALAR *x1, int dim, SCALAR sigma2)
{
	SCALAR r = 0.0;
	
	for(int i = 0; i < dim; i++) {
		r += (*x0 - *x1) * (*x0 - *x1);
		x0++;
		x1++;
	}
	
	return exp(-r / sigma2);
}

SCALAR SVM::kernel(SCALAR *x0, SCALAR *x1) const
{
	//return product(x0, x1, dim);
	//return polynomial(x0, x1, dim, 1.0, 2);
	return gaussian(x0, x1, dim, 1.0/8);
}

SCALAR SVM::w(SCALAR *x) const
{
	SCALAR wx = 0.0;
	
	for(SVector *ptr_v = sv; ptr_v < sv + num; ptr_v++) {
		if(ptr_v->a > 0.0) {
			wx += ptr_v->a * ptr_v->y * kernel(ptr_v->x, x);
		}
	}
	
	return wx;
}

void SVM::calc_bias(void)
{
	SCALAR min_wx_p = 0.0;
	SCALAR max_wx_n = 0.0;
	bool first_p = true;
	bool first_n = true;
	
	for(SVector *ptr_v = sv; ptr_v < sv + num; ptr_v++) {
		if( (0.0 < ptr_v->a) && (ptr_v->a < m_a) ) {
			if(ptr_v->y > 0) {
				if(first_p) {
					first_p = false;
					min_wx_p = w(ptr_v->x);
				} else {
					min_wx_p = min(min_wx_p, w(ptr_v->x));
				}
			} else {
				if(first_n) {
					first_n = false;
					max_wx_n = w(ptr_v->x);
				} else {
					max_wx_n = max(max_wx_n, w(ptr_v->x));
				}
			}
		}
	}
	
	b = - ( max_wx_n + min_wx_p ) / 2.0;
}

bool SVM::smo_update(SVector *v0, SVector *v1)
{
	if((v0 == v1) || (v0 == NULL) || (v1 == NULL)) return false;
	
	SCALAR e0 = w(v0->x) /* + b */ - v0->y;
	SCALAR e1 = w(v1->x) /* + b */ - v1->y;
	
	SCALAR k = kernel(v0->x, v0->x) + kernel(v1->x, v1->x) - 2.0 * kernel(v0->x, v1->x);
	
	SCALAR a0 = v0->a + v0->y * (e1 - e0) / k;
	
	SCALAR u;
	SCALAR v;
	if(v0->y != v1->y) {
		u = max(0.0, v0->a - v1->a);
		v = min(m_a, v0->a - v1->a + m_a);
	} else {
		u = max(0.0, v0->a + v1->a - m_a);
		v = min(m_a, v0->a + v1->a);
	}
	a0 = max(u, a0);
	a0 = min(v, a0);
	
	if(a0 < 0.0 + EPSILON) a0 = 0.0;
	if(a0 > m_a - EPSILON) a0 = m_a;
	
	SCALAR a1 = v1->a + v0->y * v1->y * (v0->a - a0);
	
	if(a1 < 0.0 + EPSILON) a1 = 0.0;
	if(a1 > m_a - EPSILON) a1 = m_a;
	
	if( isnan(a0) || isnan(a1) ) return false;
	if( (abs(v0->a - a0) < EPSILON) && (abs(v1->a - a1) < EPSILON) ) return false;
	
	//cerr << v0 << " " << v1 << " ";
	//cerr << abs(v0->a - a0) << " " << abs(v1->a - a1) << " ";
	//cerr << a0 << " " << a1 << endl;
	
	v0->a = a0;
	v1->a = a1;
	
	return true;
}

bool SVM::smo_iteration(bool scanall)
{
	vector<SVector*> cand0;
	
	for(SVector *ptr_v = sv; ptr_v < sv + num; ptr_v++) {
		SCALAR c = 1.0 - ptr_v->y * (w(ptr_v->x) + b);
		
		if(ptr_v->a < 0.0 + EPSILON) {
			if(scanall && (c > 0.0) ) {
				cand0.push_back(ptr_v);
			}
		} else if(ptr_v->a > m_a - EPSILON) {
			if(scanall && (c < 0.0) ) {
				cand0.push_back(ptr_v);
			}
		} else {
			if(abs(c) > EPSILON) {
				cand0.push_back(ptr_v);
			}
		}
	}
	
	random_shuffle ( cand0.begin(), cand0.end() );
	
	while(!cand0.empty()) {
		vector<SVector*> cand1_2nd, cand1_3rd;
		
		SVector *v0 = cand0.back();
		SVector *v1 = NULL;
		
		SCALAR maxe = 0.0;
		SCALAR e0 = w(v0->x) /* + b */ - v0->y;
		
		for(SVector *ptr_v = sv; ptr_v < sv + num; ptr_v++) {
			if( (0.0 < ptr_v->a) && (ptr_v->a < m_a) ) {
				SCALAR e1 = w(ptr_v->x) /* + b */ - ptr_v->y;
				if(abs(e0 - e1) > maxe) {
					if(v1 != NULL) cand1_2nd.push_back(v1);
					v1 = ptr_v;
					maxe = abs(e0 - e1);
				} else {
					cand1_2nd.push_back(ptr_v);
				}
			} else {
				cand1_3rd.push_back(ptr_v);
			}
		}
		
		if(smo_update(v0, v1)) return true;
		
		random_shuffle ( cand1_2nd.begin(), cand1_2nd.end() );
		
		while(!cand1_2nd.empty()) {
			v1 = cand1_2nd.back();
			if(smo_update(v0, v1)) return true;
			cand1_2nd.pop_back();
		}
		
		random_shuffle ( cand1_3rd.begin(), cand1_3rd.end() );
		
		while(!cand1_3rd.empty()) {
			v1 = cand1_3rd.back();
			if(smo_update(v0, v1)) return true;
			cand1_3rd.pop_back();
		}
		
		cand0.pop_back();
	}
	
	return false;
}

void SVM::operator<<(const SCALAR &d)
{
	//cerr << d << endl;
	
	if(input < sv + num) {
		if(cnt_x < dim) {
			input->x[cnt_x] = d;
			cnt_x++;
		} else {
			input->y = (d > 0.0)? 1: -1;
			input->a = 0.0;
			input++;
			cnt_x = 0;
		}
	} else {
		cerr << "Error: input data overflow!" << endl;
	}
}

void SVM::dprint(ostream &ofs) const
{
	if(input < sv + num) {
		cerr << "Error: lack of input data!" << endl;
		return;
	}
	
	/*
	for(SVector *ptr_v = sv; ptr_v < sv + num; ptr_v++) {
		SCALAR *x = ptr_v->x;
		for(int i = 0; i < dim; i++) ofs << (*x++) << " ";
		ofs << " " << (ptr_v->a) << " " << (w(ptr_v->x) + b) << endl;
	}
	ofs << endl << endl;
	return;
	*/
	
	ofs << "# 1 : lower boundaries, positive" << endl;
	for(SVector *ptr_v = sv; ptr_v < sv + num; ptr_v++) {
		if((ptr_v->y > 0) && (ptr_v->a < 0.0 + EPSILON)) {
			SCALAR *x = ptr_v->x;
			ofs << "1  ";
			for(int i = 0; i < dim; i++) ofs << (*x++) << " ";
			ofs << " " << (ptr_v->a) << " " << (w(ptr_v->x) + b) << endl;
		}
	}
	
	ofs << "# 2 : lower boundaries, negative" << endl;
	for(SVector *ptr_v = sv; ptr_v < sv + num; ptr_v++) {
		if((ptr_v->y < 0) && (ptr_v->a < 0.0 + EPSILON)) {
			SCALAR *x = ptr_v->x;
			ofs << "2  ";
			for(int i = 0; i < dim; i++) ofs << (*x++) << " ";
			ofs << " " << (ptr_v->a) << " " << (w(ptr_v->x) + b) << endl;
		}
	}
	
	ofs << "# 3 : upper boundaries, positive" << endl;
	for(SVector *ptr_v = sv; ptr_v < sv + num; ptr_v++) {
		if((ptr_v->y > 0) && (ptr_v->a > m_a - EPSILON)) {
			SCALAR *x = ptr_v->x;
			ofs << "3  ";
			for(int i = 0; i < dim; i++) ofs << (*x++) << " ";
			ofs << " " << (ptr_v->a) << " " << (w(ptr_v->x) + b) << endl;
		}
	}
	
	ofs << "# 4 : upper boundaries, negative" << endl;
	for(SVector *ptr_v = sv; ptr_v < sv + num; ptr_v++) {
		if((ptr_v->y < 0) && (ptr_v->a > m_a - EPSILON)) {
			SCALAR *x = ptr_v->x;
			ofs << "4  ";
			for(int i = 0; i < dim; i++) ofs << (*x++) << " ";
			ofs << " " << (ptr_v->a) << " " << (w(ptr_v->x) + b) << endl;
		}
	}
	
	ofs << "# 5 : support vectors, positive" << endl;
	for(SVector *ptr_v = sv; ptr_v < sv + num; ptr_v++) {
		if((ptr_v->y > 0) && (0.0 < ptr_v->a) && (ptr_v->a < m_a)) {
			SCALAR *x = ptr_v->x;
			ofs << "5  ";
			for(int i = 0; i < dim; i++) ofs << (*x++) << " ";
			ofs << " " << (ptr_v->a) << " " << (w(ptr_v->x) + b) << endl;
		}
	}
	
	ofs << "# 6 : support vectors, negaitive" << endl;
	for(SVector *ptr_v = sv; ptr_v < sv + num; ptr_v++) {
		if((ptr_v->y < 0) && (0.0 < ptr_v->a) && (ptr_v->a < m_a)) {
			SCALAR *x = ptr_v->x;
			ofs << "6  ";
			for(int i = 0; i < dim; i++) ofs << (*x++) << " ";
			ofs << " " << (ptr_v->a) << " " << (w(ptr_v->x) + b) << endl;
		}
	}
	
	ofs << endl << endl;
}

int SVM::learn(int n_iterate)
{
	if(input < sv + num) {
		cerr << "Error: lack of input data!" << endl;
	}
	
	bool scanall = true;
	int i;
	
	for(i = 0; i != n_iterate; i++) {
		
		bool changed = smo_iteration(scanall);
		
		calc_bias();
		
		if(scanall) {
			if(!changed) break;
			scanall = false;
		} else {
			if(!changed) scanall = true;
		}
	}
	
	return i;
}

