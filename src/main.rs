extern crate rand;

mod lexer;
#[macro_use]
mod reserr;
#[macro_use]
mod expr;
mod eval;

use expr::*;
use eval::*;
use std::env;

fn main() {
    let ops = vec![
        Op::new('&', "&&", Box::new(|a : bool, b : bool|{a && b})),
        Op::new('|', "||", Box::new(|a : bool, b : bool|{a || b})),
        Op::new('=', "==", Box::new(|a : bool, b : bool|{a == b})),
        Op::new('/', "!=", Box::new(|a : bool, b : bool|{a != b}))
    ];
    //let e = bin!(and, cnd!(var!('X'), val!(true), val!(false)), not!(var!('Y')));
    //let mut line = String::new();
    //stdin().read_line(&mut line);
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("no args");
        return;
    }
    match Expr::read(&*args[1], &ops) {
        Ok(e) => {
            println!("{}", e.to_str());
            println!("{}", e.to_hash());
            let eval = Evaluator::new(&*e);
            eval.print(Some(&*e));
        },
        Err(e) => println!("error: {}", e)
    }
}
