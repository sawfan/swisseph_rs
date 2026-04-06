use crate::*;
use derive_builder::*;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
#[allow(dead_code)]
struct Calc {
    tjd: f64,
    body: Body,
    //iflag: i32,

    // Reserved for future house-related calculations.
    #[allow(dead_code)]
    hsys: HouseSystemKind,
    //planet_lon: f64,
    //planet_lat: f64,
}

impl Calc {
    //    pub fn calc(&self) {
    //        //swe2::calc2();
    //    }

    pub fn equatorial(&self) -> Result<EquatorialPosition, String> {
        let flag = Seflg::EQUATORIAL;
        swe2::calc_ut2_equatorial(self.tjd, self.body.clone(), flag)
    }
}

// Kept as a scratchpad for development; not part of the public API.
#[allow(dead_code)]
fn _test_builder_scratchpad() {
    use Body::*;
    use HouseSystemKind::*;

    let calc = CalcBuilder::default()
        .hsys(WholeSign)
        .body(Sun)
        .build()
        .unwrap();

    let _e = calc.equatorial();
    let _ = _e;
    //println!("{:?}", c);
}
