use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ZodiacalSplitDegree {
    pub split_degree: SplitDegree,
    pub sign: ZodiacalSign,
}

impl ZodiacalSplitDegree {
    pub fn from_split_deg(split_degree: SplitDegree) -> Self {
        let sign = ZodiacalSign::from_repr(split_degree.isgn.try_into().unwrap()).unwrap();
        Self { split_degree, sign }
    }

    pub fn to_text(&self) -> String {
        let sign = self.sign.to_text();
        //let degree = "degree".to_owned();
        let degree = self.split_degree.to_text();
        format!("{}{}", sign, degree)
    }
}

//impl fmt::Display for ZodiacalSplitDegree {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", self.0)
//    }
//}

