#include <string>
#include <vector>
#include <sstream>
#include <unordered_set>
#include <cmath>

#define VAR 0
#define NOT 1
#define BIN 2

struct Expr {
	union {
		struct {
			Expr *left;
			Expr *right;
			int   opr; // 0 .. 127
		}     bin_call; // - field call
		Expr *not_expr; // - field not
		int   var; //      - field var; 0 .. 127
	};
	char type; //          - field type
	char depth; //         - field depth
};

struct Table {
	std::vector<bool>               values;
	int                             var_count;
	std::string                     res_str;
	std::unordered_set<std::string> history;
};

typedef std::vector<bool> Env;
typedef std::vector<Env*> EnvSet;

extern "C" Table* new_state(int*, int);
extern "C" void   delete_state(Table*);
extern "C" void   init_funs();
extern "C" char*  find_analog(Table*);

bool eval(Expr*, Env*);
