use crate::*;

use std::fmt;
//use crate::SplitDegKind::Zodiacal;

#[derive(Clone)]
pub struct ZodiacalAscMc {
    pub asc_mc: AscMc,
    pub asc: ZodiacalSplitDegree,
    pub mc: ZodiacalSplitDegree,
    pub vertex: ZodiacalSplitDegree,
}

impl ZodiacalAscMc {
    pub fn new(asc_mc: AscMc) -> Self {
        let asc = swe2::split_deg2_zodiacal(asc_mc.ascendant, SplitDegKind::none());
        let mc = swe2::split_deg2_zodiacal(asc_mc.mc, SplitDegKind::none());
        let vertex = swe2::split_deg2_zodiacal(asc_mc.vertex, SplitDegKind::none());
        Self { asc_mc, mc, asc, vertex }
    }

    pub fn to_text(&self) -> Vec<String> {
        let asc = format!("Asc {}", self.asc.to_text());
        let mc = format!("Mc {}", self.mc.to_text());
        let vertex = format!("Vx {}", self.vertex.to_text());
        vec![asc, mc, vertex]
    }

//    pub fn from_asc_mc(asc_mc: AscMc) -> Self {
//        Self::new(asc_mc)
//    }
}

impl fmt::Debug for ZodiacalAscMc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZodiacalAscMc")
            .field("mc", &self.mc)
            .field("asc", &self.asc)
            .finish()
    }
}

//impl fmt::Display for ZodiacalAscMc {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//
//        use crate::swe2::split_deg2;
//        let z_mc = split_deg2(self.asc_mc.mc, SplitDegKind::Zodiacal);
//        write!(f, "{}", z_mc)
//    }
//}
//
//

