use nix::{
    errno::Errno,
    libc::{self, c_int, setlocale, LC_ALL},
};
use std::ffi::CStr;

pub fn set_all_locale_by_env() {
    unsafe {
        setlocale(LC_ALL, [0].as_ptr());
    }
}

pub fn strerror(errno: Errno) -> String {
    let c_str = unsafe {
        let p = libc::strerror(errno as c_int);
        CStr::from_ptr(p)
    };
    c_str.to_string_lossy().to_string()
}
