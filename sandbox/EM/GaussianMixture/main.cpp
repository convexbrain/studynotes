
#include "emgm.h"

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv)
{
	if(argc - 1 != 3) {
		fprintf(stderr, "Usage: %s <input csv> <output data> <max # of iterations>\n", argv[0]);
		exit(1);
	}
	
	FILE *fp_i = fopen(argv[1], "rt");
	
	int D, N;
	fscanf(fp_i, "%d,%d,", &D, &N);
	fprintf(stderr, "%d %d\n", D, N);
	
	EMGM emgm(3, D, N);
	
	SCALAR *x = new SCALAR[D];
	for(int in = 0; in < N; in++) {
		for(int id = 0; id < D; id++) {
			fscanf(fp_i, "%lf,", &x[id]);
		}
		emgm.setData(in, x);
	}
	
	fclose(fp_i);
	
	//
	
	int itnum = emgm.learn(atoi(argv[3]));
	fprintf(stderr, "%d\n", itnum);
	
	//
	
	FILE *fp_o = fopen(argv[2], "wt");
	
	emgm.dprint(fp_o);
	
	fclose(fp_o);
	
	return 0;
}
