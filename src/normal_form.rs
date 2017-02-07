use expr::*;
use eval::*;
use analyse::*;
use std::fmt::Write;
//use rand::random;

macro_rules! sknf_to_expr {($or:expr, $and:expr, $vec:expr) => {{
    let mut root = None;
    for dis in $vec.into_iter () {
        let mut sum = None;
        for e in dis.into_iter() {
            let ns = match sum {
                None => e,
                Some(s) => bin!($or, e, s)
            };
            sum = Some(ns);
        }
        let kon = match root {
            None => sum.unwrap(),
            Some(kon) => bin!($and, sum.unwrap(), kon)
        };
        root = Some(kon);
    }
    root.unwrap()
}};}

pub struct NF {
    seq     : Vec<Vec<RExpr>>,
    is_sknf : bool
}

impl NF {
    pub fn to_str(self : &NF) -> String {
        let (op_out, op_in) =
            if self.is_sknf {
                ("&","|")
            } else {
                ("|","&")
            };
        let mut acc = String::new();
        for sub in self.seq.iter() {
            let mut local = String::new();
            for item in sub.iter() {
                if local.len() == 0 {
                    local = item.to_str();
                } else {
                    let _ = write!(local, "{}{}", op_in, item.to_str());
                }
            }
            if acc.len() == 0 {
                let _ = write!(acc, "({})", local);
            } else {
                let _ = write!(acc, " {} ({})", op_out, local);
            }
        }
        acc
    }
    pub fn from_expr(e : &Expr) -> NF {
        let tbl = Diapason::from_expr(e).table_for(e);
        let tlen = tbl.len();
        let mut t = 0;
        for b in tbl {
            if b {
                t += 1;
            }
        }
        if t < tlen - 1 {
            NF {
                seq : sdnf(e),
                is_sknf : false
            }
        } else {
            NF {
                seq : sknf(e),
                is_sknf : true
            }
        }
    }
}

pub fn sknf(expr : &Expr) -> Vec<Vec<RExpr>> {
    let (vars,_) = used_unused(expr);
    let vlen = vars.len();
    let diap = Diapason::new(vars);
    // sknf : [[.. v .. ] ^ ..]
    let mut sknf : Vec<Vec<RExpr>> = vec![];
    for vals in diap.val_tbl.iter() {
        if !eval(expr, vals) {
            let mut seq = vec![];
            for i in 0 .. vlen {
                if vals.contains(&diap.vars[i]) {
                    seq.push(not!(var!(diap.vars[i])));
                } else {
                    seq.push(var!(diap.vars[i]))
                }
            }
            sknf.push(seq);
        }
    }
    sknf
}

pub fn sdnf(expr : &Expr) -> Vec<Vec<RExpr>> {
    let (vars,_) = used_unused(expr);
    let vlen = vars.len();
    let diap = Diapason::new(vars);
    // sknf : [[.. ^ .. ] v ..]
    let mut sdnf : Vec<Vec<RExpr>> = vec![];
    for vals in diap.val_tbl.iter() {
        if eval(expr, vals) {
            let mut seq = vec![];
            for i in 0 .. vlen {
                if !vals.contains(&diap.vars[i]) {
                    seq.push(not!(var!(diap.vars[i])));
                } else {
                    seq.push(var!(diap.vars[i]))
                }
            }
            sdnf.push(seq);
        }
    }
    sdnf
}
