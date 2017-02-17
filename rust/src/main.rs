extern crate rand;

mod expr;
mod combine;

use std::env;
use std::str::FromStr;

fn main() {
    let args : Vec<String> = env::args().collect();
    let var_count = match usize::from_str(&*args[1]) {
        Ok(a) => a,
        _ => {
            println!("incorrect args");
            return;
        }
    };
    let hash = match combine::parse_hash(&*args[2]) {
        Ok(a) => a,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let depth = ((var_count as f32) * 2.5) as usize;
    let req = combine::Request{var_count : var_count, hash : hash};
    match combine::find_analog(req, 1000, depth) {
        Some(e) => println!("{}", e.show(None)),
        _ => println!("none")
    }
}
