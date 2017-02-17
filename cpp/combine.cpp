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

inline static Expr* rand_combine(vector<Expr*> &exprs, Expr** hash_tbl) {
	int func = RAND_INT(FUN_CNT);
	int size = exprs.size();
	LOG("combine","in");
	Expr* a = exprs[RAND_INT(size)];
	LOG("combine-l", log_expr(a, hash_tbl));
	Expr* b;
	do {
		b = exprs[RAND_INT(size)];
	} while(b == a);
	LOG("combine-r", log_expr(b, hash_tbl));
	Expr* res = e_bin_t(func, a -> hash, b -> hash);//e_bin(func, a, b);
	LOG("combine-res", log_expr(res, hash_tbl));
	return res;
}

static void check_tbl(Expr* e, EnvSet* envs, const vector<bool>* target, bool& target_match, bool& tautology, int& hash) {
	int i = 0;
	bool ok = true;
	bool all_t = true;
	bool all_f = true;
	hash = 0;
	int p2 = 1;
	for(Env* env : *envs) {
		bool val = eval(e, env, NULL);
		ok = ok && val == (*target)[i];
		all_t = all_t && val;
		all_f = all_f && !val;
		//hash[i] = (val)? '1': '0';
		hash = hash | (p2 * val);
		++i;
		p2 *= 2;
	}
	target_match = ok;
	tautology = all_t || all_f;
	e -> hash = hash;
}

static Expr* finish(vector<Expr*>& variants, Expr* e) {
	Expr* res = clone_expr(e, NULL);
	for(Expr* ptr : variants) {
		//LOG("del", "finish");
		delete ptr;
	}
	return res;
}

extern "C" Expr* find_analog(Table* tbl, int iter_count, int max_depth) {
	SRAND;
	vector<Expr*> variants;
	//unordered_set<int> history;
	EnvSet* envs = gen_env_set(tbl -> var_count);
	int hash;
	const vector<bool>* target = &tbl -> values;
	int hash_tbl_len = 1 << (target -> size());
	Expr** hash_tbl  = new Expr*[hash_tbl_len];
	for(int i=0; i<hash_tbl_len; ++i)
		hash_tbl[i] = NULL;
	bool is_target;
	bool is_tautology;
	//for(int i=0; i<target -> size(); ++i) {
	//	hash += '0';
	//}
#define ADD_VAR(e) {\
	e -> pos_in_vec = variants.size();\
	variants.push_back(e);\
	check_tbl(e, envs, target, is_target, is_tautology, hash);\
	if(is_target) {\
		return finish(variants, e);\
	}\
	hash_tbl[hash] = e;\
	/*history.insert(hash);*/\
}
	for(int i=0; i < tbl -> var_count; ++i) {
		Expr* e = e_var(i);
		ADD_VAR(e);
		e = e_not(e);
		ADD_VAR(e);
	}
	int v_size = variants.size();
	int res_tbl_index = -1;
#define NEW_VAL(e) {\
	e -> pos_in_vec = v_size;\
	variants.push_back(e);\
	hash_tbl[hash] = e;\
	v_size++;\
}
#define REPLACE_IN_TBL(e) \
	if(expr_depth(e, hash_tbl) < expr_depth(hash_tbl[hash], hash_tbl)) {\
		variants[hash_tbl[hash] -> pos_in_vec] = e;\
		e -> pos_in_vec = hash_tbl[hash] -> pos_in_vec;\
		delete hash_tbl[hash];\
		hash_tbl[hash] = e;\
	} else {\
		delete e;\
	}
#define TRY_THIS(e) {\
	check_tbl(e, envs, target, is_target, is_tautology, hash);\
	if(is_target) {\
		/*variants.push_back(e);\
		return finish(variants, e);\*/\
		res_tbl_index = hash;\
		if(hash_tbl[hash] != NULL) {\
			REPLACE_IN_TBL(e);\
		} else {\
			NEW_VAL(e);\
		}\
	} else if(/*history[hash] != NULL ||*/ is_tautology || expr_depth(e, hash_tbl) > max_depth) {\
		delete e;\
		continue;\
	} else if(hash_tbl[hash] != NULL) {\
		REPLACE_IN_TBL(e);\
	} else {\
		NEW_VAL(e);\
	}\
}
	while(iter_count > 0) {
		LOG("loop","in");
		if(RAND_BOOL) {
			// NOT BRANCH
			int i = RAND_INT(v_size);
			LOG2("loop-not", i, v_size);
			Expr* e = variants[i];
			LOG("loop-not", log_expr(e, hash_tbl));
			if(e -> type == NOT) {
				//LOG("del", "not");
				//delete e;
				continue;
			} else {
				e = e_not_t(e -> hash);//e_not(e);
				LOG("loop-not-res", log_expr(e, hash_tbl));
				TRY_THIS(e);
			}
		} else {
			// BIN BRANCH
			LOG("loop","bin");
			Expr* e = rand_combine(variants, hash_tbl);
			TRY_THIS(e);
		}
		--iter_count;
	}
	if(res_tbl_index != -1) {
		return finish(variants, hash_tbl[res_tbl_index]);
	} else {
		return NULL;
	}
}
