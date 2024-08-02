use libswisseph_sys::AS_MAXCH;

pub type CalcPrimRet = [f64; 6];

pub const MAXCH: usize = AS_MAXCH as usize;
pub type BufferType = [i8; MAXCH];

#[derive(Debug)]
pub struct Out<T, U> {
    pub out: T,
    pub code: U,
}

impl<T, U> From<Out<T, U>> for (T, U) {
    fn from(e: Out<T, U>) -> (T, U) {
        (e.out, e.code)
    }
}

pub struct DateTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: f64,
}

impl DateTime {
    pub fn new(year: i32, month: i32, day: i32, hour: f64) -> Self {
        Self {
            year,
            month,
            day,
            hour,
        }
    }
}

