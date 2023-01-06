use std::ffi::CString;

pub fn to_cstring(src: impl Into<Vec<u8>>) -> CString {
    CString::new(src).expect("source bytes must not contain nul bytes")
}
