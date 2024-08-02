use crate::*;

use crate::body::Body;

#[derive(Clone, Debug)]
pub struct ZodiacalBody {
    pub body: Body,
    pub house_position: f64,
    pub degree: ZodiacalSplitDegree,
}
impl ZodiacalBody {
    pub fn new(body: Body, house_position: f64, degree: ZodiacalSplitDegree) -> Self {
        Self {
            body,
            house_position,
            degree,
        }
    }

    pub fn to_text(&self) -> String {
        let degree = self.degree.to_text();
        let body = self.body.to_text();
        format!("{} {}", body, degree)
    }
}

