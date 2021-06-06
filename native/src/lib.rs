use std::{
    ffi::{CStr, CString},
    mem,
    os::raw::c_char,
};

#[no_mangle]
pub extern "C" fn hello_rust(name: *const c_char) -> *const c_char {
    let name_str = to_string(name);
    return to_ptr(format!("hello {}", name_str));
}

#[no_mangle]
pub extern "C" fn sum_numbers(num1: usize, num2: usize) -> usize {
    return num1 + num2;
}

fn to_string(pointer: *const c_char) -> String {
    let slice = unsafe { CStr::from_ptr(pointer).to_bytes() };
    std::str::from_utf8(slice).unwrap().to_string()
}

fn to_ptr(string: String) -> *const c_char {
    let cs = CString::new(string.as_bytes()).unwrap();
    let ptr = cs.as_ptr();
    mem::forget(cs);
    ptr
}
