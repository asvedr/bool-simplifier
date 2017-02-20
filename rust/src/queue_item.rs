use expr::*;
use std::rc::Rc;
use std::cmp::Ordering;

#[derive(Eq)]
pub struct Item {
    pub value    : RExpr,
    pub capacity : f32,
    pub hash     : usize,
    pub depth    : usize,
    pub age      : usize
}

impl Item {
    pub fn new(val : RExpr, hash : usize, d_count : f64, d_depth : f64) -> Item {
        let depth = expr.depth();
        Item {
            value    : val,
            capacity : d_depth * (depth as f32) + d_count * (expr.var_count() as f32),
            depth    : depth,
            hash     : hash,
            age      : 0
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other : &Item) -> Ordering {
        self.capacity.cmp(other.capacity)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other : &Item) -> Option<Ordering> {
        self.capacity.partial_cmp(other.capacity)
    }
}

impl PartialEq for Item {
    fn eq(&self, other : &Item) -> bool {
        self.hash == other.hash
    }
}
