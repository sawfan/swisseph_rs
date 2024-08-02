use derive_builder::*;
use crate::*;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
struct Calc {
    tjd: f64,
    body: Body,
    //iflag: i32,
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

pub fn test() {
    use HouseSystemKind::*;
    use Body::*;

    let calc = CalcBuilder::default()
        .hsys(WholeSign)
        .body(Sun)
        .build()
        .unwrap();
    
    let e = calc.equatorial();
    //println!("{:?}", c);

}

