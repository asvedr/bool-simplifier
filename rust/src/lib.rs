extern crate rand;
//extern crate libc;

mod combine;
mod expr;

use std::mem;
use std::slice;
use std::ptr;
use std::os::raw::{c_int, c_char, c_void};
use std::ffi;

struct Container {
    data : expr::RExpr,
    view : ffi::CString
}

#[no_mangle]
pub fn get_expr(var_count : c_int, table : *const c_int) -> *const c_void {
    let len = usize::pow(2, var_count as u32);
    let vec : &[c_int] = unsafe { slice::from_raw_parts(table, len) };
    let mut p2 = 1;
    let mut hash = 0;
    for i in 0 .. len {
        hash = hash | (p2 * (vec[i] as usize));
        p2 *= 2;
    }
    let vc = var_count as usize;
    let req = combine::Request{var_count : vc, hash : hash};
    let res = combine::find_analog(req, 1000, ((vc as f32) * 2.5) as usize);
    unsafe {
        match res {
            Some(e) => {
                let view = ffi::CString::new(e.show(None)).unwrap();
                let ptr = Box::new(Container {
                    data : e,
                    view : view
                });
                mem::transmute(ptr)
            },
            _ => ptr::null::<c_void> ()
        }
    }
}

#[no_mangle]
pub fn show_expr(data : *const c_void) -> *const c_char {
    unsafe {
        let cont : &Container = mem::transmute(data);
        mem::transmute(cont.view.as_ptr())
    }
}

#[no_mangle]
pub fn rem_expr(ptr : *const c_void) {
    let _ : Box<Container> = unsafe { mem::transmute(ptr) };
}
