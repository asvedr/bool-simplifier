#include "expr.h"

// (tbl,iter_count,max_depth) -> Option<Expr>
extern "C" const char* find_analog(Table*,int,int);
