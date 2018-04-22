
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef unsigned char UCHAR;
typedef float SCALAR;

#define TAU (1.0/8.0)
#define EPSILON (1.0/256.0)

//#define YUV

class UField {
	
private:
	SCALAR **mem;
	int width;
	int height;
	
public:
	UField(int w, int h) : width(w), height(h)
	{
		mem = new (SCALAR*[height]);
		for(int y = 0; y < height; y++) {
			mem[y] = new SCALAR[width];
		}
	}
	
	~UField()
	{
		for(int y = 0; y < height; y++) {
			delete[] mem[y];
		}
		delete[] mem;
	}
	
	SCALAR &a(int w, int h)
	{
		return mem[h][w];
	}
	
};

class PField {
	
private:
	SCALAR ***mem;
	int width;
	int height;
	
public:
	PField(int w, int h) : width(w), height(h)
	{
		mem = new (SCALAR**[height]);
		for(int y = 0; y < height; y++) {
			mem[y] = new (SCALAR*[width]);
			for(int x = 0; x < width; x++) {
				mem[y][x] = new SCALAR[2];
				mem[y][x][0] = mem[y][x][1] = 0;
			}
		}
	}
	
	~PField()
	{
		for(int y = 0; y < height; y++) {
			for(int x = 0; x < width; x++) {
				delete[] mem[y][x];
			}
			delete[] mem[y];
		}
		delete[] mem;
	}
	
	SCALAR &a(int w, int h, int d)
	{
		return mem[h][w][d];
	}
	
	SCALAR div(int w, int h)
	{
		SCALAR r = 0;
		
		if(w < width - 1)  r += a(w    , h    , 0);
		if(w > 0)          r -= a(w - 1, h    , 0);
		if(h < height - 1) r += a(w    , h    , 1);
		if(h > 0)          r -= a(w    , h - 1, 1);
		
		return r;
	}
	
	void nabla(SCALAR *n, UField *g, SCALAR l, int w, int h)
	{
		if(w < width - 1) {
			n[0] = (div(w + 1, h) - g->a(w + 1, h) / l) - (div(w, h) - g->a(w, h) / l);
		} else {
			n[0] = 0;
		}
		
		if(h < height - 1) {
			n[1] = (div(w, h + 1) - g->a(w, h + 1) / l) - (div(w, h) - g->a(w, h) / l);
		} else {
			n[1] = 0;
		}
	}
	
};

void tv_min(UField *u, UField *g, SCALAR param, bool denoise, int width, int height)
{
	SCALAR lambda, sigma;
	
	if(denoise) {
		sigma = param;
		lambda = 1.0;
	} else {
		lambda = param;
	}
	
	PField *p  = new PField(width, height);
	PField *np = new PField(width, height);
	
	SCALAR maxsad;
	
	for(;;) {
		maxsad = 0;
		
		for(int h = 0; h < height; h++) {
			for(int w = 0; w < width; w++) {
				SCALAR n[2];
				
				p->nabla(n, g, lambda, w, h);
				
				SCALAR r = 1 + TAU * sqrt(n[0] * n[0] + n[1] * n[1]);
				np->a(w, h, 0) = (p->a(w, h, 0) + TAU * n[0]) / r;
				np->a(w, h, 1) = (p->a(w, h, 1) + TAU * n[1]) / r;
				
				SCALAR sad;
				sad = fabs(np->a(w, h, 0) - p->a(w, h, 0));
				if(maxsad < sad) maxsad = sad;
				sad = fabs(np->a(w, h, 1) - p->a(w, h, 1));
				if(maxsad < sad) maxsad = sad;
			}
		}
		
		PField *swap = p;
		p = np;
		np = swap;
		
		fprintf(stderr, "Max SAD: %f, Lambda: %f\n", maxsad, lambda);
		if(maxsad <= EPSILON) break;
		
		if(denoise) {
			SCALAR r = 0;
			for(int h = 0; h < height; h++) {
				for(int w = 0; w < width; w++) {
					r += p->div(w, h) * p->div(w, h);
				}
			}
			lambda = sqrt(width * height) * sigma / sqrt(r);
			if(lambda > 1.0) lambda = 1.0;
			if(lambda < EPSILON) lambda = EPSILON;
		}
		
	}
	
	for(int h = 0; h < height; h++) {
		for(int w = 0; w < width; w++) {
			u->a(w, h) = g->a(w, h) - lambda * p->div(w, h);
		}
	}
	
	delete np;
	delete p;
}

int read_ppm_header(FILE *fin, int *x, int *y)
{
	static char buf[256];
	int tmp;
	
	fread(buf, sizeof(char), 3, fin);
	if(strncmp(buf, "P6\n", 3) != 0) {
		return 1;
	}
	
	if(fgetc(fin) == '#') {
		while(fgetc(fin) != '\n');
	}
	
	fscanf(fin, "%d %d\n", x, y);
	fscanf(fin, "%d\n", &tmp);
	
	if(tmp != 255) {
		return 1;
	}
	
	return 0;
}

void read_ppm(UField *R, UField *G, UField *B, FILE *fin, int width, int height)
{
	SCALAR mr = 0, vr = 0;
	SCALAR mg = 0, vg = 0;
	SCALAR mb = 0, vb = 0;
	static UCHAR buf[3];
	
	for(int h = 0; h < height; h++) {
		for(int w = 0; w < width; w++) {
			fread(buf, sizeof(UCHAR), 3, fin);
#ifdef YUV
			R->a(w, h) = ( 0.29891 * buf[0] + 0.58661 * buf[1] + 0.11448 * buf[2]) / 128 - 1;
			G->a(w, h) = (-0.16874 * buf[0] - 0.33126 * buf[1] + 0.50000 * buf[2]) / 128;
			B->a(w, h) = ( 0.50000 * buf[0] - 0.41869 * buf[1] - 0.08131 * buf[2]) / 128;
#else
			R->a(w, h) = ((SCALAR)buf[0] - 128) / 128;
			G->a(w, h) = ((SCALAR)buf[1] - 128) / 128;
			B->a(w, h) = ((SCALAR)buf[2] - 128) / 128;
#endif
			mr += R->a(w, h);
			mg += G->a(w, h);
			mb += B->a(w, h);
			vr += R->a(w, h) * R->a(w, h);
			vg += G->a(w, h) * G->a(w, h);
			vb += B->a(w, h) * B->a(w, h);
		}
	}
	
	mr /= width * height;
	mg /= width * height;
	mb /= width * height;
	vr /= width * height;
	vg /= width * height;
	vb /= width * height;
	fprintf(stderr, "sigma: %f, %f, %f\n", sqrt(vr - mr * mr), sqrt(vg - mg * mg), sqrt(vb - mb * mb));
}

void write_ppm(UField *R, UField *G, UField *B, FILE *fout, int width, int height)
{
	fprintf(fout, "P6\n");
	fprintf(fout, "%d %d\n", width, height);
	fprintf(fout, "255\n");
	
	static UCHAR buf[3];
	
	for(int h = 0; h < height; h++) {
		for(int w = 0; w < width; w++) {
#ifdef YUV
			SCALAR r = (1 + R->a(w, h)                        + 1.40200 * B->a(w, h)) * 128;
			SCALAR g = (1 + R->a(w, h) - 0.34414 * G->a(w, h) - 0.71414 * B->a(w, h)) * 128;
			SCALAR b = (1 + R->a(w, h) + 1.77200 * G->a(w, h)                       ) * 128;
#else
			SCALAR r = R->a(w, h) * 128 + 128;
			SCALAR g = G->a(w, h) * 128 + 128;
			SCALAR b = B->a(w, h) * 128 + 128;
#endif
			buf[0] = (UCHAR)( (r < 0)? 0: ((r > 255)? 255: r) );
			buf[1] = (UCHAR)( (g < 0)? 0: ((g > 255)? 255: g) );
			buf[2] = (UCHAR)( (b < 0)? 0: ((b > 255)? 255: b) );
			
			fwrite(buf, sizeof(UCHAR), 3, fout);
		}
	}
}

int main(int argc, char **argv)
{
	if(argc - 1 != 3) {
		fprintf(stderr, "Usage: %s <input ppm> <output ppm> <param>\n", argv[0]);
		exit(1);
	}
	
	FILE *fin, *fout;
	
	if((fin = fopen(argv[1], "rb")) == NULL) {
		fprintf(stderr, "Cannot open: %s\n", argv[1]);
		exit(1);
	}
	
	if((fout = fopen(argv[2], "wb")) == NULL) {
		fprintf(stderr, "Cannot open: %s\n", argv[2]);
		fclose(fin);
		exit(1);
	}
	
	SCALAR param = atof(argv[3]);
	
	int width, height;
	
	if(read_ppm_header(fin, &width, &height)) {
		fprintf(stderr, "Header error: %s\n", argv[1]);
		fclose(fin);
		fclose(fout);
		exit(1);
	}
	
	UField *uR = new UField(width, height);
	UField *uG = new UField(width, height);
	UField *uB = new UField(width, height);
	UField *gR = new UField(width, height);
	UField *gG = new UField(width, height);
	UField *gB = new UField(width, height);
	
	read_ppm(gR, gG, gB, fin, width, height);
	tv_min(uR, gR, param, true/*denoise*/, width, height);
	tv_min(uG, gG, param, true/*denoise*/, width, height);
	tv_min(uB, gB, param, true/*denoise*/, width, height);
	write_ppm(uR, uG, uB, fout, width, height);
	
	delete uR;
	delete uG;
	delete uB;
	delete gR;
	delete gG;
	delete gB;
	
	fclose(fout);
	fclose(fin);
}
