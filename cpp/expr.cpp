/*
#include <string>
#include <vector>
#include <sstream>
#include <unordered_set>
#include <cmath>
*/
#include "expr.h"

/*
 * void  init_funs();
 * void* new_state(int*, int); - (values, var_count) -> new_state
 * void  delete_state(void*); - clear memory from state
 */

using namespace std;

typedef bool(*BinFun)(bool,bool);

extern "C" Table* new_state(int* tbl_src, int var_cnt) {
	Table* tbl = new Table;
	int tbl_len = pow(2, var_cnt);
	tbl -> values.reserve(tbl_len);
	//for(int i=0; i<tbl_len; ++i) {
	for(int i=tbl_len-1; i>=0; --i) {
		tbl -> values.push_back(tbl_src[i]);
	}
	tbl -> var_count = var_cnt;
	return tbl;
}

extern "C" void delete_state(Table* t) {
	delete t;
}

//#define RAND rand

BinFun funs[FUN_CNT];
const char* fun_names[FUN_CNT];

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

bool eval(Expr* expr, Env* env) {
	switch(expr -> type) {
		case VAR:
			return (*env)[expr -> var];
		case NOT:
			return !eval(expr -> not_expr, env);
		case BIN:
			return funs[expr -> bin_call.opr](eval(expr -> bin_call.left, env), eval(expr -> bin_call.right, env));
	}
	return false;
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

/*
void expr_hash(Expr* expr, string &out) {
	out.clear();
	vector<Expr*> stack;
	stack.push_back(expr);
	while(!stack.empty()) {
		expr = stack.back();
		stack.pop_back();
		switch(expr -> type) {
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
*/

inline static void expr_str(Expr* expr, string &out) {
	out.clear();
	stringstream ss;
	vector<Expr*> stack;
	stack.push_back(expr);
	while(!stack.empty()) {
		expr = stack.back();
		stack.pop_back();
		switch(expr -> type) {
			case VAR:
				ss << "VAR " << expr -> var << " ";
			break;
			case NOT:
				ss << "NOT ";
				stack.push_back(expr -> not_expr);
			break;
			case BIN:
				ss << "BIN " << fun_names[expr -> bin_call.opr] << " ";
				stack.push_back(expr -> bin_call.left);
				stack.push_back(expr -> bin_call.right);
			break;
		} 
	}
	out = ss.str();
}

static string show_expr_rec(Expr* e) {
	stringstream ss;
	string a;
	string b;
	const char* o;
	switch(e -> type) {
		case VAR:
			ss << "V" << e -> var;
			break;
		case NOT:
			ss << "~" << show_expr_rec(e -> not_expr);
			break;
		case BIN:
			a = show_expr_rec(e -> bin_call.left);
			b = show_expr_rec(e -> bin_call.right);
			o = fun_names[e -> bin_call.opr];
			ss << "(" << a << " " << o << " " << b << ")";
			break;
	}
	return ss.str();
}

extern "C" const char* show_expr(Expr* e, Table* t) {
	//expr_str(e, t -> res_str);
	t -> res_str = show_expr_rec(e);
	return t -> res_str.c_str();
}

/*
extern "C" void* new_expr_view() {
	return (void*)(new string());
}

extern "C" char* show_expr(Expr* e, void* view) {
	expr_str(e, *(string*)view);
	return view.s_str();
}

extern "C" void del_expr_view(void* view) {
	delete (string*)view;
}
*/

EnvSet* gen_env_set(int vcount) {
	EnvSet buf;
	EnvSet *acc = new vector<Env*>();
	acc -> push_back(new vector<bool>());
	for(int var = 0; var < vcount; ++var) {
		buf.clear();
		for(auto env = acc -> begin(); env != acc -> end(); ++env) {
			Env* alternate = new vector<bool>(**env);
			alternate -> push_back(false);
			(*env) -> push_back(true);
			buf.push_back(alternate);
		}
		for(auto env = buf.begin(); env != buf.end(); ++env) {
			acc -> push_back(*env);
		}
	}
	return acc;
}

Expr* clone_expr(Expr* e) {
	auto new_e = new Expr;
	new_e -> type = e -> type;
	new_e -> depth = e -> depth;
	switch(e -> type) {
		case VAR:
			new_e -> var = e -> var;
			break;
		case NOT:
			new_e -> not_expr = clone_expr(e -> not_expr);
			break;
		case BIN:
			new_e -> bin_call.opr   = e -> bin_call.opr;
			new_e -> bin_call.left  = clone_expr(e -> bin_call.left);
			new_e -> bin_call.right = clone_expr(e -> bin_call.right);
			break;
	}
	return new_e;
}

Expr* e_var(int v) {
	Expr* e = new Expr;
	e -> type = VAR;
	e -> depth = 1;
	e -> var = v;
	return e;
}

Expr* e_not(Expr* chld) {
	Expr* e = new Expr;
	e -> type = NOT;
	e -> depth = chld -> depth + 1;
	e -> not_expr = chld;
	return e;
}

Expr* e_bin(int op, Expr* a, Expr* b) {
	Expr* e = new Expr;
	e -> type = BIN;
	e -> depth = a -> depth + b -> depth + 1;
	e -> bin_call.left = a;
	e -> bin_call.right = b;
	e -> bin_call.opr = op;
	return e;
}

string log_expr(Expr* e) {
	string res;
	expr_str(e, res);
	return res;
}

extern "C" int expr_depth(Expr* e) {
	return e -> depth;
}

extern "C" void delete_expr(Expr* e) {
	delete e;
}
