#ifndef _SVM_H_
#define _SVM_H_

#include <iostream>
using namespace std;

typedef double SCALAR;

struct SVector {
	SCALAR *x;
	int     y;
	SCALAR  a;
	
	SVector() {
		x = NULL;
	}
	void alloc(int d) {
		x = new SCALAR[d];
	}
	~SVector() {
		if(x != NULL) delete[] x;
	}
};

class SVM {
	
protected:
	
	SCALAR kernel(SCALAR *x0, SCALAR *x1) const;
	
private:
	
	static const SCALAR EPSILON = 1.0/65536.0/65536.0;
	
	static inline SCALAR max(SCALAR x, SCALAR y) {
		return (x > y)? x: y;
	}
	static inline SCALAR min(SCALAR x, SCALAR y) {
		return (x > y)? y: x;
	}
	static inline SCALAR abs(SCALAR x) {
		return (x < 0)? -x: x;
	}
	
	SVector *sv;
	SCALAR   m_a;
	SCALAR   b;
	int      dim;
	int      num;
	
	SVector *input;
	int      cnt_x;
	
	SCALAR w(SCALAR *x) const;
	void   calc_bias(void);
	bool   smo_update(SVector *v0, SVector *v1);
	bool   smo_iteration(bool scanall);
	
public:
	
	SVM(int d, int n, SCALAR c): b(0.0), dim(d), num(n), cnt_x(0) {
		m_a = c;
		sv = new SVector[num];
		for(SVector *ptr_v = sv; ptr_v < sv + num; ptr_v++) {
			ptr_v->alloc(dim);
		}
		input = sv;
	}
	~SVM() {
		if(sv != NULL) delete[] sv;
	}
	SCALAR function(SCALAR *x) const {
		return w(x) + b;
	}
	int classify(SCALAR *x) const {
		if(input < sv + num) {
			cerr << "Error: lack of input data!" << endl;
			return 0;
		}
		
		if(function(x) > 0.0) return  1;
		else                  return -1;
	}
	
	void operator<<(const SCALAR &d);
	void dprint(ostream &os) const;
	int  learn(int n_iterate);
};

#endif // end of #ifndef _SVM_H_
