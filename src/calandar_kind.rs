use libswisseph_sys::{SE_JUL_CAL, SE_GREG_CAL};

#[derive(Debug)]
pub enum CalandarKind {
    Julian      = SE_JUL_CAL as isize,
    Gregorian   = SE_GREG_CAL as isize,
}

