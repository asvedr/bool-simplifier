use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use rand::random;

pub enum Expr {
    Bin(Bin),
    Not(RExpr),
    Var(usize),
    Hash(usize)
}

#[derive(PartialEq)]
pub enum Opr {
    And, Or, Eq, Xor
}

impl Opr {
    pub fn random() -> Opr {
        match random::<usize>() % 4 {
            0 => Opr::And,
            1 => Opr::Or,
            2 => Opr::Eq,
            3 => Opr::Xor,
            _ => unreachable!()
        }
    }
}

pub struct Bin {
    pub opr   : Opr,
    pub left  : RExpr,
    pub right : RExpr
}

pub type RExpr = Rc<Expr>;
pub type Env   = Vec<bool>;
pub type Envs  = Vec<Vec<bool>>;
pub type HashStore = HashMap<usize,RExpr>;

impl Expr {
    pub fn eval(&self, env : &Env, tbl : Option<&HashStore>) -> bool {
        match *self {
            Expr::Var(ref n) => env[*n],
            Expr::Bin(ref bin) => {
                let a = bin.left.eval(env, tbl);
                let b = bin.right.eval(env, tbl);
                match bin.opr {
                    Opr::And => a && b,
                    Opr::Or  => a || b,
                    Opr::Eq  => a == b,
                    Opr::Xor => a != b
                }
            },
            Expr::Not(ref e) => !e.eval(env, tbl),
            Expr::Hash(ref hash) => {
                match tbl {
                    Some(ref hash_t) => {
                        match hash_t.get(hash) {
                            Some(ref val) => val.eval(env, tbl),
                            _ => panic!("key error")
                        }
                    },
                    _ => panic!("no hash tbl found")
                }
            }
        }
    }
    pub fn is_not(&self, tbl : Option<&HashStore>) -> bool {
        match *self {
            Expr::Not(_) => true,
            Expr::Hash(ref hash) => {
                match tbl {
                    Some(ref hash_t) => {
                        match hash_t.get(hash) {
                            Some(ref val) => val.is_not(tbl),
                            _ => panic!("key error")
                        }
                    },
                    _ => panic!("no hash tbl found")
                }
            },
            _ => false
        }
    }
    pub fn bin(op : Opr, a : RExpr, b : RExpr) -> RExpr {
        Rc::new(Expr::Bin(Bin{opr : op, left : a, right : b}))
    }
    pub fn not(a : RExpr) -> RExpr {
        Rc::new(Expr::Not(a))
    }
    pub fn var(v : usize) -> RExpr {
        Rc::new(Expr::Var(v))
    }
    pub fn hash_item(i : usize) -> RExpr {
        Rc::new(Expr::Hash(i))
    }
    pub fn hash(&self, envs : &Envs, tbl : Option<&HashStore>, out : &mut usize, tautology : &mut bool) {
        let mut p2 = 1;
        let mut hash = 0;
        let mut all_t = true;
        let mut all_f = true;
        for env in envs.iter() {
            let ok = self.eval(env, tbl);
            if ok {
                hash = hash | p2;
            }
            all_t = all_t && ok;
            all_f = all_f && !ok;
            p2 *= 2;
        }
        *out = hash;
        *tautology = all_t || all_f;
    }
    fn show_loc(&self, tbl : Option<&HashStore>, out : &mut String) -> fmt::Result {
        match *self {
            Expr::Var(n) => write!(out, "V{}", n),
            Expr::Bin(ref bin) => {
                write!(out, "(")?;
                bin.left.show_loc(tbl, out)?;
                let o = match bin.opr {
                    Opr::And => "&&",
                    Opr::Or => "||",
                    Opr::Eq => "==",
                    Opr::Xor => "!="
                };
                write!(out, " {} ", o)?;
                bin.right.show_loc(tbl, out)?;
                write!(out, ")")
            },
            Expr::Not(ref e) => {
                write!(out, "~")?;
                e.show_loc(tbl, out)
            },
            Expr::Hash(ref hash) => {
                match tbl {
                    Some(ref hash_t) => {
                        match hash_t.get(hash) {
                            Some(ref val) => val.show_loc(tbl, out),
                            _ => panic!("key error")
                        }
                    },
                    _ => panic!("no hash tbl found")
                }
            }
        }
    }
    pub fn show(&self, tbl : Option<&HashStore>) -> String {
        let mut out = String::new();
        self.show_loc(tbl, &mut out);
        return out;
    }
    pub fn depth(&self, tbl : Option<&HashStore>) -> usize {
        match *self {
            Expr::Var(_) => 1,
            Expr::Bin(ref bin) =>
                bin.left.depth(tbl) + bin.right.depth(tbl) + 1,
            Expr::Not(ref e) => e.depth(tbl) + 1,
            Expr::Hash(ref hash) => {
                match tbl {
                    Some(ref hash_t) => {
                        match hash_t.get(hash) {
                            Some(ref val) => val.depth(tbl),
                            _ => panic!("key error")
                        }
                    },
                    _ => panic!("no hash tbl found")
                }
            }
        }
    }
}
