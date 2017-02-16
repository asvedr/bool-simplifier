#include <string>
#include <vector>
#include <sstream>
//#include <unordered_set>
#include <cmath>

#define VAR   0
#define NOT   1
#define BIN   2
#define NOT_T 3
#define BIN_T 4

#define FUN_CNT 4

struct Expr {
	union {
		struct {
			Expr *left;
			Expr *right;
			int   opr; // 0 .. 127
		}     bin_call; // - field call
		Expr *not_expr; // - field not
		int   var; //      - field var; 0 .. 127
		int   not_t; //    - index in tbl for NOT_T expr
		struct {
			int left;
			int right;
			int opr;
		}     bin_t; //    - field for call with table
	};
	char type; //          - field type
	char depth; //         - field depth
};

struct Table {
	std::vector<bool>               values;
	int                             var_count;
	std::string                     res_str;
};

typedef std::vector<bool> Env;
typedef std::vector<Env*> EnvSet;
typedef void* View;

extern "C" void         init_funs(); // call before all
extern "C" Table*       new_state(int*, int);
extern "C" void         delete_state(Table*);
extern "C" const char*  show_expr(Expr*,Table*);
extern "C" int          expr_depth(Expr*,Expr**);
extern "C" void         delete_expr(Expr*);
//extern "C" View   new_expr_view(); // to veiw in python
//extern "C" char*  show_expr(Expr*, View); // to view in python
//extern "C" void   del_expr_view(View); // to view in python

Expr*        e_var(int);
Expr*        e_not(Expr*);
Expr*        e_bin(int,Expr*,Expr*);
Expr*        e_bin_t(int,int,int);
Expr*        e_not(int);
bool         eval(Expr*, Env*, Expr**);
Expr*        clone_expr(Expr*, Expr**);
EnvSet*      gen_env_set(int var_count);
std::string  log_expr(Expr* e);
