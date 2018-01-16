extern crate libc;

#[no_mangle]

pub extern "C" fn multiply (arg1: libc::c_int, arg2: libc::c_int) -> libc::c_int {

    arg1 * arg2
}

#[no_mangle]

pub extern "C" fn concat (arg1: libc::c_int, arg2: libc::c_int) -> libc::c_int  {

    arg1 * arg2
}
