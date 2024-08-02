use crate::*;
use std::fmt;


#[derive(Clone)]
pub struct ZodiacalCusp {
    pub cusp: Cusp,

    pub first       :ZodiacalSplitDegree,
    pub second      :ZodiacalSplitDegree,
    pub third       :ZodiacalSplitDegree,
    pub fourth      :ZodiacalSplitDegree,
    pub fifth       :ZodiacalSplitDegree,
    pub sixth       :ZodiacalSplitDegree,
    pub seventh     :ZodiacalSplitDegree,
    pub eighth      :ZodiacalSplitDegree,
    pub ninth       :ZodiacalSplitDegree,
    pub tenth       :ZodiacalSplitDegree,
    pub eleventh    :ZodiacalSplitDegree,
    pub twelfth     :ZodiacalSplitDegree,
}

impl ZodiacalCusp {
    pub fn new(cusp: Cusp) -> Self {

        let first       = swe2::split_deg2_zodiacal(cusp.first, SplitDegKind::none());
        let second      = swe2::split_deg2_zodiacal(cusp.second, SplitDegKind::none());
        let third       = swe2::split_deg2_zodiacal(cusp.third, SplitDegKind::none());
        let fourth      = swe2::split_deg2_zodiacal(cusp.fourth, SplitDegKind::none());
        let fifth       = swe2::split_deg2_zodiacal(cusp.fifth, SplitDegKind::none());
        let sixth       = swe2::split_deg2_zodiacal(cusp.sixth, SplitDegKind::none());
        let seventh     = swe2::split_deg2_zodiacal(cusp.seventh, SplitDegKind::none());
        let eighth      = swe2::split_deg2_zodiacal(cusp.eighth, SplitDegKind::none());
        let ninth       = swe2::split_deg2_zodiacal(cusp.ninth, SplitDegKind::none());
        let tenth       = swe2::split_deg2_zodiacal(cusp.tenth, SplitDegKind::none());
        let eleventh    = swe2::split_deg2_zodiacal(cusp.eleventh, SplitDegKind::none());
        let twelfth     = swe2::split_deg2_zodiacal(cusp.twelfth, SplitDegKind::none());

        Self { 
            cusp,

            first    ,
            second   ,
            third    ,
            fourth   ,
            fifth    ,
            sixth    ,
            seventh  ,
            eighth   ,
            ninth    ,
            tenth    ,
            eleventh ,
            twelfth  ,

        }
    }

    pub fn to_text(&self) -> Vec<String> {
        self.clone().into_iter().map(|x| x.to_text()).collect()
    }

}

impl fmt::Debug for ZodiacalCusp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        f.debug_struct("ZodiacalAscMc")
            .field("first", &self.first)
            .field("second", &self.second)
            .field("third", &self.third)
            .field("fourth", &self.fourth)
            .field("fifth", &self.fifth)
            .field("sixth", &self.sixth)
            .field("seventh", &self.seventh)
            .field("eighth",&self.eighth)
            .field("ninth", &self.ninth)
            .field("tenth", &self.tenth)
            .field("eleventh", &self.eleventh)
            .field("twelfth", &self.twelfth)
            .finish()
    }
}

impl IntoIterator for ZodiacalCusp {
    type Item = ZodiacalSplitDegree;
    type IntoIter = ZodiacalCuspIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        ZodiacalCuspIntoIterator {
            cusp: self,
            index: 0,
        }
    }
}
pub struct ZodiacalCuspIntoIterator {
    cusp: ZodiacalCusp,
    index: usize,
}

impl Iterator for ZodiacalCuspIntoIterator {
    type Item = ZodiacalSplitDegree;
    fn next(&mut self) -> Option<ZodiacalSplitDegree> {
        let result = match self.index {
            0 => self.cusp.first.clone(),
            1 => self.cusp.second.clone(),
            2 => self.cusp.third.clone(),
            3 => self.cusp.fourth.clone(),
            4 => self.cusp.fifth.clone(),
            5 => self.cusp.sixth.clone(),
            6 => self.cusp.seventh.clone(),
            7 => self.cusp.eighth.clone(),
            8 => self.cusp.ninth.clone(),
            9 => self.cusp.tenth.clone(),
            10 => self.cusp.eleventh.clone(),
            11 => self.cusp.twelfth.clone(),
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

