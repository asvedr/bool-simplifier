use expr::*;
use std::fmt::Write;
pub use std::collections::BTreeSet;

pub type Values = BTreeSet<char>;
pub struct Diapason {
    pub vars    : Vec<char>,
    pub val_tbl : Vec<Values>
}

// defined pub eval(&Expr,&Values) -> bool
// defined pub get_vars(&Expr) -> Vec<char>

impl Diapason {
    pub fn from_expr(expr : &Expr) -> Diapason {
        let vars = get_vars(expr);
        let tbl = gen_tbl(&vars);
        Diapason {
            vars    : vars,
            val_tbl : tbl
        }
    }
    pub fn new(vars : Vec<char>) -> Diapason {
        let tbl = gen_tbl(&vars);
        Diapason {
            vars    : vars,
            val_tbl : tbl
        }
    }
    pub fn table_for(&self, expr : &Expr) -> Vec<bool> {
        let mut acc = vec![];
        acc.reserve(self.val_tbl.len());
        for vals in self.val_tbl.iter() {
            acc.push(eval(expr, vals));
        }
        acc
    }
    pub fn cmp_with_table(&self, expr : &Expr, tbl : &Vec<bool>) -> bool {
        for i in 0 .. self.val_tbl.len() {
            if eval(expr, &self.val_tbl[i]) != tbl[i] {
                return false
            }
        }
        true
    }
    pub fn print(&self, expr : Option<&Expr>) {
        let mut line = format!("|");
        for x in self.vars.iter() {
            let _ = write!(line, "{}|", x);
        }
        match expr {
            Some(ref e) => {
                let _ = write!(line, "{}|", e.to_str());
            },
            _ => ()
        }
        println!("{}",line);
        for i in 0 .. self.val_tbl.len() {
            let vals = &self.val_tbl[i];
            line.clear();
            line.push('|');
            for x in self.vars.iter() {
                let v = if vals.contains(x) {
                    '1'
                } else {
                    '0'
                };
                let _ = write!(line, "{}|", v);
            }
            match expr {
                Some(ref e) => {
                    let _ = write!(line, "{}|", if eval(e, &self.val_tbl[i]) {'1'} else {'0'});
                    //write!(line, 
                },
                _ => ()
            }
            println!("{}",line);
        }
    }
    pub fn is_eq(&self, a : &Expr, b : &Expr) -> bool {
        for vals in self.val_tbl.iter() {
            if eval(a, vals) != eval(b, vals) {
                return false;
            }
        }
        true
    }
}

enum Cmd<'a> {
    Val(bool),
    Not,
    Fun(&'a Fn(bool,bool) -> bool)
}

pub fn eval(expr : &Expr, set : &Values) -> bool {
    //let set = &self.val_tbl[set_num];
    let mut stack = vec![expr];
    let mut eval_s = vec![];
    let mut log_s = vec![];
    macro_rules! epush {($c:expr, $l:expr) => {{
        eval_s.push($c);
        log_s.push($l.to_string());
    }};}
    while let Some(e) = stack.pop() {
        match *e {
            Expr::Var(ref n) => epush!(Cmd::Val(set.contains(n)), n),
            Expr::Not(ref e) => {
                epush!(Cmd::Not, "!");
                stack.push(e);
            },
            Expr::Bin(ref a, ref b, ref o) => {
                epush!(Cmd::Fun(&*o.func), o.name);
                stack.push(b);
                stack.push(a);
            },
            Expr::If(ref c, ref t, ref e) => {
                if eval(&**c, set) {
                    stack.push(t);
                } else {
                    stack.push(e);
                }
            },
            Expr::Val(ref b) => {
                if *b {
                    epush!(Cmd::Val(true), 'T');
                } else {
                    epush!(Cmd::Val(false), 'F');
                }
            }
        }
    }
    //println!("{:?}", log_s);
    let mut stack = vec![];
    while let Some(e) = eval_s.pop() {
        match e {
            Cmd::Val(b) => stack.push(b),
            Cmd::Not => {
                let a = stack[0];
                stack.pop();
                stack.push(!a)
            },
            Cmd::Fun(f) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(f(a,b));
            }
        }
    }
    //println!("{:?}", stack);
    return stack[0];
}

pub fn get_vars(expr : &Expr) -> Vec<char> {
    let mut set : Values = BTreeSet::new();
    let mut stack : Vec<&Expr> = vec![expr];
    while let Some(val) = stack.pop() {
        match *val {
            Expr::Var(ref c) => {
                set.insert(*c);
            },
            Expr::Bin(ref a, ref b, _) => {
                stack.push(&**a);
                stack.push(&**b);
            },
            Expr::Not(ref e) => stack.push(&**e),
            Expr::If(ref c, ref t, ref e) => {
                stack.push(&**c);
                stack.push(&**t);
                stack.push(&**e);
            },
            Expr::Val(_) => ()
        }
    }
    return set.into_iter().collect();
}

fn gen_tbl(vars : &Vec<char>) -> Vec<Values> {
    let mut old = vec![BTreeSet::new()];
    let mut new = vec![];
    for v in vars.iter() {
        for mut set in old.into_iter() {
            new.push(set.clone());
            set.insert(*v);
            new.push(set);
        }
        old = new;
        new = vec![];
    }
    old
}
