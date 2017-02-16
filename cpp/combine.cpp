#include "combine.h"
#include <vector>
#include <cstdlib>
#include <unordered_set>
#include <iostream>

//#define DEBUG true

#ifdef DEBUG
#define LOG(key,val) {cerr << "DEBUG " << key << ": " << val << endl;}
#define LOG2(key,a,b) {cerr << "DEBUG " << key << ": " << a << ", " << b << endl;}
#else
#define LOG(key,name) {}
#define LOG2(key,a,b) {}
#endif

#define RAND_INT(n) (rand() % n)
#define RAND_BOOL (rand() % 2)
#define SRAND srand(time(0))

using namespace std;

inline static Expr* rand_combine(vector<Expr*> &exprs) {
	int func = RAND_INT(FUN_CNT);
	int size = exprs.size();
	LOG("combine","in");
	Expr* a = exprs[RAND_INT(size)];
	LOG("combine-l", log_expr(a));
	Expr* b;
	do {
		b = exprs[RAND_INT(size)];
	} while(b == a);
	LOG("combine-r", log_expr(b));
	Expr* res = e_bin(func, a, b);
	LOG("combine-res", log_expr(res));
	return res;
}

static void check_tbl(Expr* e, EnvSet* envs, const vector<bool>* target, bool& target_match, bool& tautology, string& hash) {
	int i = 0;
	bool ok = true;
	bool all_t = true;
	bool all_f = true;
	for(Env* env : *envs) {
		bool val = eval(e, env);
		ok = ok && val == (*target)[i];
		all_t = all_t && val;
		all_f = all_f && !val;
		hash[i] = (val)? '1': '0';
		++i;
	}
	target_match = ok;
	tautology = all_t || all_f;
}

static Expr* finish(vector<Expr*>& variants, Expr* e) {
	Expr* res = clone_expr(e);
	for(Expr* ptr : variants) {
		//LOG("del", "finish");
		delete ptr;
	}
	return res;
}

extern "C" Expr* find_analog(Table* tbl, int iter_count, int max_depth) {
	SRAND;
	vector<Expr*> variants;
	unordered_set<string> history;
	EnvSet* envs = gen_env_set(tbl -> var_count);
	string hash;
	const vector<bool>* target = &tbl -> values;
	bool is_target;
	bool is_tautology;
	for(int i=0; i<target -> size(); ++i) {
		hash += '0';
	}
#define ADD_VAR(e) {\
	variants.push_back(e);\
	/*++v_size;*/\
	check_tbl(e, envs, target, is_target, is_tautology, hash);\
	if(is_target) {\
		return finish(variants, e);\
	}\
	history.insert(hash);\
}
	for(int i=0; i < tbl -> var_count; ++i) {
		Expr* e = e_var(i);
		ADD_VAR(e);
		e = e_not(e);
		ADD_VAR(e);
	}
	int v_size = variants.size();
#define TRY_THIS(e) {\
	check_tbl(e, envs, target, is_target, is_tautology, hash);\
	if(is_target) {\
		variants.push_back(e);\
		return finish(variants, e);\
	} else if(history.find(hash) != history.end() || is_tautology || e -> depth > max_depth) {\
		delete e;\
		continue;\
	} else {\
		LOG("newvar", log_expr(e));\
		variants.push_back(e);\
		++v_size;\
	}\
}
	//for(Expr* e : variants) {
	//	cerr << "VAR: " << log_expr(e) << endl;
	//}
	while(iter_count > 0) {
		LOG("loop","in");
		if(RAND_BOOL) {
			// NOT BRANCH
			int i = RAND_INT(v_size);
			LOG2("loop-not", i, v_size);
			Expr* e = variants[i];
			LOG("loop-not", log_expr(e));
			if(e -> type == NOT) {
				//LOG("del", "not");
				//delete e;
				continue;
			} else {
				e = e_not(e);
				LOG("loop-not-res", log_expr(e));
				TRY_THIS(e);
			}
		} else {
			// BIN BRANCH
			LOG("loop","bin");
			Expr* e = rand_combine(variants);
			TRY_THIS(e);
		}
		--iter_count;
	}
	return NULL;
}
