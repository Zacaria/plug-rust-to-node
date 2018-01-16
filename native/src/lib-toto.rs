extern crate libc;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn multiply (arg1: libc::c_int, arg2: libc::c_int) -> libc::c_int {

    arg1 * arg2
}

#[no_mangle]
pub extern "C" fn concat () -> *const c_char  {
    let s = CString::new("Hello World").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}
