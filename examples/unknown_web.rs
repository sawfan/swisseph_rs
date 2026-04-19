//! wasm32-unknown-unknown Swiss Ephemeris demo (no WASI), using swisseph's higher-level APIs.
//!
//! This mirrors the `libswisseph-sys` `unknown_web` example, but calls into the
//! higher-level abstractions provided by the `swisseph` crate.
//!
//! The JS host must provide ephemeris assets by calling the exported functions:
//! - `swisseph_add_ephe_file()` for each `.se1` file
//! - then `swisseph_set_ephe_path_utf8("ephe")`
//! - then `swisseph_calc_ut()`

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void, CStr, CString};

use swisseph::swe;
use swisseph::{swe2, Body, CalandarKind, Seflg};

thread_local! {
    static LAST_ERROR: RefCell<[u8; 512]> = const { RefCell::new([0; 512]) };

    #[allow(clippy::missing_const_for_thread_local)]
    static EPHE_FILES: RefCell<HashMap<String, Vec<u8>>> = RefCell::new(HashMap::new());
}

fn set_last_error(msg: &str) {
    LAST_ERROR.with(|buf| {
        let mut buf = buf.borrow_mut();
        buf.fill(0);

        let bytes = msg.as_bytes();
        let n = bytes.len().min(buf.len().saturating_sub(1));
        buf[..n].copy_from_slice(&bytes[..n]);
    });
}

#[no_mangle]
pub extern "C" fn swisseph_last_error_ptr() -> *const u8 {
    LAST_ERROR.with(|buf| buf.borrow().as_ptr())
}

/// Entry point (not used in this embedding).
fn main() {}

#[no_mangle]
pub extern "C" fn swisseph_alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Deallocate a buffer previously allocated with `swisseph_alloc`.
///
/// # Safety
///
/// - `ptr` must have been returned by `swisseph_alloc(len)`.
/// - `len` must be the same length that was passed to `swisseph_alloc`.
#[no_mangle]
pub unsafe extern "C" fn swisseph_dealloc(ptr: *mut u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }

    drop(Vec::<u8>::from_raw_parts(ptr, 0, len));
}

#[no_mangle]
pub extern "C" fn swisseph_alloc_f64(len: usize) -> *mut f64 {
    let mut buf = Vec::<f64>::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Deallocate a buffer previously allocated with `swisseph_alloc_f64`.
///
/// # Safety
///
/// - `ptr` must have been returned by `swisseph_alloc_f64(len)`.
/// - `len` must be the same length that was passed to `swisseph_alloc_f64`.
#[no_mangle]
pub unsafe extern "C" fn swisseph_dealloc_f64(ptr: *mut f64, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }

    drop(Vec::<f64>::from_raw_parts(ptr, 0, len));
}

/// Add (or replace) an ephemeris file in the in-wasm VFS.
///
/// - `name_ptr`/`name_len`: UTF-8 file name, e.g. "seas_18.se1".
/// - `data_ptr`/`data_len`: file bytes.
///
/// Returns 0 on success, -1 on error.
///
/// # Safety
///
/// - `name_ptr` must point to a valid readable buffer of length `name_len`.
/// - `data_ptr` must point to a valid readable buffer of length `data_len`.
#[no_mangle]
pub unsafe extern "C" fn swisseph_add_ephe_file(
    name_ptr: *const u8,
    name_len: usize,
    data_ptr: *const u8,
    data_len: usize,
) -> i32 {
    if name_ptr.is_null() || data_ptr.is_null() {
        set_last_error("null pointer");
        return -1;
    }

    let name_bytes = unsafe { std::slice::from_raw_parts(name_ptr, name_len) };
    let name = match std::str::from_utf8(name_bytes) {
        Ok(s) => s.to_string(),
        Err(e) => {
            set_last_error(&format!("invalid utf-8 file name: {e}"));
            return -1;
        }
    };

    let data = unsafe { std::slice::from_raw_parts(data_ptr, data_len) }.to_vec();

    EPHE_FILES.with(|m| {
        m.borrow_mut().insert(name, data);
    });

    0
}

#[repr(C)]
struct VfsHandle {
    /// Stored as a heap-allocated C string (so we can carry it through `void*`).
    name: *mut c_char,
    /// Current read cursor for stdio-compat helpers (fread/fgets/fseek).
    pos: usize,
}

unsafe extern "C" fn vfs_open(
    _ifno: c_int,
    fname: *const c_char,
    _ephepath: *const c_char,
    _serr: *mut c_char,
) -> *mut c_void {
    if fname.is_null() {
        return std::ptr::null_mut();
    }

    let name = unsafe { CStr::from_ptr(fname) }
        .to_string_lossy()
        .to_string();

    let exists = EPHE_FILES.with(|m| m.borrow().contains_key(&name));
    if !exists {
        return std::ptr::null_mut();
    }

    let cname = match CString::new(name) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let h = Box::new(VfsHandle {
        name: cname.into_raw(),
        pos: 0,
    });

    Box::into_raw(h) as *mut c_void
}

unsafe extern "C" fn vfs_read_at(
    h: *mut c_void,
    dst: *mut c_void,
    size: usize,
    count: usize,
    offset: i32,
    _serr: *mut c_char,
) -> usize {
    if h.is_null() || dst.is_null() {
        return 0;
    }

    if size == 0 || count == 0 {
        return 0;
    }

    let h = unsafe { &*(h as *const VfsHandle) };
    if h.name.is_null() {
        return 0;
    }

    let name = unsafe { CStr::from_ptr(h.name) }
        .to_string_lossy()
        .to_string();
    let buf_opt = EPHE_FILES.with(|m| m.borrow().get(&name).cloned());
    let buf = match buf_opt {
        Some(b) => b,
        None => return 0,
    };

    let offset = offset as isize;
    if offset < 0 {
        return 0;
    }

    let offset = offset as usize;
    if offset > buf.len() {
        return 0;
    }

    let available = buf.len() - offset;
    let want_bytes = size.saturating_mul(count);
    let read_bytes = available.min(want_bytes);

    unsafe {
        std::ptr::copy_nonoverlapping(buf.as_ptr().add(offset), dst as *mut u8, read_bytes);
    }

    // fread-like semantics: return number of whole items read.
    read_bytes / size
}

unsafe extern "C" fn vfs_close(h: *mut c_void) {
    if h.is_null() {
        return;
    }

    let h = unsafe { Box::from_raw(h as *mut VfsHandle) };
    if !h.name.is_null() {
        unsafe {
            drop(CString::from_raw(h.name));
        }
    }
}

#[no_mangle]
pub extern "C" fn swisseph_vfs_init() -> i32 {
    // Safety: We register function pointers that remain valid for the lifetime
    // of the module.
    let api = swe::swe_vfs_api {
        open: Some(vfs_open),
        read_at: Some(vfs_read_at),
        close: Some(vfs_close),
    };

    // Leak the API struct so the pointer remains stable for the duration of the program.
    let api_ptr: *const swe::swe_vfs_api = Box::leak(Box::new(api));

    // Safety: `api_ptr` is valid and points to a stable (leaked) VFS API struct for the
    // remainder of the module's lifetime.
    unsafe {
        swe::set_vfs_api(api_ptr);
    }

    0
}

/// Set Swiss Ephemeris ephemeris path.
///
/// Returns 0 on success, -1 on error.
///
/// # Safety
///
/// - `ptr` must point to a valid readable buffer of length `len`.
#[no_mangle]
pub unsafe extern "C" fn swisseph_set_ephe_path_utf8(ptr: *const u8, len: usize) -> i32 {
    if ptr.is_null() {
        set_last_error("null pointer");
        return -1;
    }

    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    let path = match std::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(e) => {
            set_last_error(&format!("invalid utf-8: {e}"));
            return -1;
        }
    };

    // Use the higher-level wrapper from swisseph.
    if path.contains('\0') {
        set_last_error("path contained NUL byte");
        return -1;
    }

    swe::set_ephe_path(path);

    0
}

#[no_mangle]
pub extern "C" fn swisseph_julday_ut(year: i32, month: i32, day: i32, hour_ut: f64) -> f64 {
    swe::julday(year, month, day, hour_ut, CalandarKind::Gregorian as u32)
}

#[no_mangle]
pub unsafe extern "C" fn swisseph_calc_ut(jd_ut: f64, ipl: i32, _iflag: i32, out_ptr: *mut f64) -> i32 {
    if out_ptr.is_null() {
        set_last_error("out_ptr is null");
        return -1;
    }

    let body = match Body::from_repr(ipl as isize) {
        Some(b) => b,
        None => {
            set_last_error(&format!("unknown body id: {ipl}"));
            return -1;
        }
    };

    // For this demo we intentionally keep the flag set minimal and stable.
    // The JS runner still passes flags for compatibility with the lower-level
    // demo, but we compute using the swisseph high-level API.
    let flag = Seflg::SWIEPH | Seflg::SPEED;

    let pos = match swe2::calc_ut2_ecliptic(jd_ut, body, flag) {
        Ok(p) => p,
        Err(e) => {
            set_last_error(&e);
            return -1;
        }
    };

    let out = unsafe { std::slice::from_raw_parts_mut(out_ptr, 6) };
    out[0] = pos.longitude;
    out[1] = pos.latitude;
    out[2] = pos.distance_in_au;
    out[3] = pos.speed_in_longitude;
    out[4] = pos.speed_in_latitude;
    out[5] = pos.speed_in_distance;

    0
}

// Convenience exports for common bodies (avoid JS needing constants).
#[no_mangle]
pub extern "C" fn swisseph_se_sun() -> i32 {
    Body::Sun as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_moon() -> i32 {
    Body::Moon as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_mercury() -> i32 {
    Body::Mercury as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_venus() -> i32 {
    Body::Venus as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_mars() -> i32 {
    Body::Mars as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_jupiter() -> i32 {
    Body::Jupiter as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_saturn() -> i32 {
    Body::Saturn as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_uranus() -> i32 {
    Body::Uranus as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_neptune() -> i32 {
    Body::Neptune as i32
}

#[no_mangle]
pub extern "C" fn swisseph_se_pluto() -> i32 {
    Body::Pluto as i32
}

#[no_mangle]
pub extern "C" fn swisseph_seflg_swieph() -> i32 {
    Seflg::SWIEPH.bits() as i32
}

#[no_mangle]
pub extern "C" fn swisseph_seflg_speed() -> i32 {
    Seflg::SPEED.bits() as i32
}

// -----------------------------------------------------------------------------
// wasm32-unknown-unknown: minimal libc/stdio shims
// -----------------------------------------------------------------------------
//
// When compiling the Swiss Ephemeris C sources for `wasm32-unknown-unknown`,
// Clang will otherwise leave common libc symbols unresolved, which become
// WebAssembly imports ("env.*").
//
// For the browser demo, we provide a tiny subset of libc and stdio directly
// inside the wasm module.
//
// Notes:
// - This is intentionally minimal and only targets the needs of the demo.
// - These symbols are only provided for wasm32-unknown-unknown to avoid
//   colliding with the host libc on native builds.

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm_shims {
    use super::{set_last_error, VfsHandle, EPHE_FILES};
    use core::ffi::{c_char, c_int, c_void};
    use core::ptr;
    use std::alloc::{alloc, dealloc, realloc as std_realloc, Layout};
    use std::ffi::CStr;

    const HEADER_SIZE: usize = core::mem::size_of::<usize>();
    const HEADER_ALIGN: usize = core::mem::align_of::<usize>();

    unsafe fn c_strlen(mut p: *const c_char) -> usize {
        if p.is_null() {
            return 0;
        }
        let mut n = 0usize;
        while unsafe { *p } != 0 {
            n += 1;
            p = unsafe { p.add(1) };
        }
        n
    }

    unsafe fn c_str_bytes(p: *const c_char) -> &'static [u8] {
        if p.is_null() {
            return &[];
        }
        let len = unsafe { c_strlen(p) };
        unsafe { core::slice::from_raw_parts(p as *const u8, len) }
    }

    unsafe fn c_str_to_string(p: *const c_char) -> String {
        if p.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(p) }.to_string_lossy().to_string()
    }

    unsafe fn write_bytes_with_nul(dst: *mut c_char, bytes: &[u8]) -> c_int {
        if dst.is_null() {
            return 0;
        }
        let mut i = 0usize;
        for &b in bytes {
            unsafe {
                *dst.add(i) = b as c_char;
            }
            i += 1;
        }
        unsafe {
            *dst.add(i) = 0;
        }
        i as c_int
    }

    // ----------------------------- memory -----------------------------------

    #[no_mangle]
    pub unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
        if size == 0 {
            return ptr::null_mut();
        }

        let total = size.saturating_add(HEADER_SIZE);
        let layout = match Layout::from_size_align(total, HEADER_ALIGN) {
            Ok(l) => l,
            Err(_) => return ptr::null_mut(),
        };

        let base = unsafe { alloc(layout) };
        if base.is_null() {
            return ptr::null_mut();
        }

        unsafe {
            *(base as *mut usize) = size;
            base.add(HEADER_SIZE) as *mut c_void
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn calloc(nmemb: usize, size: usize) -> *mut c_void {
        let bytes = match nmemb.checked_mul(size) {
            Some(b) => b,
            None => return ptr::null_mut(),
        };
        let p = unsafe { malloc(bytes) };
        if !p.is_null() {
            unsafe { ptr::write_bytes(p, 0, bytes) };
        }
        p
    }

    #[no_mangle]
    pub unsafe extern "C" fn free(ptr_: *mut c_void) {
        if ptr_.is_null() {
            return;
        }
        let base = unsafe { (ptr_ as *mut u8).sub(HEADER_SIZE) };
        let size = unsafe { *(base as *const usize) };
        let total = size.saturating_add(HEADER_SIZE);
        if let Ok(layout) = Layout::from_size_align(total, HEADER_ALIGN) {
            unsafe { dealloc(base, layout) };
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn realloc(ptr_: *mut c_void, new_size: usize) -> *mut c_void {
        if ptr_.is_null() {
            return unsafe { malloc(new_size) };
        }
        let base = unsafe { (ptr_ as *mut u8).sub(HEADER_SIZE) };
        let old_size = unsafe { *(base as *const usize) };
        let old_total = old_size.saturating_add(HEADER_SIZE);
        let new_total = new_size.saturating_add(HEADER_SIZE);

        let old_layout = match Layout::from_size_align(old_total, HEADER_ALIGN) {
            Ok(l) => l,
            Err(_) => return ptr::null_mut(),
        };

        let new_base = unsafe { std_realloc(base, old_layout, new_total) };
        if new_base.is_null() {
            return ptr::null_mut();
        }
        unsafe {
            *(new_base as *mut usize) = new_size;
            new_base.add(HEADER_SIZE) as *mut c_void
        }
    }

    // ----------------------------- env --------------------------------------

    #[no_mangle]
    pub unsafe extern "C" fn getenv(_name: *const c_char) -> *mut c_char {
        // No environment in this embedding.
        ptr::null_mut()
    }

    // ----------------------------- parsing ----------------------------------

    #[no_mangle]
    pub unsafe extern "C" fn tolower(c: c_int) -> c_int {
        if (b'A' as c_int..=b'Z' as c_int).contains(&c) {
            c + 32
        } else {
            c
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn isdigit(c: c_int) -> c_int {
        if (b'0' as c_int..=b'9' as c_int).contains(&c) {
            1
        } else {
            0
        }
    }

    fn parse_i64(s: &str) -> i64 {
        let s = s.trim_start();
        let mut chars = s.chars();
        let mut sign = 1i64;
        if let Some(c) = chars.clone().next() {
            if c == '-' {
                sign = -1;
                chars.next();
            } else if c == '+' {
                chars.next();
            }
        }
        let mut acc = 0i64;
        for c in chars {
            if !c.is_ascii_digit() {
                break;
            }
            acc = acc
                .saturating_mul(10)
                .saturating_add((c as u8 - b'0') as i64);
        }
        acc * sign
    }

    #[no_mangle]
    pub unsafe extern "C" fn atoi(s: *const c_char) -> c_int {
        parse_i64(&unsafe { c_str_to_string(s) }) as c_int
    }

    #[no_mangle]
    pub unsafe extern "C" fn atol(s: *const c_char) -> c_int {
        parse_i64(&unsafe { c_str_to_string(s) }) as c_int
    }

    #[no_mangle]
    pub unsafe extern "C" fn atof(s: *const c_char) -> f64 {
        let st = unsafe { c_str_to_string(s) };
        st.trim().parse::<f64>().unwrap_or(0.0)
    }

    // ----------------------------- strings ----------------------------------

    #[no_mangle]
    pub unsafe extern "C" fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char {
        if dst.is_null() || src.is_null() {
            return dst;
        }
        let mut i = 0usize;
        loop {
            let b = unsafe { *src.add(i) };
            unsafe { *dst.add(i) = b };
            i += 1;
            if b == 0 {
                break;
            }
        }
        dst
    }

    #[no_mangle]
    pub unsafe extern "C" fn strncpy(
        dst: *mut c_char,
        src: *const c_char,
        n: usize,
    ) -> *mut c_char {
        if dst.is_null() || src.is_null() {
            return dst;
        }
        let mut i = 0usize;
        while i < n {
            let b = unsafe { *src.add(i) };
            unsafe { *dst.add(i) = b };
            i += 1;
            if b == 0 {
                break;
            }
        }
        while i < n {
            unsafe { *dst.add(i) = 0 };
            i += 1;
        }
        dst
    }

    #[no_mangle]
    pub unsafe extern "C" fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char {
        if dst.is_null() || src.is_null() {
            return dst;
        }
        let mut d = dst;
        while unsafe { *d } != 0 {
            d = unsafe { d.add(1) };
        }
        let mut s = src;
        loop {
            let b = unsafe { *s };
            unsafe { *d = b };
            if b == 0 {
                break;
            }
            d = unsafe { d.add(1) };
            s = unsafe { s.add(1) };
        }
        dst
    }

    #[no_mangle]
    pub unsafe extern "C" fn strcmp(a: *const c_char, b: *const c_char) -> c_int {
        let mut i = 0usize;
        loop {
            let ca = unsafe { *a.add(i) as u8 };
            let cb = unsafe { *b.add(i) as u8 };
            if ca != cb {
                return ca as c_int - cb as c_int;
            }
            if ca == 0 {
                return 0;
            }
            i += 1;
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int {
        let mut i = 0usize;
        while i < n {
            let ca = unsafe { *a.add(i) as u8 };
            let cb = unsafe { *b.add(i) as u8 };
            if ca != cb {
                return ca as c_int - cb as c_int;
            }
            if ca == 0 {
                return 0;
            }
            i += 1;
        }
        0
    }

    #[no_mangle]
    pub unsafe extern "C" fn strchr(s: *const c_char, c: c_int) -> *mut c_char {
        if s.is_null() {
            return ptr::null_mut();
        }
        let mut p = s;
        let c = c as u8;
        loop {
            let b = unsafe { *p as u8 };
            if b == c {
                return p as *mut c_char;
            }
            if b == 0 {
                return ptr::null_mut();
            }
            p = unsafe { p.add(1) };
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn strrchr(s: *const c_char, c: c_int) -> *mut c_char {
        if s.is_null() {
            return ptr::null_mut();
        }
        let c = c as u8;
        let mut last: *const c_char = ptr::null();
        let mut p = s;
        loop {
            let b = unsafe { *p as u8 };
            if b == c {
                last = p;
            }
            if b == 0 {
                break;
            }
            p = unsafe { p.add(1) };
        }
        last as *mut c_char
    }

    #[no_mangle]
    pub unsafe extern "C" fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char {
        if haystack.is_null() || needle.is_null() {
            return ptr::null_mut();
        }
        let h = unsafe { c_str_bytes(haystack) };
        let n = unsafe { c_str_bytes(needle) };
        if n.is_empty() {
            return haystack as *mut c_char;
        }
        if let Some(pos) = h.windows(n.len()).position(|w| w == n) {
            unsafe { haystack.add(pos) as *mut c_char }
        } else {
            ptr::null_mut()
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char {
        if s.is_null() || accept.is_null() {
            return ptr::null_mut();
        }
        let acc = unsafe { c_str_bytes(accept) };
        let mut p = s;
        loop {
            let b = unsafe { *p as u8 };
            if b == 0 {
                return ptr::null_mut();
            }
            if acc.contains(&b) {
                return p as *mut c_char;
            }
            p = unsafe { p.add(1) };
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn strdup(s: *const c_char) -> *mut c_char {
        if s.is_null() {
            return ptr::null_mut();
        }
        let len = unsafe { c_strlen(s) };
        let out = unsafe { malloc(len + 1) } as *mut c_char;
        if out.is_null() {
            return ptr::null_mut();
        }
        unsafe {
            ptr::copy_nonoverlapping(s, out, len + 1);
        }
        out
    }

    /// Minimal sprintf implementation for the wasm demo.
    ///
    /// The Swiss Ephemeris code paths hit by this demo typically use `sprintf`
    /// with at most one extra argument (often a `%s`). We implement only a very
    /// small subset.
    #[no_mangle]
    pub unsafe extern "C" fn sprintf(
        dst: *mut c_char,
        fmt: *const c_char,
        arg1: *const c_char,
    ) -> c_int {
        if dst.is_null() || fmt.is_null() {
            return 0;
        }

        let fmt_s = unsafe { c_str_to_string(fmt) };
        let arg1_s = if arg1.is_null() {
            String::new()
        } else {
            unsafe { c_str_to_string(arg1) }
        };

        let out = if let Some(idx) = fmt_s.find("%s") {
            let mut s = String::new();
            s.push_str(&fmt_s[..idx]);
            s.push_str(&arg1_s);
            s.push_str(&fmt_s[idx + 2..]);
            s
        } else {
            fmt_s
        };

        unsafe { write_bytes_with_nul(dst, out.as_bytes()) }
    }

    // ----------------------------- stdio ------------------------------------

    const SEEK_SET: c_int = 0;
    const SEEK_CUR: c_int = 1;
    const SEEK_END: c_int = 2;

    unsafe fn with_file_bytes<T>(h: &VfsHandle, f: impl FnOnce(&[u8]) -> T) -> Option<T> {
        if h.name.is_null() {
            return None;
        }
        let name = unsafe { CStr::from_ptr(h.name) }
            .to_string_lossy()
            .to_string();
        EPHE_FILES.with(|m| m.borrow().get(&name).map(|v| f(v.as_slice())))
    }

    #[no_mangle]
    pub unsafe extern "C" fn ftell(stream: *mut c_void) -> c_int {
        if stream.is_null() {
            return -1;
        }
        let h = unsafe { &*(stream as *const VfsHandle) };
        h.pos as c_int
    }

    #[no_mangle]
    pub unsafe extern "C" fn fseek(stream: *mut c_void, offset: c_int, whence: c_int) -> c_int {
        if stream.is_null() {
            return -1;
        }
        let h = unsafe { &mut *(stream as *mut VfsHandle) };

        let len = unsafe { with_file_bytes(h, |b| b.len()).unwrap_or(0usize) };

        let base: isize = match whence {
            SEEK_SET => 0,
            SEEK_CUR => h.pos as isize,
            SEEK_END => len as isize,
            _ => return -1,
        };

        let new_pos = base.saturating_add(offset as isize);
        if new_pos < 0 {
            return -1;
        }
        let new_pos = new_pos as usize;
        h.pos = new_pos.min(len);
        0
    }

    #[no_mangle]
    pub unsafe extern "C" fn rewind(stream: *mut c_void) {
        if stream.is_null() {
            return;
        }
        let h = unsafe { &mut *(stream as *mut VfsHandle) };
        h.pos = 0;
    }

    #[no_mangle]
    pub unsafe extern "C" fn fread(
        ptr_: *mut c_void,
        size: usize,
        count: usize,
        stream: *mut c_void,
    ) -> usize {
        if ptr_.is_null() || stream.is_null() || size == 0 || count == 0 {
            return 0;
        }
        let h = unsafe { &mut *(stream as *mut VfsHandle) };

        let want = size.saturating_mul(count);
        let start_pos = h.pos;
        let Some(read_bytes) = (unsafe {
            with_file_bytes(h, |b| {
                if start_pos >= b.len() {
                    return 0usize;
                }
                let avail = b.len() - start_pos;
                let n = avail.min(want);
                ptr::copy_nonoverlapping(b.as_ptr().add(start_pos), ptr_ as *mut u8, n);
                n
            })
        }) else {
            return 0;
        };

        h.pos = start_pos.saturating_add(read_bytes);

        read_bytes / size
    }

    #[no_mangle]
    pub unsafe extern "C" fn fgets(s: *mut c_char, n: c_int, stream: *mut c_void) -> *mut c_char {
        if s.is_null() || stream.is_null() || n <= 0 {
            return ptr::null_mut();
        }

        let h = unsafe { &mut *(stream as *mut VfsHandle) };

        let start_pos = h.pos;

        let Some(got) = (unsafe {
            with_file_bytes(h, |b| {
                if start_pos >= b.len() {
                    return 0usize;
                }

                let max = (n as usize).saturating_sub(1);
                let mut i = 0usize;
                while i < max && start_pos + i < b.len() {
                    let ch = b[start_pos + i];
                    i += 1;
                    if ch == b'\n' {
                        break;
                    }
                }

                ptr::copy_nonoverlapping(b.as_ptr().add(start_pos), s as *mut u8, i);
                *(s.add(i) as *mut u8) = 0;

                i
            })
        }) else {
            return ptr::null_mut();
        };

        h.pos = start_pos.saturating_add(got);

        if got == 0 {
            ptr::null_mut()
        } else {
            s
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn fclose(stream: *mut c_void) -> c_int {
        if stream.is_null() {
            return 0;
        }

        // This stream is actually our VFS handle (see vfs_open in the parent module).
        unsafe { super::vfs_close(stream) };
        0
    }

    // ----------------------------- JPL stubs --------------------------------
    //
    // We intentionally don't ship JPL ephemeris support for the wasm32-unknown-unknown
    // web demo. Provide stubs so references don't become wasm imports.

    #[no_mangle]
    pub unsafe extern "C" fn swi_open_jpl_file(
        _ss: *mut f64,
        _fname: *mut c_char,
        _fpath: *mut c_char,
        serr: *mut c_char,
    ) -> c_int {
        if !serr.is_null() {
            let _ = unsafe {
                super::wasm_shims::write_bytes_with_nul(serr, b"JPL ephemeris not supported\0")
            };
        }
        set_last_error("JPL ephemeris not supported in wasm32-unknown-unknown build");
        -1
    }

    #[no_mangle]
    pub unsafe extern "C" fn swi_close_jpl_file() {}

    #[no_mangle]
    pub unsafe extern "C" fn swi_get_jpl_denum() -> i32 {
        0
    }

    #[no_mangle]
    pub unsafe extern "C" fn swi_pleph(
        _et: f64,
        _ntarg: c_int,
        _ncent: c_int,
        _rrd: *mut f64,
        _serr: *mut c_char,
    ) -> c_int {
        -1
    }
}
