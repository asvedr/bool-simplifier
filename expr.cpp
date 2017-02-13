#include <string>
#include <vector>
#include <sstream>
#include <unordered_set>
#include <cmath>

/*
 * void  init_funs();
 * void* new_state(int*, int); - (values, var_count) -> new_state
 * void  delete_state(void*); - clear memory from state
 * char* find_analog(void*) - calculate analog and return as string. after result you should delete_state
 */

using namespace std;

struct Expr {
	union {
		struct {
			Expr *left;
			Expr *right;
			int opr; // 0 .. 127
		} bin_call;
		Expr *not_expr;
		int var; // 0 .. 127
	}
	char type;
	char depth;
};
#define VAR 0
#define NOT 1
#define BIN 2

typedef bool(*BinFun)(bool,bool);
struct Table {
	vector<bool> values;
	int          var_count;
	string       res_str;
	history      unordered_set<string>;
}

extern "C" Table* new_state(int* tbl, int var_cnt) {
	Table* tbl = new Table;
	int tbl_len = pow(2, var_cnt);
	tbl -> values.reserve(tbl_len);
	for(int i=0; i<tbl_len; ++i) {
		tbl -> values.push_back(tbl[i]);
	}
	tbl -> var_count = var_cnt;
}

extern "C" delete_state(Table* t) {
	delete t;
}

#define FUN_CNT 4
#define RAND rand

BinFun funs[FUN_CNT];
char* fun_names[FUN_CNT];

static bool f_and(bool a, bool b) {
	return a && b;
}
static bool f_or(bool a, bool b) {
	return a || b;
}
static bool f_eq(bool a, bool b) {
	return a == b;
}
static bool f_neq(bool a, bool b) {
	return a != b;
}

// function for call in PY
extern "C" void init_funs() {
	funs[0] = &f_and;
	fun_names[0] = "&&";
	funs[1] = &f_or;
	fun_names[1] = "||";
	funs[2] = &f_eq;
	fun_names[2] = "==";
	funs[3] = &f_neq;
	fun_names[3] = "!=";
}

inline static Expr* rand_combine(vector<Expr*> exprs) {
	int op_id = RAND() % (FUN_CNT + 1);
	if(op_id == 0) {
		// NOT
		Expr* e = exprs[RAND() % exprs.size()];
		Expr* res = new Expr;
		res -> type = NOT;
		res -> depth = e -> depth + 1;
		res -> not_expr = e;
		return res;
	} else {
		// BIN OP
		int func = op_id - 1;
		Expr* a = exprs[RAND() % exprs.size()];
		Expr* b = exprs[RAND() % exprs.size()];
		Expr* res = new Expr;
		res -> type = BIN;
		res -> depth = a -> depth + b -> depth + 1;
		res -> bin_call.left = a;
		res -> bin_call.right = b;
		res -> bin_call.opr = op_id;
		return res;
	}
}

inline static void expr_hash(Expr* expr, string &out) {
	out.clear();
	vector<Expr*> stack;
	stack.push_back(expr);
	while(!stack.is_empty()) {
		expr = stack.pop_back();
		switch(expr.type) {
			case VAR:
				out.push_back('v');
				out.push_back((char)expr -> var);
			break;
			case NOT:
				out.push_back('!');
				stack.push_back(expr -> not_expr);
			break;
			case BIN:
				out.push_back('f');
				out.push_back((char)expr -> bin_call.opr);
				stack.push_back(expr -> bin_call.left);
				stack.push_back(expr -> bin_call.right);
			break;
		} 
	}
}

inline static void expr_str(Expr* expr, string &out) {
	out.clear();
	stringstream ss;
	vector<Expr*> stack;
	stack.push_back(expr);
	while(!stack.is_empty()) {
		expr = stack.pop_back();
		switch(expr.type) {
			case VAR:
				ss << "VAR " << expr -> var << " ";
			break;
			case NOT:
				ss << "NOT " << stack.push_back(expr -> not_expr) << " ";
			break;
			case BIN:
				ss << "BIN " << op_names[expr -> bin_call.opr] << " ";
				stack.push_back(expr -> bin_call.left);
				stack.push_back(expr -> bin_call.right);
			break;
		} 
	}
	out = ss.str();
}

extern "C" char* find_analog(Table* tbl) {
	
}
