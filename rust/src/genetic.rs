use queue_item::Item;
use std::collections::{BinaryHeap, HashMap};
use rand::distributions::*;

struct Genesys {
    generation   : BinaryHeap<Item>,
    dep_map      : HashSet<usize,usize>, // key - bool-map, value - depth for this bool-map
    prims        : Vec<RExpr>,
    new          : Vec<(RExpr,usize)>,
    max_age      : usize,
    max_depth    : usize,
    child_in_gen : usize,
    var_count    : usize,
    envs         : Vec<Vec<bool>>
}

macro_rules! get_rand_item{($ind:expr, $heap:expr) => {{
    let mut i = 0;
    for val in $heap.iter() {
        if i == $ind {
            return val;
        }
        i += 1;
    }
    unreachable!()
}};}

// XXX try add os.rng
fn get_rand_exp(heap : &BinaryHeap<Item>) -> &Item {
    let max_val_f = heap.count() as f64;
    loop {
        let Exp1(d) = rand::random();
        let v = d * max_val_f * 0.2;
        if v < max_val_f {
            let ind = v as usize;
            get_rand_item!(ind, heap)
        }
    }
}

fn get_rand_norm(heap : &BinaryHeap<Item>) -> &Item {
    let max_val_f = heap.count() as f64;
    loop {
        let StandardNormal(d) = rand::random();
        let v = ((d - 0.5) * 0.33) * max_val_f;
        if v >= 0.0 && v < max_val_f {
            let ind = v as usize;
            get_rand_item!(ind, heap)
        }
    }
}

impl Genesys {
    pub fn new(var_count : usize, max_age : usize, max_depth : usize, chldng : usize, target_hash : usize) -> Genesys {
        let mut prims = Vec::new();
        for v in var_count {
            let e = Expr::var(v);
            prims.push(e.clone());
            prims.push(Expr::not(e));
        }
        let envs = gen_envs(var_count);
        Genesys {
            generation   : BinaryHeap::new(),
            prims        : prims,
            var_count    : var_count,
            new          : Vec::new(),
            max_age      : max_age,
            max_depth    : depth,
            child_in_gen : chldng,
            target_hash  : target_hash,
            envs         : envs
            d_count      : 
            d_depth      : 
        }
    }
    #[inline(always)]
    pub fn gen_child(&self) -> Option<Item> {
        let pcnt  = self.prims.len() as f32;
        let total = pcnt + self.generation() as f32;
        macro_rules! parent {() => {
            if (rand::random::<f32>() * total) < pcnt {
                self.prims[rand::random::<usize>() % self.prims.len()].clone()
            } else {
                get_rand_norm(&self.generation).value.clone()
            }
        };}
        let mam : RExpr = parent!();
        let dad : RExpr = parent!();
        let chld = Expr::bin(Opr::random(), mam, dad);
        let hash = expr.eval(&self.envs);
        if hash == self.target_hash {
            let item = Item::new(chld, hash, 0.0, 0.0);
            return Some(item)
        }
        if item.depth > self.max_depth {
            return None;
        }
        let item = Item::new(chld, hash, d_count, d_depth);
        return Some(item);
    }
    pub fn new_generation(&mut self) -> Option<RExpr> {
        self.new.clear();
        'outer : for _ in 0 .. self.child_in_gen {
            'inner : loop {
                match self.gen_child() {
                    Some(item) => {
                        if item.hash == self.target_hash {
                            return Some(item.value)
                        } else {
                            self.new.push(item.value.clone());
                            self.generation.push(item);
                            continue 'outer;
                        }
                    },
                    _ => continue 'inner
                }
            }
        }
        for item in self.generation.iter_mut() {
            item.age += 1;
        }
        None
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
