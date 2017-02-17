extern crate rand;
extern crate libc;

mod combine;
mod expr;

#[no_mangle]
pub fn get_expr(var_count : libc::c_int, table : *const libc::c_int) -> *const libc::c_void {

}

#[no_mangle]
pub fn show_expr(*const libc::c_void) -> *const libc::c_char {
}

#[no_mangle]
pub fn rem_expr(*const libc::c_void) {
}
