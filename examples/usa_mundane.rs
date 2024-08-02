use swisseph::*;
use swisseph::swe2::*;
use swisseph::HouseSystemKind::*;
use swisseph::Body::*;
use CalandarKind::*;


fn main() {

    // 5:10 PM LMT +5

    //Thursday, July 4, 1776, 4:50 PM
    // EDT Eastern Daylight Savings Time 4 hour offset
    //Philadelphia, PA, USA  39.9526° N, 75.1652° W
    let tjd = utc_to_jd2(1776, 7, 4, 16, 50, 4., Gregorian).unwrap();
    let tjd_ut = tjd.ut;
    let geolat = -75.1652;
    let geolon =  39.9526;

    let nutation = calc_ut2_ecliptic(tjd_ut, EclNut, Seflg::SPEED).unwrap();
    let eps = nutation.longitude;

    let h = houses2(tjd_ut, geolon, geolat, Placidus);
    let armc = h.1.armc; // should be in degrees
    let pp: Vec<ZodiacalBody> = Body::standard_bodies().iter().map(|b| {
        let planet_pos = calc_ut2_ecliptic(tjd_ut, b.clone(), Seflg::SPEED).unwrap();
        let planet_lon = planet_pos.longitude;
        let planet_lat = planet_pos.latitude;
        let hp = house_pos2(armc, geolon, eps, Placidus, planet_lon, planet_lat).unwrap();
        let body_deg = split_deg2_zodiacal(planet_lon, SplitDegKind::none());
        //let body_deg = SplitDegreeZodiacal::from_split_deg(body_deg);
        let zb = ZodiacalBody::new(b.clone(), hp, body_deg);

        zb
    }).collect();

    let _z_asc_mc = ZodiacalAscMc::new(h.1.clone());
    let _z_cusp = ZodiacalCusp::new(h.0);
    let _zodiacal_house = ZodiacalHouses { asc_mc: _z_asc_mc, cusp: _z_cusp };

    let _zodiacal_info = ZodiacalInfo::new(_zodiacal_house, pp);

    panic!("zb: {:#?}", _zodiacal_info);
    //panic!("zb: {}", _zodiacal_info);

    //let s = split_deg2(h.1.ascendant, SplitDegKind::Zodiacal);
    //panic!("ASC: {:#?}", s);
}

