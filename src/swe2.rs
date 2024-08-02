use crate::*;

use swe::split_deg;

pub fn house_name2(hsys: HouseSystemKind) -> String {
    swe::house_name(hsys as i32)
}

pub fn sidtime(tjd_ut: f64) -> f64 {
    swe::sidtime(tjd_ut)
}

pub fn sidtime02(tjd_ut: f64, eps: f64, nut: f64) -> f64 {
    swe::sidtime0(tjd_ut, eps, nut)
}

//With house system ‘G’ (Gauquelin sectors), a value between 1.0 and 36.9999999 is returned.
//Note that, while all other house systems number house cusps in counterclockwise direction,
//Gauquelin sectors are numbered in clockwise direction.
pub fn house_pos2(
    armc: f64,
    geolat: f64,
    eps: f64,
    hsys: HouseSystemKind,
    planet_lon: f64,
    planet_lat: f64,
) -> Result<f64, String> {
    let r = swe::house_pos(armc, geolat, eps, hsys as i32, planet_lon, planet_lat)?;

    Ok(r)
}

pub fn houses_ex2(
    tjd_ut: f64,
    iflag: i32,
    geolat: f64,
    geolon: f64,
    hsys: HouseSystemKind,
) -> (Cusp, AscMc) {
    // TODO: cusps needs to be optionally 37 when hsys is G
    let (c, a) = swe::houses_ex(tjd_ut, iflag, geolat, geolon, hsys as i32);
    let ascmc = AscMc::from_array(a);
    let cusp = Cusp::from_array(c);
    (cusp, ascmc)
}

pub fn houses2(tjd_ut: f64, geolat: f64, geolon: f64, hsys: HouseSystemKind) -> (Cusp, AscMc) {
    // TODO: cusps needs to be optionally 37 when hsys is G
    let (c, a) = swe::houses(tjd_ut, geolat, geolon, hsys as i32);
    let ascmc = AscMc::from_array(a);
    let cusp = Cusp::from_array(c);
    (cusp, ascmc)
}

//ascmc[0] = Ascendant
//ascmc[1] = MC
//ascmc[2] = ARMC
//ascmc[3] = Vertex
//ascmc[4] = "equatorial ascendant"
//ascmc[5] = "co-ascendant" (Walter Koch)
//ascmc[6] = "co-ascendant" (Michael Munkasey)
//ascmc[7] = "polar ascendant" (M. Munkasey)
pub fn split_deg2(ddeg: f64, roundflag: SplitDegKind) -> SplitDegree {
    let sd = split_deg(ddeg, roundflag.bits() as i32);
    SplitDegree::from_tuple(sd)
}

pub fn split_deg2_zodiacal(ddeg: f64, roundflag: SplitDegKind) -> ZodiacalSplitDegree {
    let s = split_deg2(ddeg, SplitDegKind::Zodiacal | roundflag);
    let sd = ZodiacalSplitDegree::from_split_deg(s);
    sd
}

pub fn calc2(
    tjd: f64, 
    body: Body, 
    flag: Seflg
) -> Result<CalcResult, String> {
    let ipl = body as i32;
    let c = swe::calc(tjd, ipl, flag.bits() as i32)?;

    if flag.contains(Seflg::EQUATORIAL) {
        //EquatorialPosition {};
        todo!();
    } else if flag.contains(Seflg::XYZ) {
        //RectangularPosition{};
        todo!();
    } else {
        let p = EclipticPosition::with_array(c.out);
        return Ok(CalcResult::_EclipticPosition(p));
    }
}

pub fn calc_ut2(
    tjd: f64, 
    body: Body, 
    flag: Seflg
) -> Result<CalcResult, String> {
    let ipl = body as u32;
    let c = swe::calc_ut(tjd, ipl, flag.bits())?;

    if flag.contains(Seflg::EQUATORIAL) {
        //EquatorialPosition {};
        todo!();
    } else if flag.contains(Seflg::XYZ) {
        //RectangularPosition{};
        todo!();
    } else {
        let p = EclipticPosition::with_array(c.out);
        return Ok(CalcResult::_EclipticPosition(p));
    }
}

use CalcResult::*;
pub fn calc_ut2_ecliptic(
    tjd: f64,
    body: Body,
    flag: Seflg,
) -> Result<EclipticPosition, String> {
    let c = calc_ut2(tjd, body, flag)?;
    match c {
        _EclipticPosition(p) => return Ok(p),
        _ => {
            panic!("Expected ecliptic position!")
        }
    };
}

pub fn calc_ut2_equatorial(
    tjd: f64,
    body: Body,
    flag: Seflg,
) -> Result<EquatorialPosition, String> {
    let c = calc_ut2(tjd, body, Seflg::EQUATORIAL | flag)?;
    match c {
        _EquatorialPosition(p) => return Ok(p),
        _ => {
            panic!("Expected equitorial position!")
        }
    };
}

pub fn calc_ut2_rectangular(
    tjd: f64,
    body: Body,
    flag: Seflg,
) -> Result<RectangularPosition, String> {
    let c = calc_ut2(tjd, body, Seflg::XYZ | flag)?;
    match c {
        _RectangularPosition(p) => return Ok(p),
        _ => {
            panic!("Expected rectangular position!")
        }
    };
}

pub fn julday2(dt: DateTime, c: CalandarKind) -> f64 {
    swe::julday(dt.year, dt.month, dt.day, dt.hour, c as u32)
}

pub fn utc_to_jd2(
    iyear: i32,
    imonth: i32,
    iday: i32,
    ihour: i32,
    imin: i32,
    dsec: f64,
    gregflag: CalandarKind,
) -> Result<JulianDay, String> {
    let r = swe::utc_to_jd(iyear, imonth, iday, ihour, imin, dsec, gregflag as i32)?;

    let et = r[0];
    let ut = r[1];
    Ok(JulianDay { et, ut })
}

#[derive(Debug)]
pub struct UtcTimeZoneRet {
    pub iyear_out: i32,
    pub imonth_out: i32,
    pub iday_out: i32,
    pub ihour_out: i32,
    pub imin_out: i32,
    pub dsec_out: f64,
}

pub fn utc_time_zone2(
    iyear: i32,
    imonth: i32,
    iday: i32,
    ihour: i32,
    imin: i32,
    dsec: f64,
    d_timezone: f64,
) -> UtcTimeZoneRet {
    let (iyear_out, imonth_out, iday_out, ihour_out, imin_out, dsec_out) =
        swe::utc_time_zone(iyear, imonth, iday, ihour, imin, dsec, d_timezone);

    UtcTimeZoneRet {
        iyear_out,
        imonth_out,
        iday_out,
        ihour_out,
        imin_out,
        dsec_out,
    }
}

