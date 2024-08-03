use libswisseph_sys as raw;

use std::ffi::CString;
use std::os::raw::c_char;

use crate::*;

////////////////////////////////////////
// next 4 all return < 0 when error
// There are potential different sized float outputs depending on flag
// todo: create different structs with named args for each case
////////////////////////////////////////

pub fn calc(
        tjd: f64,
        ipl: i32,
        iflag: i32,
) -> Result<Out<CalcPrimRet, i32>, String> {
    unsafe {
        let (mut out, mut serr) = new_ret_serr();
        let code = raw::swe_calc(
            tjd, 
            ipl, 
            iflag, 
            out.as_mut_ptr(), 
            serr.as_mut_ptr(),
        );

        if code < 0 {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        let out = Out { out, code };
        Ok(out)
    }
}

pub fn calc_ut(
    tjd_ut: f64,
    ipl: u32,
    iflag: u32,
) -> Result<Out<CalcPrimRet, i32>, String> {
    unsafe {
        let (mut out, mut serr) = new_ret_serr();
        let code = raw::swe_calc_ut(
            tjd_ut, 
            ipl as i32, 
            iflag as i32, 
            out.as_mut_ptr(), 
            serr.as_mut_ptr(),
        );

        if code < 0 {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        let out = Out { out, code };
        Ok(out)
    }
}

pub fn fixstar(
    star: *mut ::std::os::raw::c_char,  // TODO: use &str
    tjd: f64,
    iflag: i32,
) -> Result<Out<CalcPrimRet, i32>, String> {
    unsafe {
        let (mut out, mut serr) = new_ret_serr();
        let code = raw::swe_fixstar(
            star,
            tjd,
            iflag,
            out.as_mut_ptr(), 
            serr.as_mut_ptr(),
        );

        if code < 0 {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        let out = Out { out, code };
        Ok(out)
    }
}

pub fn fixstar_ut(
        star: *mut ::std::os::raw::c_char,
        tjd_ut: f64,
        iflag: i32,
) -> Result<Out<CalcPrimRet, i32>, String> {
    unsafe {
        let (mut out, mut serr) = new_ret_serr();
        let code = raw::swe_fixstar_ut(
            star,
            tjd_ut,
            iflag,
            out.as_mut_ptr(), 
            serr.as_mut_ptr(),
        );

        if code < 0 {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        let out = Out { out, code };
        Ok(out)
    }
}

////////////////////////////////////////

pub fn calc_pctr(
        tjd: f64,
        ipl: i32,
        iplctr: i32,
        iflag: i32,
) -> Result<Out<CalcPrimRet, i32>, String> {
    unsafe {
        let (mut out, mut serr) = new_ret_serr(); // TODO: double check return vec size
        let code = raw::swe_calc_pctr(
            tjd,
            ipl,
            iplctr,
            iflag,
            out.as_mut_ptr(),
            serr.as_mut_ptr()
        );

        if code < 0 {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        let out = Out { out, code };
        Ok(out)
    }
}

pub fn solcross(
    x2cross: f64,
    jd_et: f64,
    flag: i32,
) -> Result<f64, String> {
    unsafe {
        let mut serr = crate::new_serr_buffer();
        let jx = raw::swe_solcross(
            x2cross,
            jd_et,
            flag,
            serr.as_mut_ptr(),
        );

        // jx = time of next crossing
        // In case of error, a value of jx < tjd
        if jx < jd_et {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        Ok(jx)
    }
}

pub fn solcross_ut(
    x2cross: f64,
    jd_ut: f64,
    flag: i32,
) -> Result<f64, String> {
    unsafe {
        let mut serr = crate::new_serr_buffer();
        let jx = raw::swe_solcross_ut(
            x2cross,
            jd_ut,
            flag,
            serr.as_mut_ptr(),
        );

        // jx = time of next crossing
        // In case of error, a value of jx < tjd
        if jx < jd_ut {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        Ok(jx)
    }
}

pub fn mooncross(
    x2cross: f64,
    jd_et: f64,
    flag: i32,
) -> Result<f64, String> {
    unsafe {
        let mut serr = crate::new_serr_buffer();
        let jx = raw::swe_mooncross(
            x2cross,
            jd_et,
            flag,
            serr.as_mut_ptr(),
        );

        // jx = time of next crossing
        // In case of error, a value of jx < tjd
        if jx < jd_et {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        Ok(jx)
    }
}

pub fn mooncross_ut(
    x2cross: f64,
    jd_ut: f64,
    flag: i32,
) -> Result<f64, String> {
    unsafe {
        let mut serr = crate::new_serr_buffer();
        let jx = raw::swe_mooncross_ut(
            x2cross,
            jd_ut,
            flag,
            serr.as_mut_ptr(),
        );

        // jx = time of next crossing
        // In case of error, a value of jx < tjd
        if jx < jd_ut {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        Ok(jx)
    }
}

pub fn helio_cross(
        ipl: i32,
        x2cross: f64,
        jd_et: f64,
        iflag: i32,
        dir: i32,
        jd_cross: *mut f64,     // TODO:   change to array type
) -> Result<i32, String> {
    unsafe {
        let mut serr = crate::new_serr_buffer();
        let jx = raw::swe_helio_cross(
            ipl,
            x2cross,
            jd_et,
            iflag,
            dir,
            jd_cross,
            serr.as_mut_ptr(),
        );

        // jx = time of next crossing
        // In case of error, a value of jx < tjd
        if jx < 0 {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        Ok(jx)
    }
}

pub fn swe_helio_cross(
    ipl: i32,
    x2cross: f64,
    jd_ut: f64,
    iflag: i32,
    dir: i32,
    jd_cross: *mut f64, // TODO: Update
) -> Result<i32, String> {
    unsafe {
        let mut serr = crate::new_serr_buffer();
        let jx = raw::swe_helio_cross_ut(
            ipl,
            x2cross,
            jd_ut,
            iflag,
            dir,
            jd_cross,
            serr.as_mut_ptr(),
        );

        // jx = time of next crossing
        // In case of error, a value of jx < tjd
        if jx < 0 {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        Ok(jx)
    }
}

pub fn close() {
    unsafe {
        raw::swe_close()
    }
}

pub fn get_ayanamsa(tjd_et: f64) -> f64 {
    unsafe {
        raw::swe_get_ayanamsa(tjd_et)
    }
}

pub fn get_ayanamsa_ut(tjd_ut: f64) -> f64 {
    unsafe {
        raw::swe_get_ayanamsa_ut(tjd_ut)
    }
}

pub fn julday(year: i32, month: i32, day: i32, hour: f64, gregflag: u32) -> f64 {
    unsafe {
        raw::swe_julday(year, month, day, hour, gregflag as i32)
    }
}

//double *dret);      /* array of 4 doubles; declare 20 ! */
//   * - dret[0] true altitude, if possible; otherwise input value
//   * - dret[1] apparent altitude, if possible; otherwise input value
//   * - dret[2] refraction
//   * - dret[3] dip of the horizon
//   /* either SE_TRUE_TO_APP or SE_APP_TO_TRUE */
pub fn refrac_extended(
        inalt: f64,
        geoalt: f64,
        atpress: f64,
        attemp: f64,
        lapse_rate: f64,
        calc_flag: i32,
) -> Out<[f64; 20], f64> {
    unsafe {
        let mut dret: [f64; 20] = [0.0; 20];
        let code = raw::swe_refrac_extended(
            inalt,
            geoalt,
            atpress,
            attemp,
            lapse_rate,
            calc_flag,
            dret.as_mut_ptr(),
        );

        let out = Out { out: dret, code };
        out
    }
}

pub fn get_orbital_elements(
    tjd_et: f64,
    ipl: i32,
    iflag: i32,
) -> Result<Out<[f64; 6], i32>, String> {
    unsafe {
        let mut serr = crate::new_serr_buffer();
        let mut dret: [f64; 6] = [0.0; 6];

        let code = raw::swe_get_orbital_elements(
            tjd_et, 
            ipl,
            iflag,
            dret.as_mut_ptr(),
            serr.as_mut_ptr(),
        );

        if code < 0 {
            let serr = String::from_utf8(serr.iter().map(|&c| c as u8).collect()).unwrap();
            return Err(serr);
        }

        let out = Out { out: dret, code };
        Ok(out)
    }
}

pub fn deltat(tjd: f64) -> f64 {
    unsafe {
        raw::swe_deltat(tjd)
    }
}

pub fn deltat_ex(
    tjd: f64, 
    iflag: i32, 
) -> Result<f64, String> {
    unsafe {
        let mut serr = crate::new_serr_buffer();

        let code = raw::swe_deltat_ex(
            tjd, 
            iflag,
            serr.as_mut_ptr(),
        );

        if code < 0.0 {
            let serr = String::from_utf8(serr.iter().map(|&c| c as u8).collect()).unwrap();
            return Err(serr);
        }

        Ok(code)
    }
}

pub fn time_equ(
    tjd: f64, 
) -> Result<Out<[f64; 6], i32>, String> {
    unsafe {
        // TODO! check size!!!
        let mut te: [f64; 6] = [0.0; 6];
        let mut serr = crate::new_serr_buffer();

        let code = raw::swe_time_equ(
            tjd, 
            te.as_mut_ptr(), 
            serr.as_mut_ptr(),
        );

        if code < 0 {
            let serr = String::from_utf8(serr.iter().map(|&c| c as u8).collect()).unwrap();
            return Err(serr);
        }

        let out = Out { out: te, code };
        Ok(out)
    }
}

pub fn lmt_to_lat(
    tjd_lmt: f64,
    geolon: f64,
) -> Result<Out<[f64; 6], i32>, String> {
    unsafe {
        // TODO! check size!!!
        let mut tjd_lat: [f64; 6] = [0.0; 6];
        let mut serr = crate::new_serr_buffer();

        let code = raw::swe_lmt_to_lat(
            tjd_lmt,
            geolon,
            tjd_lat.as_mut_ptr(),
            serr.as_mut_ptr()
        );

        if code < 0 {
            let serr = String::from_utf8(serr.iter().map(|&c| c as u8).collect()).unwrap();
            return Err(serr);
        }

        let out = Out { out: tjd_lat, code };
        Ok(out)
    }
}

pub fn lat_to_lmt(
    tjd_lat: f64,
    geolon: f64,
) -> Result<Out<[f64; 6], i32>, String> {
    unsafe {
        // TODO! check size!!!
        let mut tjd_lmt: [f64; 6] = [0.0; 6];
        let mut serr = crate::new_serr_buffer();

        let code = raw::swe_lat_to_lmt(
            tjd_lat,
            geolon,
            tjd_lmt.as_mut_ptr(),
            serr.as_mut_ptr(),
        );

        if code < 0 {
            let serr = String::from_utf8(serr.iter().map(|&c| c as u8).collect()).unwrap();
            return Err(serr);
        }

        let out = Out { out: tjd_lmt, code };
        Ok(out)
    }
}

pub fn sidtime0(tjd_ut: f64, eps: f64, nut: f64) -> f64 {
    unsafe {
        raw::swe_sidtime0(tjd_ut, eps, nut) 
    }
}

pub fn sidtime(tjd_ut: f64) -> f64 {
    unsafe {
        raw::swe_sidtime(tjd_ut)
    }
}

pub fn get_tid_acc() -> f64 {
    unsafe {
        raw::swe_get_tid_acc()
    }
}

pub fn set_tid_acc(t_acc: f64) {
    unsafe {
        raw::swe_set_tid_acc(t_acc)
    }
}

pub fn set_delta_t_userdef(dt: f64) {
    unsafe {
        raw::swe_set_delta_t_userdef(dt)
    }
}

pub fn degnorm(x: f64) -> f64 {
    unsafe {
        raw::swe_degnorm(x)
    }
}

pub fn radnorm(x: f64) -> f64 {
    unsafe {
        raw::swe_radnorm(x)
    }
}

pub fn rad_midp(x1: f64, x0: f64) -> f64 {
    unsafe {
        raw::swe_rad_midp(x1, x0)
    }
}

pub fn deg_midp(x1: f64, x0: f64) -> f64 {
    unsafe {
        raw::swe_deg_midp(x1, x0)
    }
}

pub fn difdegn(p1: f64, p2: f64) -> f64 {
    unsafe {
        raw::swe_difdegn(p1, p2)
    }
}

pub fn difdeg2n(p1: f64, p2: f64) -> f64 {
    unsafe {
        raw::swe_difdeg2n(p1, p2)
    }
}

pub fn difrad2n(p1: f64, p2: f64) -> f64 {
    unsafe {
        raw::swe_difrad2n(p1, p2)
    }
}

pub fn swe_d2l(x: f64) -> i32 {
    unsafe {
        raw::swe_d2l(x)
    }
}

pub fn day_of_week(jd: f64) -> i32 {
    unsafe {
        raw::swe_day_of_week(jd)
    }
}

pub fn version() -> String {
    unsafe {
        let mut b = util::new_buffer();
        raw::swe_version(b.as_mut_ptr());
        util::c_chars_to_string(b.as_mut_ptr())
    }
}

pub fn get_library_path() -> String {
    unsafe {
        let mut b = util::new_buffer();
        raw::swe_get_library_path(b.as_mut_ptr());
        util::c_chars_to_string(b.as_mut_ptr())
    }
}


pub fn set_ephe_path(s: &str) {
    unsafe {
        let c_str = CString::new(s).unwrap();
        let path: *const c_char = c_str.as_ptr() as *const c_char;
        raw::swe_set_ephe_path(path);
    }
}

pub fn set_jpl_file(s: &str) {
    unsafe {
        let c_str = CString::new(s).unwrap();
        let file: *const c_char = c_str.as_ptr() as *const c_char;
        raw::swe_set_jpl_file(file);
    }
}

pub fn get_planet_name(
    ipl: i32,
) -> String {
    unsafe {
        let spname = [0; 256];
        let pn = raw::swe_get_planet_name(ipl, spname.as_ptr() as *mut i8);
        util::c_chars_to_string(pn)
    }
}

pub fn house_name(
    hsys: i32,
) -> String {
    unsafe {
        let hn = raw::swe_house_name(hsys);
        util::c_chars_to_string(hn as *mut i8)
    }
}

pub fn utc_time_zone(
    iyear: i32,
    imonth: i32,
    iday: i32,
    ihour: i32,
    imin: i32,
    dsec: f64,
    d_timezone: f64,
) -> (i32, i32, i32, i32, i32, f64) {
    unsafe {
        let mut iyear_out: i32 = 0;
        let mut imonth_out: i32 = 0;
        let mut iday_out: i32 = 0;
        let mut ihour_out: i32 = 0;
        let mut imin_out: i32 = 0;
        let mut dsec_out: f64 = 0.;

        raw::swe_utc_time_zone(
            iyear,
            imonth,
            iday,
            ihour,
            imin,
            dsec,
            d_timezone,
            &mut iyear_out,
            &mut imonth_out,
            &mut iday_out,
            &mut ihour_out,
            &mut imin_out,
            &mut dsec_out,
        );

        (iyear_out, imonth_out, iday_out, ihour_out, imin_out, dsec_out)
    }
}

pub fn utc_to_jd(
        iyear: i32,
        imonth: i32,
        iday: i32,
        ihour: i32,
        imin: i32,
        dsec: f64,
        gregflag: i32,
) -> Result<[f64;2], String> {
    unsafe {
        let mut serr = crate::new_serr_buffer();
        let mut dret: [f64;2] = [0.;2];

        let code = raw::swe_utc_to_jd(
            iyear,
            imonth,
            iday,
            ihour,
            imin,
            dsec,
            gregflag,
            dret.as_mut_ptr(),
            serr.as_mut_ptr(),
        );

        if code < 0 {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        Ok(dret)
    }
}


//extern "C" {
//    pub fn swe_revjul(
//        jd: f64,
//        gregflag: ::std::os::raw::c_int,
//        jyear: *mut ::std::os::raw::c_int,
//        jmon: *mut ::std::os::raw::c_int,
//        jday: *mut ::std::os::raw::c_int,
//        jut: *mut f64,
//    );
//}
//
pub fn revjul(
        jd: f64,
        gregflag: i32,
) -> (i32, i32, i32, f64) {
    unsafe {
        let mut jyear: i32 = 0;
        let mut jmon: i32 = 0;
        let mut jday: i32 = 0;
        let mut jut: f64 = 0.;

        raw::swe_revjul(
            jd,
            gregflag,
            &mut jyear,
            &mut jmon,
            &mut jday,
            &mut jut,
        );

        (jyear, jmon, jday, jut)
    }
}

//extern "C" {
//    pub fn swe_set_topo(geolon: f64, geolat: f64, geoalt: f64);
//}
//extern "C" {
//    pub fn swe_set_sid_mode(sid_mode: int32, t0: f64, ayan_t0: f64);
//}
//extern "C" {
//    pub fn swe_orbit_max_min_true_distance(
//        tjd_et: f64,
//        ipl: int32,
//        iflag: int32,
//        dmax: *mut f64,
//        dmin: *mut f64,
//        dtrue: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//
//extern "C" {
//    pub fn swe_mooncross_node(
//        jd_et: f64,
//        flag: int32,
//        xlon: *mut f64,
//        xlat: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> f64;
//}
//extern "C" {
//    pub fn swe_mooncross_node_ut(
//        jd_ut: f64,
//        flag: int32,
//        xlon: *mut f64,
//        xlat: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> f64;
//}
//
//extern "C" {
//    pub fn swe_helio_cross_ut(
//        ipl: int32,
//        x2cross: f64,
//        jd_ut: f64,
//        iflag: int32,
//        dir: int32,
//        jd_cross: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//
//extern "C" {
//    pub fn swe_fixstar_mag(
//        star: *mut ::std::os::raw::c_char,
//        mag: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_fixstar2(
//        star: *mut ::std::os::raw::c_char,
//        tjd: f64,
//        iflag: int32,
//        xx: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_fixstar2_ut(
//        star: *mut ::std::os::raw::c_char,
//        tjd_ut: f64,
//        iflag: int32,
//        xx: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_fixstar2_mag(
//        star: *mut ::std::os::raw::c_char,
//        mag: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//
//extern "C" {
//    pub fn swe_get_ayanamsa_ex(
//        tjd_et: f64,
//        iflag: int32,
//        daya: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_get_ayanamsa_ex_ut(
//        tjd_ut: f64,
//        iflag: int32,
//        daya: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//
//extern "C" {
//    pub fn swe_get_ayanamsa_name(isidmode: int32) -> *const ::std::os::raw::c_char;
//}
//extern "C" {
//    pub fn swe_get_current_file_data(
//        ifno: ::std::os::raw::c_int,
//        tfstart: *mut f64,
//        tfend: *mut f64,
//        denum: *mut ::std::os::raw::c_int,
//    ) -> *const ::std::os::raw::c_char;
//}
//extern "C" {
//    #[doc = " exports from swedate.c"]
//    pub fn swe_date_conversion(
//        y: ::std::os::raw::c_int,
//        m: ::std::os::raw::c_int,
//        d: ::std::os::raw::c_int,
//        utime: f64,
//        c: ::std::os::raw::c_char,
//        tjd: *mut f64,
//    ) -> ::std::os::raw::c_int;
//}
//
//
//extern "C" {
//    pub fn swe_revjul(
//        jd: f64,
//        gregflag: ::std::os::raw::c_int,
//        jyear: *mut ::std::os::raw::c_int,
//        jmon: *mut ::std::os::raw::c_int,
//        jday: *mut ::std::os::raw::c_int,
//        jut: *mut f64,
//    );
//}
//extern "C" {
//    pub fn swe_jdet_to_utc(
//        tjd_et: f64,
//        gregflag: int32,
//        iyear: *mut int32,
//        imonth: *mut int32,
//        iday: *mut int32,
//        ihour: *mut int32,
//        imin: *mut int32,
//        dsec: *mut f64,
//    );
//}
//extern "C" {
//    pub fn swe_jdut1_to_utc(
//        tjd_ut: f64,
//        gregflag: int32,
//        iyear: *mut int32,
//        imonth: *mut int32,
//        iday: *mut int32,
//        ihour: *mut int32,
//        imin: *mut int32,
//        dsec: *mut f64,
//    );
//}
//extern "C" {
//    pub fn swe_utc_time_zone(
//        iyear: int32,
//        imonth: int32,
//        iday: int32,
//        ihour: int32,
//        imin: int32,
//        dsec: f64,
//        d_timezone: f64,
//        iyear_out: *mut int32,
//        imonth_out: *mut int32,
//        iday_out: *mut int32,
//        ihour_out: *mut int32,
//        imin_out: *mut int32,
//        dsec_out: *mut f64,
//    );
//}
//extern "C" {
//    #[doc = " exports from swehouse.c"]
//    pub fn swe_houses(
//        tjd_ut: f64,
//        geolat: f64,
//        geolon: f64,
//        hsys: ::std::os::raw::c_int,
//        cusps: *mut f64,
//        ascmc: *mut f64,
//    ) -> ::std::os::raw::c_int;
//}
//
pub fn houses(
    tjd_ut: f64,
    geolat: f64,
    geolon: f64,
    hsys: i32, 
) -> ([f64; 13], [f64; 10]) {
    unsafe {
        let mut cusps: [f64; 13] = [0.0; 13];
        let mut ascmc: [f64; 10] = [0.0; 10];

//                   * eastern longitude is positive,
//                   * western longitude is negative,
//                   * northern latitude is positive,
//                   * southern latitude is negative */

        raw::swe_houses(
            tjd_ut,
            geolat,  /* geographic latitude, in degrees */
            geolon,  /* geographic longitude, in degrees */
            hsys,
            cusps.as_mut_ptr(),
            ascmc.as_mut_ptr(),
        );

        (cusps, ascmc)
    }
}

//# define SE_SPLIT_DEG_KEEP_DEG      32   * don't round to next degree
//* e.g. 10.9999999 will be rounded
//* to 10d59'59" (or 10d59' or 10d)
//* output:
//* ideg degrees,
//* imin minutes,
//* isec seconds,
//* dsecfr fraction of seconds
//* isgn zodiac sign number;
//* or +/- sign
//extern "C" {
//    pub fn swe_split_deg(
//        ddeg: f64,
//        roundflag: int32,
//        ideg: *mut int32,
//        imin: *mut int32,
//        isec: *mut int32,
//        dsecfr: *mut f64,
//        isgn: *mut int32,
//    );
//}
pub fn split_deg(
    ddeg: f64,
    roundflag: i32,

) -> (i32, i32, i32, f64, i32) {
    unsafe {
        let mut ideg: i32=0 ;//*mut int32,
        let mut imin: i32 = 0;//*mut int32,
        let mut isec: i32 = 0;//*mut int32,
        let mut dsecfr: f64 = 0.; //: *mut f64,
        let mut isgn: i32 =0 ;    //: *mut int32,

        raw::swe_split_deg(
            ddeg,
            roundflag,
            &mut ideg,
            &mut imin,
            &mut isec,
            &mut dsecfr,
            &mut isgn,
        );

        (ideg, imin, isec, dsecfr, isgn)
    }
}

pub fn houses_ex(
    tjd_ut: f64,
    iflag:  i32,
    geolat: f64,
    geolon: f64,
    hsys: i32, 
) -> ([f64; 13], [f64; 10]) {
    unsafe {
        let mut cusps: [f64; 13] = [0.0; 13];
        let mut ascmc: [f64; 10] = [0.0; 10];

//                   * eastern longitude is positive,
//                   * western longitude is negative,
//                   * northern latitude is positive,
//                   * southern latitude is negative */

        raw::swe_houses_ex(
            tjd_ut,
            iflag,
            geolat,  /* geographic latitude, in degrees */
            geolon,  /* geographic longitude, in degrees */
            hsys,
            cusps.as_mut_ptr(),
            ascmc.as_mut_ptr(),
        );

        (cusps, ascmc)
    }
}

pub fn house_pos(
    armc: f64,
    geolat: f64,
    eps: f64,
    hsys: i32,
    planet_lon: f64,
    planet_lat: f64
) -> Result<f64, String> {
    unsafe {
        let mut xpin = [planet_lon, planet_lat];
        let mut serr = crate::new_serr_buffer();

        let code = raw::swe_house_pos(
            armc,
            geolat,
            eps,
            hsys,
            xpin.as_mut_ptr(),
            serr.as_mut_ptr(),
        );

        if code < 1. {
            let serr = string_from_i8_array(serr);
            return Err(serr);
        }

        Ok(code)
    }
}

//
//extern "C" {
//    pub fn swe_houses_ex2(
//        tjd_ut: f64,
//        iflag: int32,
//        geolat: f64,
//        geolon: f64,
//        hsys: ::std::os::raw::c_int,
//        cusps: *mut f64,
//        ascmc: *mut f64,
//        cusp_speed: *mut f64,
//        ascmc_speed: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> ::std::os::raw::c_int;
//}
//extern "C" {
//    pub fn swe_houses_armc(
//        armc: f64,
//        geolat: f64,
//        eps: f64,
//        hsys: ::std::os::raw::c_int,
//        cusps: *mut f64,
//        ascmc: *mut f64,
//    ) -> ::std::os::raw::c_int;
//}
//extern "C" {
//    pub fn swe_houses_armc_ex2(
//        armc: f64,
//        geolat: f64,
//        eps: f64,
//        hsys: ::std::os::raw::c_int,
//        cusps: *mut f64,
//        ascmc: *mut f64,
//        cusp_speed: *mut f64,
//        ascmc_speed: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> ::std::os::raw::c_int;
//}
//extern "C" {
//    #[doc = " exports from swecl.c"]
//    pub fn swe_gauquelin_sector(
//        t_ut: f64,
//        ipl: int32,
//        starname: *mut ::std::os::raw::c_char,
//        iflag: int32,
//        imeth: int32,
//        geopos: *mut f64,
//        atpress: f64,
//        attemp: f64,
//        dgsect: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_sol_eclipse_where(
//        tjd: f64,
//        ifl: int32,
//        geopos: *mut f64,
//        attr: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_lun_occult_where(
//        tjd: f64,
//        ipl: int32,
//        starname: *mut ::std::os::raw::c_char,
//        ifl: int32,
//        geopos: *mut f64,
//        attr: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_sol_eclipse_how(
//        tjd: f64,
//        ifl: int32,
//        geopos: *mut f64,
//        attr: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_sol_eclipse_when_loc(
//        tjd_start: f64,
//        ifl: int32,
//        geopos: *mut f64,
//        tret: *mut f64,
//        attr: *mut f64,
//        backward: int32,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_lun_occult_when_loc(
//        tjd_start: f64,
//        ipl: int32,
//        starname: *mut ::std::os::raw::c_char,
//        ifl: int32,
//        geopos: *mut f64,
//        tret: *mut f64,
//        attr: *mut f64,
//        backward: int32,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_sol_eclipse_when_glob(
//        tjd_start: f64,
//        ifl: int32,
//        ifltype: int32,
//        tret: *mut f64,
//        backward: int32,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_lun_occult_when_glob(
//        tjd_start: f64,
//        ipl: int32,
//        starname: *mut ::std::os::raw::c_char,
//        ifl: int32,
//        ifltype: int32,
//        tret: *mut f64,
//        backward: int32,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_lun_eclipse_how(
//        tjd_ut: f64,
//        ifl: int32,
//        geopos: *mut f64,
//        attr: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_lun_eclipse_when(
//        tjd_start: f64,
//        ifl: int32,
//        ifltype: int32,
//        tret: *mut f64,
//        backward: int32,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_lun_eclipse_when_loc(
//        tjd_start: f64,
//        ifl: int32,
//        geopos: *mut f64,
//        tret: *mut f64,
//        attr: *mut f64,
//        backward: int32,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_pheno(
//        tjd: f64,
//        ipl: int32,
//        iflag: int32,
//        attr: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_pheno_ut(
//        tjd_ut: f64,
//        ipl: int32,
//        iflag: int32,
//        attr: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_refrac(inalt: f64, atpress: f64, attemp: f64, calc_flag: int32) -> f64;
//}
//
//extern "C" {
//    pub fn swe_set_lapse_rate(lapse_rate: f64);
//}
//extern "C" {
//    pub fn swe_azalt(
//        tjd_ut: f64,
//        calc_flag: int32,
//        geopos: *mut f64,
//        atpress: f64,
//        attemp: f64,
//        xin: *mut f64,
//        xaz: *mut f64,
//    );
//}
//extern "C" {
//    pub fn swe_azalt_rev(
//        tjd_ut: f64,
//        calc_flag: int32,
//        geopos: *mut f64,
//        xin: *mut f64,
//        xout: *mut f64,
//    );
//}
//extern "C" {
//    pub fn swe_rise_trans_true_hor(
//        tjd_ut: f64,
//        ipl: int32,
//        starname: *mut ::std::os::raw::c_char,
//        epheflag: int32,
//        rsmi: int32,
//        geopos: *mut f64,
//        atpress: f64,
//        attemp: f64,
//        horhgt: f64,
//        tret: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_rise_trans(
//        tjd_ut: f64,
//        ipl: int32,
//        starname: *mut ::std::os::raw::c_char,
//        epheflag: int32,
//        rsmi: int32,
//        geopos: *mut f64,
//        atpress: f64,
//        attemp: f64,
//        tret: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_nod_aps(
//        tjd_et: f64,
//        ipl: int32,
//        iflag: int32,
//        method: int32,
//        xnasc: *mut f64,
//        xndsc: *mut f64,
//        xperi: *mut f64,
//        xaphe: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_nod_aps_ut(
//        tjd_ut: f64,
//        ipl: int32,
//        iflag: int32,
//        method: int32,
//        xnasc: *mut f64,
//        xndsc: *mut f64,
//        xperi: *mut f64,
//        xaphe: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//
//extern "C" {
//    #[doc = " other functions from swephlib.c;\n they are not needed for Swiss Ephemeris,\n but may be useful to former Placalc users."]
//    pub fn swe_csnorm(p: centisec) -> centisec;
//}
//extern "C" {
//    pub fn swe_difcsn(p1: centisec, p2: centisec) -> centisec;
//}
//
//extern "C" {
//    pub fn swe_csroundsec(x: centisec) -> centisec;
//}
//
//extern "C" {
//    pub fn swe_cs2timestr(
//        t: centisec,
//        sep: ::std::os::raw::c_int,
//        suppressZero: AS_BOOL,
//        a: *mut ::std::os::raw::c_char,
//    ) -> *mut ::std::os::raw::c_char;
//}
//extern "C" {
//    pub fn swe_cs2lonlatstr(
//        t: centisec,
//        pchar: ::std::os::raw::c_char,
//        mchar: ::std::os::raw::c_char,
//        s: *mut ::std::os::raw::c_char,
//    ) -> *mut ::std::os::raw::c_char;
//}
//extern "C" {
//    pub fn swe_cs2degstr(
//        t: centisec,
//        a: *mut ::std::os::raw::c_char,
//    ) -> *mut ::std::os::raw::c_char;
//}
//
//extern "C" {
//    pub fn swe_difcs2n(p1: centisec, p2: centisec) -> centisec;
//}
//
//    pub fn swe_heliacal_ut(
//        tjdstart_ut: f64,
//        geopos: *mut f64,
//        datm: *mut f64,
//        dobs: *mut f64,
//        ObjectName: *mut ::std::os::raw::c_char,
//        TypeEvent: int32,
//        iflag: int32,
//        dret: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_heliacal_pheno_ut(
//        tjd_ut: f64,
//        geopos: *mut f64,
//        datm: *mut f64,
//        dobs: *mut f64,
//        ObjectName: *mut ::std::os::raw::c_char,
//        TypeEvent: int32,
//        helflag: int32,
//        darr: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_vis_limit_mag(
//        tjdut: f64,
//        geopos: *mut f64,
//        datm: *mut f64,
//        dobs: *mut f64,
//        ObjectName: *mut ::std::os::raw::c_char,
//        helflag: int32,
//        dret: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_heliacal_angle(
//        tjdut: f64,
//        dgeo: *mut f64,
//        datm: *mut f64,
//        dobs: *mut f64,
//        helflag: int32,
//        mag: f64,
//        azi_obj: f64,
//        azi_sun: f64,
//        azi_moon: f64,
//        alt_moon: f64,
//        dret: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_topo_arcus_visionis(
//        tjdut: f64,
//        dgeo: *mut f64,
//        datm: *mut f64,
//        dobs: *mut f64,
//        helflag: int32,
//        mag: f64,
//        azi_obj: f64,
//        alt_obj: f64,
//        azi_sun: f64,
//        azi_moon: f64,
//        alt_moon: f64,
//        dret: *mut f64,
//        serr: *mut ::std::os::raw::c_char,
//    ) -> int32;
//}
//extern "C" {
//    pub fn swe_set_astro_models(samod: *mut ::std::os::raw::c_char, iflag: int32);
//}
//extern "C" {
//    pub fn swe_get_astro_models(
//        samod: *mut ::std::os::raw::c_char,
//        sdet: *mut ::std::os::raw::c_char,
//        iflag: int32,
//    );
//}
//
//extern "C" {
//    pub fn swe_set_interpolate_nut(do_interpolate: AS_BOOL);
//}
//extern "C" {
//    pub fn swe_cotrans(xpo: *mut f64, xpn: *mut f64, eps: f64);
//}
//extern "C" {
//    pub fn swe_cotrans_sp(xpo: *mut f64, xpn: *mut f64, eps: f64);
//}
