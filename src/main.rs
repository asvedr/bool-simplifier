#![allow(dead_code)]

extern crate rand;

mod lexer;
#[macro_use]
mod reserr;
#[macro_use]
mod expr;
mod eval;
mod analyse;
mod normal_form;

use expr::*;
use eval::*;
use std::env;
use normal_form::*;

fn main() {
    let ops = vec![
        Op::new('&', "&", Box::new(|a : bool, b : bool|{a && b})),
        Op::new('|', "|", Box::new(|a : bool, b : bool|{a || b})),
        Op::new('=', "=", Box::new(|a : bool, b : bool|{a == b})),
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
            /*println!("{}", e.to_str());
            println!("{}", e.to_hash());
            let eval = Evaluator::new(&*e);
            eval.print(Some(&*e));
            */
            println!("{}", e.to_str());
            Diapason::from_expr(&*e).print(Some(&*e));
            let t = analyse::is_tautology(&*e, None);
            let (_,unused) = analyse::used_unused(&*e);
            println!("tautology: {}", t);
            println!("unused: {:?}", unused);
            let nf = NF::from_expr(&*e);
            println!("NF: {}", nf.to_str());
        },
        Err(e) => println!("error: {}", e)
    }
}
