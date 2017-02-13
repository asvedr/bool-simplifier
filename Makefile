all:
	g++ -shared -O2 expr.cpp -o libexpr.so
