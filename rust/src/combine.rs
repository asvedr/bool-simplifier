use expr::*;
use std::collections::HashMap;
use rand::random;

pub struct Request {
    pub hash      : usize,
    pub var_count : usize
}

pub fn parse_hash(src : &str) -> Result<usize,String> {
    let mut p2 = 1;
    let mut res = 0;
    for c in src.chars() {
        if c == '1' {
            res = res | p2;
        } else if c == '0' {
            // nope
        } else {
            return Err(format!("incorrect symbol: {}", c))
        }
        p2 *= 2;
    }
    Ok(res)
}

pub fn find_analog(req : Request, iter_count : usize, max_depth : usize) -> Option<RExpr> {
    let mut hash_tbl  : HashMap<usize,RExpr> = HashMap::new();
    let mut index_tbl : HashMap<usize,usize> = HashMap::new();
    hash_tbl.reserve(usize::pow(2, req.var_count as u32));
    index_tbl.reserve(usize::pow(2, req.var_count as u32));
    let mut seq : Vec<RExpr> = Vec::new();
    // making start samples
    for i in 0 .. req.var_count {
        let e = Expr::var(i);
        seq.push(e.clone());
        let e = Expr::not(e);
        seq.push(e.clone());
    }
    let envs = gen_envs(req.var_count);
    let mut hash : usize = 0;
    let mut is_tautology : bool = false;
    let mut answer = None;
    macro_rules! try_this{($e:expr) => {{
        $e.hash(&envs, None, &mut hash, &mut is_tautology);
        let depth = $e.depth(Some(&hash_tbl));
        if (is_tautology || depth > max_depth) && hash != req.hash {
            continue;
        } else {
            let new;
            let mut ind : usize = 0;
            match index_tbl.get(&hash) {
                None => new = true,
                Some(ref i) => {
                    new = false;
                    ind = **i;
                    // REPLACE
                    if depth < seq[ind].depth(Some(&hash_tbl)) {
                        seq[ind] = $e.clone();
                        hash_tbl.insert(hash, $e.clone());
                    }
                }
            };
            // ADD NEW IF NEED
            if new {
                hash_tbl.insert(hash, $e.clone());
                ind = seq.len();
                index_tbl.insert(hash, ind);
                seq.push($e);
            }
            if hash == req.hash {
                answer = Some(ind);
            }
        }
    }};}
    // go search
    for _ in 0 .. iter_count {
        let len = seq.len();
        if random::<bool>() {
            // NOT
            let mut i;
            loop {
                i = random::<usize>() % len;
                if !seq[i].is_not(Some(&hash_tbl)) {
                    break
                }
            }
            let e = Expr::not(seq[i].clone());
            try_this!(e);
        } else {
            // BIN
            let a = random::<usize>() % len;
            let mut b;
            loop {
                b = random::<usize>() % len;
                if b != a {
                    break
                }
            }
            let opr = Opr::random();
            let e = Expr::bin(opr, seq[a].clone(), seq[b].clone());
            try_this!(e);
        }
    }
    match answer {
        Some(i) => return Some(seq[i].clone()),
        _ => return None
    }
}

#[inline(always)]
fn gen_envs(var_count : usize) -> Vec<Vec<bool>> {
    let mut acc = vec![Vec::new()];
    for _ in 0 .. var_count {
        let mut buf = Vec::new();
        for env in acc.iter_mut() {
            let mut alter = env.clone();
            alter.push(false);
            env.push(true);
            buf.push(alter);
        }
        for env in buf {
            acc.push(env)
        }
    }
    return acc;
}
