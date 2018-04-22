
//#define NDEBUG

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <assert.h>

typedef unsigned char UCHAR;
typedef float SCALAR;

#define TAU (1.0/256.0)
#define EPSILON (1.0/256.0)

#define REG 0

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
        if(w < 0) w = 1 - w;
        if(h < 0) h = 1 - h;
        if(w > width - 1) w = (width - 1) - (w - width);
        if(h > height - 1) h = (height - 1) - (h - height);
        
        assert(w >= 0);
        assert(h >= 0);
        assert(w < width);
        assert(h < height);
        return mem[h][w];
    }
    
	SCALAR lambda(int w, int h, UField *u0) {
        #if 1
    	if(abs_grad(w, h) < EPSILON) return 0;
        #endif
		
    	SCALAR r = abs_grad(w, h) - (
    		dx(w, h) * u0->dx(w, h) + dy(w, h) * u0->dy(w, h)
    	) / abs_grad(w, h);
		
        assert(finite(r));
        return r;
	}
	
    SCALAR dx(int w, int h)
    {
        SCALAR r = (a(w + 1, h) - a(w - 1, h)) / 2.0;
        
        assert(finite(r));
        return r;
    }
    SCALAR dy(int w, int h)
    {
        SCALAR r = (a(w, h + 1) - a(w, h - 1)) / 2.0;
        
        assert(finite(r));
        return r;
    }
	
    SCALAR dxx(int w, int h)
    {
        SCALAR r = a(w + 1, h) + a(w - 1, h) - 2.0 * a(w, h);
        
        assert(finite(r));
        return r;
    }
    
    SCALAR dyy(int w, int h)
    {
        SCALAR r = a(w, h + 1) + a(w, h - 1) - 2.0 * a(w, h);
        
        assert(finite(r));
        return r;
    }
	
    SCALAR dxy(int w, int h)
    {
        SCALAR r = (a(w + 1, h + 1) + a(w - 1, h - 1) - a(w - 1, h + 1) - a(w + 1, h - 1)) / 4.0;
        
        assert(finite(r));
        return r;
    }
	
    SCALAR abs_grad(int w, int h)
    {
        SCALAR r = sqrt(REG + dx(w, h) * dx(w, h) + dy(w, h) * dy(w, h));
        
        assert(finite(r));
        return r;
    }
	
    SCALAR div_grad_norm(int w, int h)
    {
        #if 1
    	if(abs_grad(w, h) < EPSILON) return 0;
        #endif
        
        SCALAR r = (
            dxx(w, h) * dy(w, h) * dy(w, h)
            + dyy(w, h) * dx(w, h) * dx(w, h)
            - 2.0 * dx(w, h) * dy(w, h) * dxy(w, h)
        );
        
        r = r / (abs_grad(w, h) * abs_grad(w, h) * abs_grad(w, h));
        
        assert(finite(r));
        return r;
    }
    
};

void tv_denoise(UField *u, UField *u0, int width, int height, SCALAR sigma, int maxi)
{
    UField *nu = new UField(width, height);
    SCALAR lambda = 0;
    
    for(int h = 0; h < height; h++) {
        for(int w = 0; w < width; w++) {
            u->a(w, h) = u0->a(w, h);
        }
    }
    
    for(int i = 0; i != maxi; i++) {
    	SCALAR maxstep;
    	
        for(int c = 0; c < 2; c++) {
            
            lambda = 0.0;
            
            for(int h = 0; h < height; h++) {
                for(int w = 0; w < width; w++) {
                    lambda += u->lambda(w, h, u0);
                }
            }
            
            lambda /= -2.0 * sigma * sigma;
            
        	maxstep = 0.0;
        	
            for(int h = 0; h < height; h++) {
                for(int w = 0; w < width; w++) {
                    SCALAR step = u->div_grad_norm(w, h) - lambda * (u->a(w, h) - u0->a(w, h));
                	
                	step *= TAU;
                	
                	if(maxstep < fabs(step)) maxstep = fabs(step);
                	
                    nu->a(w, h) = u->a(w, h) + step;
                }
            }
            
            UField *swap = nu;
            nu = u;
            u = swap;
            
        	printf("%d %f %f\n", i, lambda, maxstep);
        }
    	
    	//if(maxstep < EPSILON) break;
    }
    
    delete nu;
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
    } else {
        fseek(fin, -1, SEEK_CUR);
    }
    
    fscanf(fin, "%d %d\n", x, y);
    fscanf(fin, "%d\n", &tmp);
    
    //printf("%d %d\n", *x, *y);
    
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
            R->a(w, h) = ( 0.29891 * buf[0] + 0.58661 * buf[1] + 0.11448 * buf[2]) / 128.0 - 1.0;
            G->a(w, h) = (-0.16874 * buf[0] - 0.33126 * buf[1] + 0.50000 * buf[2]) / 128.0;
            B->a(w, h) = ( 0.50000 * buf[0] - 0.41869 * buf[1] - 0.08131 * buf[2]) / 128.0;
#else
            R->a(w, h) = (SCALAR)buf[0] / 256.0;
            G->a(w, h) = (SCALAR)buf[1] / 256.0;
            B->a(w, h) = (SCALAR)buf[2] / 256.0;
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
    //fprintf(stderr, "sigma: %f, %f, %f\n", sqrt(vr - mr * mr), sqrt(vg - mg * mg), sqrt(vb - mb * mb));
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
            SCALAR r = (1 + R->a(w, h)                        + 1.40200 * B->a(w, h)) * 128.0;
            SCALAR g = (1 + R->a(w, h) - 0.34414 * G->a(w, h) - 0.71414 * B->a(w, h)) * 128.0;
            SCALAR b = (1 + R->a(w, h) + 1.77200 * G->a(w, h)                       ) * 128.0;
#else
            SCALAR r = R->a(w, h) * 256.0;
            SCALAR g = G->a(w, h) * 256.0;
            SCALAR b = B->a(w, h) * 256.0;
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
    if(argc - 1 != 4) {
        fprintf(stderr, "Usage: %s <input ppm> <output ppm> <sigma> <max iteration>\n", argv[0]);
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
    
    int width, height;
    
    if(read_ppm_header(fin, &width, &height)) {
        fprintf(stderr, "Header error: %s\n", argv[1]);
        fclose(fin);
        fclose(fout);
        exit(1);
    }
    
    SCALAR sigma = atof(argv[3]);
    int maxi = atoi(argv[4]);
    
    UField *uR = new UField(width, height);
    UField *uG = new UField(width, height);
    UField *uB = new UField(width, height);
    UField *gR = new UField(width, height);
    UField *gG = new UField(width, height);
    UField *gB = new UField(width, height);
    
    read_ppm(gR, gG, gB, fin, width, height);
    tv_denoise(uR, gR, width, height, sigma, maxi);
    tv_denoise(uG, gG, width, height, sigma, maxi);
    tv_denoise(uB, gB, width, height, sigma, maxi);
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
