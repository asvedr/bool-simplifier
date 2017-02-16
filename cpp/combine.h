#include "expr.h"

// (tbl,iter_count,max_depth) -> Option<Expr>
extern "C" Expr* find_analog(Table*,int,int);
