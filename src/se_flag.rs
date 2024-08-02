use libswisseph_sys::*;
use bitmask_enum::bitmask;

#[bitmask(u32)]
pub enum Seflg {
    JPLEPH          = SEFLG_JPLEPH,
    SWIEPH          = SEFLG_SWIEPH,
    MOSEPH          = SEFLG_MOSEPH,
    HELCTR          = SEFLG_HELCTR,
    TRUEPOS         = SEFLG_TRUEPOS,
    J2000           = SEFLG_J2000,
    NONUT           = SEFLG_NONUT,
    SPEED3          = SEFLG_SPEED3,
    SPEED           = SEFLG_SPEED,
    NOGDEFL         = SEFLG_NOGDEFL,
    NOABERR         = SEFLG_NOABERR,
    ASTROMETRIC     = SEFLG_ASTROMETRIC,
    EQUATORIAL      = SEFLG_EQUATORIAL,
    XYZ             = SEFLG_XYZ,
    RADIANS         = SEFLG_RADIANS,
    BARYCTR         = SEFLG_BARYCTR,
    TOPOCTR         = SEFLG_TOPOCTR,
    ORBEL_AA        = SEFLG_ORBEL_AA,
    TROPICAL        = SEFLG_TROPICAL,
    SIDEREAL        = SEFLG_SIDEREAL,
    ICRS            = SEFLG_ICRS,
    DPSIDEPS_1980   = SEFLG_DPSIDEPS_1980,
    JPLHOR          = SEFLG_JPLHOR,
    JPLHOR_APPROX   = SEFLG_JPLHOR_APPROX,
    CENTER_BODY     = SEFLG_CENTER_BODY,
    TEST_PLMOON     = SEFLG_TEST_PLMOON,
}
