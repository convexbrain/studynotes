
SOURCES = main.cpp svm.cpp
HEADERS = svm.h
BINARY = main.out

CXX = g++
CFLAGS = -O3 -Wall
LFLAGS = #-lgsl -lgslcblas


all: $(BINARY)

$(BINARY): $(SOURCES) $(HEADERS)
	$(CXX) $(CFLAGS) $(SOURCES) -o $@ $(LFLAGS)

clean:
	rm -f $(BINARY)

