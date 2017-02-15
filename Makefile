all:
	g++ -std=c++11 -shared -O2 expr.cpp -o libexpr.so
