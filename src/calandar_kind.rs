use libswisseph_sys::{SE_GREG_CAL, SE_JUL_CAL};

#[derive(Debug)]
#[rustfmt::skip]
pub enum CalandarKind {
    Julian      = SE_JUL_CAL as isize,
    Gregorian   = SE_GREG_CAL as isize,
}
