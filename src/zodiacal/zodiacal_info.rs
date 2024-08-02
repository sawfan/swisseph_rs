use crate::*;

use std::fmt;

#[derive(Clone)]
pub struct ZodiacalInfo {
    pub houses: ZodiacalHouses,
    pub bodies: Vec<ZodiacalBody>,
}

impl ZodiacalInfo {
    pub fn new(houses: ZodiacalHouses, bodies: Vec<ZodiacalBody>) -> Self {
        Self { houses, bodies }
    }

    pub fn to_text(&self) -> (Vec<String>, (Vec<String>, Vec<String>)) {
        let bodies: Vec<String> = self.bodies
            .iter()
            .map(|x| { 
                x.to_text() 
            })
            .collect();

        (bodies, self.houses.to_text())
    }

}

impl fmt::Debug for ZodiacalInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZodiacalInfo")
            .field("houses", &self.houses)
            .field("bodies", &self.bodies)
            .finish()
    }
}

//impl fmt::Display for ZodiacalInfo {
//    // This trait requires `fmt` with this exact signature.
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", self.0)
//    }
//}

