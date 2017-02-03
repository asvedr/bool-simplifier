use expr::*;
use std::fmt::Write;
use std::collections::BTreeSet;

pub struct Evaluator {
    vars    : Vec<char>,
    val_tbl : Vec<BTreeSet<char>>
}

enum Cmd<'a> {
    Val(bool),
    Not,
    Fun(&'a Fn(bool,bool) -> bool)
}

impl Evaluator {
    pub fn new(expr : &Expr) -> Evaluator {
        let vars = get_vars(expr);
        let tbl = gen_tbl(&vars);
        Evaluator {
            vars    : vars,
            val_tbl : tbl
        }
    }
    pub fn eval(&self, expr : &Expr, set_num : usize) -> bool {
        let set = &self.val_tbl[set_num];
        let mut stack = vec![expr];
        let mut eval = vec![];
        while let Some(e) = stack.pop() {
            match *e {
                Expr::Var(ref n) => eval.push(Cmd::Val(set.contains(n))),
                Expr::Not(ref e) => {
                    eval.push(Cmd::Not);
                    stack.push(e);
                },
                Expr::Bin(ref a, ref b, ref o) => {
                    eval.push(Cmd::Fun(&*o.func));
                    stack.push(b);
                    stack.push(a);
                },
                Expr::If(ref c, ref t, ref e) => {
                    if self.eval(&**c, set_num) {
                        stack.push(t);
                    } else {
                        stack.push(e);
                    }
                },
                Expr::Val(ref b) => {
                    if *b {
                        eval.push(Cmd::Val(true));
                    } else {
                        eval.push(Cmd::Val(true));
                    }
                }
            }
        }
        let mut stack = vec![];
        while let Some(e) = eval.pop() {
            match e {
                Cmd::Val(b) => stack.push(b),
                Cmd::Not => {
                    let a = stack[0];
                    stack.pop();
                    stack.push(!a)
                },
                Cmd::Fun(f) => {
                    let a = stack[0];
                    stack.pop();
                    let b = stack[0];
                    stack.pop();
                    stack.push(f(a,b));
                }
            }
        }
        return stack[0];
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
                    let _ = write!(line, "{}|", if self.eval(e, i) {'1'} else {'0'});
                    //write!(line, 
                },
                _ => ()
            }
            println!("{}",line);
        }
    }
}

fn get_vars(expr : &Expr) -> Vec<char> {
    let mut set : BTreeSet<char> = BTreeSet::new();
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

fn gen_tbl(vars : &Vec<char>) -> Vec<BTreeSet<char>> {
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
