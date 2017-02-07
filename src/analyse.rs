use expr::*;
use eval::*;

pub fn used_unused(expr : &Expr) -> (Vec<char>,Vec<char>) {
    let mut diap = Diapason::from_expr(expr);
    let vars  = diap.vars.clone();
    let mut has_power = vec![false;vars.len()];
    let vlen = vars.len();
    for tbl in diap.val_tbl.iter_mut() {
        let a = eval(expr, tbl);
        for i in 0 .. vlen {
            if !has_power[i] {
                let var = vars[i];
                let b;
                if tbl.contains(&var) {
                    tbl.remove(&var);
                    b = eval(expr, tbl);
                    tbl.insert(var);
                } else {
                    tbl.insert(var);
                    b = eval(expr, tbl);
                    tbl.remove(&var);
                }
                has_power[i] = a != b;
            }
        }
    }
    let mut used = vec![];
    let mut unused = vec![];
    for i in 0 .. vlen {
        if has_power[i] {
            used.push(vars[i])
        } else {
            unused.push(vars[i])
        }
    }
    (used, unused)
}

pub fn is_tautology(expr : &Expr, diap : Option<&Diapason>) -> bool {
    let mut yes = 0;
    let mut no  = 0;
    macro_rules! check {($diap:expr) => {{
        for vals in $diap.val_tbl.iter() {
            if eval(expr, vals) {
                yes += 1;
            } else {
                no += 1;
            }
        }
    }};}
    match diap {
        Some(d) => check!(d),
        _ => {
            let d = Diapason::from_expr(expr);
            check!(d)
        }
    }
    return yes * no == 0;
}

