use std::rc::Rc;
use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Write;
use rand::random;

pub enum Expr {
    Bin(Bin),
    Not(RExpr),
    Var(usize)
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

impl Expr {
    pub fn eval(&self, env : &Env) -> bool {
        match *self {
            Expr::Var(ref n) => env[*n],
            Expr::Bin(ref bin) => {
                let a = bin.left.eval(env);
                let b = bin.right.eval(env);
                match bin.opr {
                    Opr::And => a && b,
                    Opr::Or  => a || b,
                    Opr::Eq  => a == b,
                    Opr::Xor => a != b
                }
            },
            Expr::Not(ref e) => !e.eval(env)
        }
    }
    fn var_count_priv(&self, set : &mut BTreeSet<usize>) {
        match *self {
            Expr::Var(ref n) => {
                set.insert(*n);
            },
            Expr::Bin(ref bin) => {
                bin.left.var_count_priv(set);
                bin.right.var_count_priv(set);
            },
            Expr::Not(ref e) => e.var_count_priv(set)
        }
    }
    pub fn var_count(&self) -> usize {
        let mut set = BTreeSet::new();
        self.var_count_priv(&mut set);
        return set.len()
    }
    pub fn is_not(&self) -> bool {
        match *self {
            Expr::Not(_) => true,
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
    pub fn hash(&self, envs : &Envs, out : &mut usize, tautology : &mut bool) {
        let mut p2 = 1;
        let mut hash = 0;
        let mut all_t = true;
        let mut all_f = true;
        for env in envs.iter() {
            let ok = self.eval(env);
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
    fn show_loc(&self, out : &mut String) -> fmt::Result {
        match *self {
            Expr::Var(n) => write!(out, "V{}", n),
            Expr::Bin(ref bin) => {
                write!(out, "(")?;
                bin.left.show_loc(out)?;
                let o = match bin.opr {
                    Opr::And => "&&",
                    Opr::Or => "||",
                    Opr::Eq => "==",
                    Opr::Xor => "!="
                };
                write!(out, " {} ", o)?;
                bin.right.show_loc(out)?;
                write!(out, ")")
            },
            Expr::Not(ref e) => {
                write!(out, "~")?;
                e.show_loc(out)
            }
        }
    }
    pub fn show(&self) -> String {
        let mut out = String::new();
        self.show_loc(&mut out);
        return out;
    }
    pub fn depth(&self) -> usize {
        match *self {
            Expr::Var(_) => 1,
            Expr::Bin(ref bin) =>
                bin.left.depth() + bin.right.depth() + 1,
            Expr::Not(ref e) => e.depth() + 1,
        }
    }
    /*pub fn capacity(&self, d_count : f64, d_depth : f64) -> f64 {
        (self.depth() as f64) * d_depth + (self.var_count() as f64) * d_count
    }*/
}
