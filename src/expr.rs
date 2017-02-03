pub use std::rc::Rc;
use reserr::*;

pub type RExpr = Rc<Expr>;
pub type ROp = Rc<Op>;
pub enum Expr {
    Var(char),
    If(RExpr,RExpr,RExpr),
    Not(RExpr),
    Bin(RExpr,RExpr,ROp),
    Val(bool)
}
pub struct Op {
    pub func : Box<Fn(bool,bool) -> bool>,
    pub name : String,
    pub code : char
}

#[macro_export]
macro_rules! var {($name:expr) => {Rc::new(Expr::Var($name))} }
#[macro_export]
macro_rules! cnd {($cnd:expr, $th:expr, $el:expr) => {Rc::new(Expr::If($cnd, $th, $el))}}
#[macro_export]
macro_rules! bin {($op:expr, $a:expr, $b:expr) => {Rc::new(Expr::Bin($a, $b, $op))}}
#[macro_export]
macro_rules! not {($name:expr) => {Rc::new(Expr::Not($name))} }
#[macro_export]
macro_rules! val {($name:expr) => {Rc::new(Expr::Val($name))} }

impl Op {
    pub fn new(code : char, name : &str, f : Box<Fn(bool,bool) -> bool>) -> ROp {
        Rc::new(Op {
            func : f,
            name : name.to_string(),
            code : code
        })
    }
}

impl Expr {
    #[inline(always)]
    pub fn height(&self) -> usize {
        // XXX THINK ABOUT FAST HEIGHT CALCULATION
        // MAYBE USE CACHE AND UPD FUNCTION
        match *self {
            Expr::Bin(ref a, ref b,_) => {
                let a = a.height();
                let b = b.height();
            },
            Expr::If(ref a, ref b, ref c) => {
                let a = a.height();
                let b = b.height();
                let c = c.height();
                if a < b {
                    a = b;
                }
                if a > c {
                    a
                } else {
                    c
                }
            },
            _ => 1
        }
    }
    pub fn to_str(&self) -> String {
        match *self {
            Expr::Var(ref n) => n.to_string(),
            Expr::If(ref c, ref t, ref e) =>
                format!("if({}){}{}{}else{}{}{}", c.to_str(), '{', t.to_str(), '}', '{', e.to_str(), '}'),
            Expr::Not(ref v) => format!("!({})", v.to_str()),
            Expr::Bin(ref a, ref b, ref op) => format!("({}) {} ({})", a.to_str(), op.name, b.to_str()),
            Expr::Val(ref b) => b.to_string()
        }
    }
    pub fn to_hash(&self) -> String {
        let mut res = String::new();
        let mut stack : Vec<&Expr> = vec![self];
        while let Some(e) = stack.pop() {
            match *e {
                Expr::Var(ref n) => res.push(*n),
                Expr::Not(ref e) => {
                    res.push('!');
                    stack.push(e);
                },
                Expr::Bin(ref a, ref b, ref o) => {
                    res.push(o.code);
                    stack.push(b);
                    stack.push(a);
                },
                Expr::If(ref c, ref t, ref e) => {
                    res.push('I');
                    stack.push(e);
                    stack.push(t);
                    stack.push(c);
                },
                Expr::Val(ref b) => {
                    if *b {
                        res.push('T');
                    } else {
                        res.push('F');
                    }
                }
            }
        }
        res
    }
    fn read_operand(lexer : &Lexer, mut curs : Cursor, ops : &Vec<ROp>) -> SynRes<RExpr> {
        let ans = lex!(lexer, &curs);
        if ans.val == "true" {
            syn_ok!(val!(true), ans.cursor);
        } else if ans.val == "false" {
            syn_ok!(val!(false), ans.cursor);
        } else if ans.val == "(" {
            let ans = Expr::read_one(lexer, ans.cursor, ops, false)?;
            curs = lex!(lexer, &ans.cursor, ")");
            syn_ok!(ans.val, curs);
        } else {
            let c = ans.val.chars().next().unwrap();
            if ans.val.len() == 1 && c >= 'a' && c <= 'z' {
                syn_ok!(var!(c), ans.cursor);
            } else {
                syn_throw!(format!("bad var name: {}", ans.val), curs)
            }
        }
    }
    fn build(mut vec : Vec<Result<RExpr,ROp>>) -> RExpr {
        if vec.len() == 1 {
            match vec.pop() {
                Some(Ok(e)) => e,
                _ => panic!()
            }
        } else {
            let a = match vec.pop().unwrap() {
                Ok(a) => a,
                _ => panic!()
            };
            let op = match vec.pop().unwrap() {
                Err(a) => a,
                _ => panic!()
            };
            let b = Expr::build(vec);
            bin!(op,b,a)
        }
    }
    fn read_one(lexer : &Lexer, mut curs : Cursor, ops : &Vec<ROp>, is_top : bool) -> SynRes<RExpr> {
        let mut acc = vec![];
        let mut need_fun = false;
        loop {
            let ans = lexer.lex(&curs);
            match ans {
                Ok(ans) => {
                    if need_fun {
                        if ans.val == ")" {
                            if is_top {
                                syn_throw!("unexpected ')'", curs)
                            } else {
                                syn_ok!(Expr::build(acc), curs)
                            }
                        } else {
                            let mut found : Option<ROp> = None;
                            for o in ops.iter() {
                                if o.name == ans.val {
                                    found = Some(o.clone());
                                    break;
                                }
                            }
                            match found {
                                None => syn_throw!(format!("bad operator '{}'", ans.val), curs),
                                Some(o) => {
                                    acc.push(Err(o));
                                    curs = ans.cursor;
                                    need_fun = false;
                                }
                            }
                        }
                    } else {
                        if ans.val == "!" {
                            let ans = Expr::read_operand(lexer, ans.cursor, ops)?;
                            acc.push(Ok(not!(ans.val)));
                            curs = ans.cursor;
                            need_fun = true;
                        } else if ans.val == "if" {
                            curs = lex!(lexer, &ans.cursor, "(");
                            let cond = Expr::read_one(lexer, curs, ops, false)?;
                            curs = lex!(lexer, &cond.cursor, ")");
                            curs = lex!(lexer, &curs, "(");
                            let th = Expr::read_one(lexer, curs, ops, false)?;
                            curs = lex!(lexer, &th.cursor, ")");
                            curs = lex!(lexer, &curs, "else");
                            curs = lex!(lexer, &curs, "(");
                            let el = Expr::read_one(lexer, curs, ops, false)?;
                            curs = lex!(lexer, &el.cursor, ")");
                            acc.push(Ok(cnd!(cond.val, th.val, el.val)));
                        } else {
                            let ans = Expr::read_operand(lexer, curs, ops)?;
                            acc.push(Ok(ans.val));
                            curs = ans.cursor;
                            need_fun = true;
                        }
                    }
                },
                Err(e) =>
                    if e.data == "EOF" && is_top {
                        syn_ok!(Expr::build(acc), curs);
                    } else {
                        syn_throw!(format!("{:?}",e), curs);
                    }
            }
        }
    }
    pub fn read(src : &str, ops : &Vec<ROp>) -> Result<RExpr,String> {
        let lexer = Lexer::new(src);
        match Expr::read_one(&lexer, Cursor::new(), ops, true) {
            Ok(a) => Ok(a.val),
            Err(e) => Err(format!("l:{}, c:{}, mess:{}", e[0].line, e[0].column, e[0].mess))
        }
    }
}
