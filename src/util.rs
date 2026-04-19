use crate::*;
use std::ffi::CStr;

pub fn string_from_i8_array(b: BufferType) -> String {
    String::from_utf8(b.iter().map(|&c| c as u8).collect()).unwrap()
}

pub fn new_buffer() -> BufferType {
    new_serr_buffer()
    //[0; MAXCH]
}

pub fn new_serr_buffer() -> BufferType {
    [0; MAXCH]
}

pub fn new_ret_serr() -> (CalcPrimRet, BufferType) {
    let out: CalcPrimRet = [0.0; 6];
    let serr = crate::new_serr_buffer();
    (out, serr)
}

/// Convert a NUL-terminated C string pointer into an owned Rust `String`.
///
/// # Safety
///
/// - `s` must be a valid pointer to a NUL-terminated C string.
/// - `s` must remain valid for the duration of this call.
/// - The pointed-to bytes must be valid UTF-8.
pub unsafe fn c_chars_to_string(s: *mut ::std::os::raw::c_char) -> String {
    serr_to_string(s)
}

/// Convert Swiss Ephemeris' `serr` C buffer into an owned Rust `String`.
///
/// # Safety
///
/// - `s` must be a valid pointer to a NUL-terminated C string.
/// - `s` must remain valid for the duration of this call.
/// - The pointed-to bytes must be valid UTF-8.
pub unsafe fn serr_to_string(s: *mut ::std::os::raw::c_char) -> String {
    let rust_id = CStr::from_ptr(s);
    let rust_id = rust_id.to_owned();
    let str_slice: &str = rust_id.to_str().unwrap();
    let serr: String = str_slice.to_owned();
    serr
}
