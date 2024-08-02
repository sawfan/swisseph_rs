use crate::*;

#[derive(Clone, Debug)]
pub struct ZodiacalHouses {
    pub asc_mc: ZodiacalAscMc,
    pub cusp: ZodiacalCusp,
}

impl ZodiacalHouses {
    pub fn new(asc_mc: ZodiacalAscMc, cusp: ZodiacalCusp) -> Self {
        Self { asc_mc, cusp }
    }

    pub fn to_text(&self) -> (Vec<String>, Vec<String>) {
        (self.asc_mc.to_text(), self.cusp.to_text())
    }
}
