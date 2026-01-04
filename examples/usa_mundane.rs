use swisseph::swe2::*;
use swisseph::Body::*;
use swisseph::HouseSystemKind::*;
use swisseph::*;
use CalandarKind::*;

fn main() {
    // USA "Sibly" chart example
    // Expected: Sagittarius rising chart for July 4, 1776, 5:10 PM LMT, Philadelphia, PA
    // Philadelphia, PA, USA: 39.9526° N, 75.1652° W
    // Local Mean Time (LMT) at this longitude is approximately UTC-5.

    // Convert 5:10 PM LMT (UTC-5) to UT, then to Julian Day.
    let ut_time = utc_time_zone2(1776, 7, 4, 17, 10, 0.0, -5.0);
    let tjd = utc_to_jd2(
        ut_time.iyear_out,
        ut_time.imonth_out,
        ut_time.iday_out,
        ut_time.ihour_out,
        ut_time.imin_out,
        ut_time.dsec_out,
        Gregorian,
    )
    .unwrap();
    let tjd_ut = tjd.ut;

    // Use clear, conventional latitude/longitude values
    let geolat = 39.9526; // 39.9526° N
    let geolon = -75.1652; // 75.1652° W (west is negative in Swiss Ephemeris)

    // True obliquity from nutation
    let nutation = calc_ut2_ecliptic(tjd_ut, EclNut, Seflg::SPEED).unwrap();
    let eps = nutation.longitude;

    // Compute Placidus houses with correct (lat, lon) ordering
    let h = houses2(tjd_ut, geolat, geolon, Placidus);
    let armc = h.1.armc; // right ascension of MC in degrees

    // Planet positions and house positions
    let pp: Vec<ZodiacalBody> = Body::standard_bodies()
        .iter()
        .map(|b| {
            let planet_pos = calc_ut2_ecliptic(tjd_ut, b.clone(), Seflg::SPEED).unwrap();
            let planet_lon = planet_pos.longitude;
            let planet_lat = planet_pos.latitude;

            // Use the same latitude for house_pos2
            let hp = house_pos2(armc, geolat, eps, Placidus, planet_lon, planet_lat).unwrap();
            let body_deg = split_deg2_zodiacal(planet_lon, SplitDegKind::none());

            ZodiacalBody::new(b.clone(), hp, body_deg)
        })
        .collect();

    let _z_asc_mc = ZodiacalAscMc::new(h.1.clone());
    let _z_cusp = ZodiacalCusp::new(h.0);
    let _zodiacal_house = ZodiacalHouses {
        asc_mc: _z_asc_mc,
        cusp: _z_cusp,
    };

    let _zodiacal_info = ZodiacalInfo::new(_zodiacal_house, pp);
    let asc_sign = _zodiacal_info.clone().houses.asc_mc.asc.sign;

    assert_eq!(ZodiacalSign::Sagittarius, asc_sign);

    println!("{:#?}", _zodiacal_info);

    swe::close();
}
