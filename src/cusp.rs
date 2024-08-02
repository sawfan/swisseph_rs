use crate::*;

#[derive(Debug, Clone)]
pub struct Cusp {
    pub first: f64,
    pub second: f64,
    pub third: f64,
    pub fourth: f64,
    pub fifth: f64,
    pub sixth: f64,
    pub seventh: f64,
    pub eighth: f64,
    pub ninth: f64,
    pub tenth: f64,
    pub eleventh: f64,
    pub twelfth: f64,
}

impl Cusp {
    pub fn iter() {
    }
    pub fn new(
        first: f64,
        second: f64,
        third: f64,
        fourth: f64,
        fifth: f64,
        sixth: f64,
        seventh: f64,
        eighth: f64,
        ninth: f64,
        tenth: f64,
        eleventh: f64,
        twelfth: f64,
    ) -> Self {
        Self {
            first,
            second,
            third,
            fourth,
            fifth,
            sixth,
            seventh,
            eighth,
            ninth,
            tenth,
            eleventh,
            twelfth,
        }
    }

    pub fn from_array(cusps: [f64; 13]) -> Self {
        // cusps[0] is always 0
        let first = cusps[1];
        let second = cusps[2];
        let third = cusps[3];
        let fourth = cusps[4];
        let fifth = cusps[5];
        let sixth = cusps[6];
        let seventh = cusps[7];
        let eighth = cusps[8];
        let ninth = cusps[9];
        let tenth = cusps[10];
        let eleventh = cusps[11];
        let twelfth = cusps[12];

        Self::new(
            first, second, third, fourth, fifth, sixth, seventh, eighth, ninth, tenth, eleventh,
            twelfth,
        )
    }
}

impl IntoIterator for Cusp {
    type Item = f64;
    type IntoIter = CuspIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        CuspIntoIterator {
            cusp: self,
            index: 0,
        }
    }
}
pub struct CuspIntoIterator {
    cusp: Cusp,
    index: usize,
}

impl Iterator for CuspIntoIterator {
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        let result = match self.index {
            0 => self.cusp.first,
            1 => self.cusp.second,
            2 => self.cusp.third,
            3 => self.cusp.fourth,
            4 => self.cusp.fifth,
            5 => self.cusp.sixth,
            6 => self.cusp.seventh,
            7 => self.cusp.eighth,
            8 => self.cusp.ninth,
            9 => self.cusp.tenth,
            10 => self.cusp.eleventh,
            11 => self.cusp.twelfth,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}
