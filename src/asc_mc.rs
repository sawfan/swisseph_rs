use crate::*;
//use libswisseph_sys::AS_MAXCH;

use libswisseph_sys::{
    SE_ASC, SE_MC, SE_ARMC, SE_VERTEX, SE_EQUASC, SE_COASC1, SE_COASC2, 
    SE_POLASC, SE_NASCMC 
};

#[derive(Debug, Clone)]
pub struct AscMc {
    pub ascendant: f64,
    pub mc: f64,
    pub armc: f64,
    pub vertex: f64,
    pub equatorial_ascendant: f64,
    pub co_ascendant_wk: f64, // walter_koch
    pub co_ascendant_mm: f64, // michael_munkasey
    pub polar_ascendant: f64,
}

impl AscMc {
    pub fn from_array(ascmc: [f64; 10]) -> Self {
        let ascendant = ascmc[SE_ASC as usize];
        let mc = ascmc[SE_MC as usize];
        let armc = ascmc[SE_ARMC as usize];
        let vertex = ascmc[SE_VERTEX as usize];
        let equatorial_ascendant = ascmc[SE_EQUASC as usize];
        let co_ascendant_wk = ascmc[SE_COASC1 as usize];
        let co_ascendant_mm = ascmc[SE_COASC2 as usize];
        let polar_ascendant = ascmc[SE_POLASC as usize];
        let _nascmc = ascmc[SE_NASCMC as usize];
        Self::new(
            ascendant,
            mc,
            armc,
            vertex,
            equatorial_ascendant,
            co_ascendant_wk,
            co_ascendant_mm,
            polar_ascendant,
        )
    }

    pub fn new(
        ascendant: f64,
        mc: f64,
        armc: f64,
        vertex: f64,
        equatorial_ascendant: f64,
        co_ascendant_wk: f64, // walter_koch
        co_ascendant_mm: f64, // michael_munkasey
        polar_ascendant: f64,
    ) -> Self {
        Self {
            ascendant,
            mc,
            armc,
            vertex,
            equatorial_ascendant,
            co_ascendant_wk,
            co_ascendant_mm,
            polar_ascendant,
        }
    }
}
