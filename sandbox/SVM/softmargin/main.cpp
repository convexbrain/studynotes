
#include "svm.h"

#include <fstream>
#include <stdlib.h>

int main(int argc, char **argv)
{
	if(argc - 1 != 4) {
		cerr << "Usage: " << argv[0] << " <input filename> <output filename> <soft margin C> <max # of iterations>" << endl;
		exit(1);
	}
	
	ifstream rdata(argv[1]);
	ofstream wdata(argv[2]);
	SCALAR c = atoi(argv[3]);
	int max_it = atoi(argv[4]);
	
	//
	
	char comma;
	int tdim, tnum;
	rdata >> tdim >> comma;
	rdata >> tnum;
	rdata.ignore(1024, '\n');
	
	SVM svm(tdim, tnum, c);
	
	for(int i = 0; i < tnum; i++) {
		SCALAR x;
		for(int j = 0; j < tdim; j++) {
			rdata >> x >> comma;
			svm << x;
		}
		
		int y;
		rdata >> y;
		svm << y;
	}
	
	//
	
	int it = svm.learn(max_it);
	cerr << it << " iterations" << endl;
	
	//
	
	svm.dprint(wdata);
	
	//
	
	SCALAR p[2];
	
	for(SCALAR y = 0.0; y < 1.0; y += 1.0/64) {
		for(SCALAR x = 0.0; x < 1.0; x += 1.0/64) {
			p[0] = x;
			p[1] = y;
			if(svm.classify(p) > 0) wdata << x << " " << y << "  " << svm.function(p) << endl;
		}
	}
	wdata << endl << endl;
	
	for(SCALAR y = 0.0; y < 1.0; y += 1.0/64) {
		for(SCALAR x = 0.0; x < 1.0; x += 1.0/64) {
			p[0] = x;
			p[1] = y;
			if(svm.classify(p) < 0) wdata << x << " " << y << "  " << svm.function(p) << endl;
		}
	}
	wdata << endl << endl;
	
	return 0;
}
