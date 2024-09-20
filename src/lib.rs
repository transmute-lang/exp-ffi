
#![deny(improper_ctypes_definitions)]

use std::ffi::CString;
use std::os::raw::c_char;
use reqwest::blocking::Client;

#[no_mangle]
pub extern "C" fn query() -> *mut c_char {
    let client = Client::new();
    let req = client.get("https://reqres.in/api/users/2").build().unwrap();

    let res = client.execute(req).unwrap().text().unwrap();
    let cstr = CString::new(res)
        .expect("Error: CString::new()")
        .into_raw();
    cstr
}

#[no_mangle]
pub extern "C" fn query_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}
