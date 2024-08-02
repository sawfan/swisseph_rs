use libswisseph_sys::*;
use bitmask_enum::*;

#[bitmask(u32)]
pub enum SplitDegKind {
    RoundSec    = SE_SPLIT_DEG_ROUND_SEC,
    RoundMin    = SE_SPLIT_DEG_ROUND_MIN,
    RoundDeg    = SE_SPLIT_DEG_ROUND_DEG,
    Zodiacal    = SE_SPLIT_DEG_ZODIACAL,
    Nakshatra   = SE_SPLIT_DEG_NAKSHATRA,
    KeepSign    = SE_SPLIT_DEG_KEEP_SIGN,
    KeepDeg     = SE_SPLIT_DEG_KEEP_DEG,
}

#[derive(Debug, Clone)]
pub struct SplitDegree {
    pub ideg: i32,
    pub imin: i32,
    pub isec: i32,
    pub dsecfr: f64,
    pub isgn: i32,
}

impl SplitDegree {
    pub fn from_tuple(t: (i32, i32, i32, f64, i32)) -> SplitDegree {
        let (ideg, imin, isec, dsecfr, isgn) = t;
        Self {
            ideg,
            imin,
            isec,
            dsecfr,
            isgn,
        }
    }

    pub fn to_text(&self) -> String {
        //′ and ″ (Unicode U+2032 and U+2033,
        format!("{}°{}′{}″", self.ideg, self.imin, self.isec)
    }
}

