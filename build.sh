#!/bin/bash
export COMPILE="g++ -std=c++11 -O2"
export    TO_O="-c"
export  TO_LIB="-shared"
export LIBNAME="combinator.so"
cd cpp
$COMPILE $TO_O expr.cpp &&
$COMPILE $TO_LIB combine.cpp expr.o -o $LIBNAME &&
mv $LIBNAME ../
cd ..
