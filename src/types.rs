//use strum_macros::{EnumString, EnumIter, FromRepr};
use crate::zodiacal_sign::*;

pub type CalcPrimRet = [f64; 6];

use libswisseph_sys::AS_MAXCH;
pub const MAXCH: usize = AS_MAXCH as usize;
pub type BufferType = [i8; MAXCH];

#[derive(Debug)]
pub struct Out<T, U> {
    pub out: T,
    pub code: U,
}

impl<T, U> From<Out<T, U>> for (T, U) {
    fn from(e: Out<T, U>) -> (T, U) {
        (e.out, e.code)
    }
}



#[bitmask(u32)]
pub enum SplitDegKind {
    RoundSec   = SE_SPLIT_DEG_ROUND_SEC ,
    RoundMin   = SE_SPLIT_DEG_ROUND_MIN ,
    RoundDeg   = SE_SPLIT_DEG_ROUND_DEG ,
    Zodiacal   = SE_SPLIT_DEG_ZODIACAL  ,
    Nakshatra  = SE_SPLIT_DEG_NAKSHATRA ,
    KeepSign   = SE_SPLIT_DEG_KEEP_SIGN ,
    KeepDeg    = SE_SPLIT_DEG_KEEP_DEG  ,
}

use bitmask_enum::bitmask;
use libswisseph_sys::*;
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


#[derive(Debug)]
pub enum CalandarKind {
    Jul     = SE_JUL_CAL    as isize,
    Greg    = SE_GREG_CAL   as isize,
}

pub struct DateTime {
    pub year    : i32,
    pub month   : i32,
    pub day     : i32,
    pub hour    : f64,
}

impl DateTime {

    pub fn new(
        year    : i32,
        month   : i32,
        day     : i32,
        hour    : f64,

    ) -> Self {
        Self {
            year    ,
            month   ,
            day     ,
            hour    ,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SplitDegreeZodiacal {
    pub split_degree: SplitDegree,
    pub sign: ZodiacalSign,
}

impl SplitDegreeZodiacal {
    pub fn from_split_deg(split_degree: SplitDegree) -> Self {
        let sign = ZodiacalSign::from_repr(split_degree.isgn.try_into().unwrap()).unwrap();
        Self {
            split_degree,
            sign, 
        }
    }
}

#[derive(Debug, Clone)]
pub struct SplitDegree {
    pub ideg: i32, 
    pub imin: i32, 
    pub isec: i32, 
    pub dsecfr: f64, 
    pub isgn: i32,
}

impl SplitDegree {
    pub fn from_tuple(t: (i32, i32, i32, f64, i32)) -> SplitDegree {
        let (ideg, imin, isec, dsecfr, isgn) = t;
        Self {
            ideg, 
            imin, 
            isec, 
            dsecfr, 
            isgn,
        }
    }
}



#[derive(Debug)]
pub enum HouseSystemKind {
    Alcabitus                                = 'B' as isize,            
    ApcHouses                                = 'Y' as isize,            
    AxialRotationSystemMeridianSystemZariel  = 'X' as isize,            
    AzimuthalHorizontalSystem                = 'H' as isize,            
    Campanus                                 = 'C' as isize,            
    CarterPoliEquatorial                     = 'F' as isize,            
    Equal                                    = 'A' as isize, //or ‘E’ (cusp 1 is Ascendant)
    EqualMc                                  = 'D' as isize, //(cusp 10 is MC)
    Equal1Aries                              = 'N' as isize,            
    GauquelinSector                          = 'G' as isize,            
    //Goelzer -> Krusinski                                
    //Horizontal system -> Azimuthal system                

    SunshineMakranskySolutionTreindl         = 'I' as isize,           
    SunshineMakranskySolutionMakransky       = 'i' as isize,            
    Koch                                     = 'K' as isize,            
    KrusinskiPisaGoelzer                     = 'U' as isize,            
    //Meridian system -> axial rotation                    

    Morinus                                  = 'M' as isize,
    //Neo-Porphyry -> Pullen SD                           
    //Pisa -> Krusinski                                    

    Placidus                                 = 'P' as isize,            
    //Poli-Equatorial -> Carter                           

    PolichPageTopocentricSystem              = 'T' as isize,            
    Porphyrius                               = 'O' as isize,            
    PullenSdSinusoidalDeltaExNeoPorphyry     = 'L' as isize,            
    PullenSrSinusoidalRatio                  = 'Q' as isize,            
    Regiomontanus                            = 'R' as isize,            
    Sripati                                  = 'S' as isize,            
    //“Topocentric” system -> Polich/Page                  

    VehlowEqual                              = 'V' as isize,//(Asc. in middle of house 1)
    WholeSign                                = 'W' as isize,            
    //Zariel -> Axial rotation system                     
}


#[derive(Debug, Clone)]
pub struct Cusp {
    pub first       : f64,
    pub second      : f64,
    pub third       : f64,
    pub fourth      : f64,
    pub fifth       : f64,
    pub sixth       : f64,
    pub seventh     : f64,
    pub eighth      : f64,
    pub ninth       : f64,
    pub tenth       : f64,
    pub eleventh    : f64,
    pub twelfth     : f64
}

impl Cusp {
    pub fn new(
        first     :f64, 
        second    :f64,  
        third     :f64, 
        fourth    :f64, 
        fifth     :f64, 
        sixth     :f64, 
        seventh   :f64, 
        eighth    :f64, 
        ninth     :f64, 
        tenth     :f64, 
        eleventh  :f64, 
        twelfth   :f64, 
    ) -> Self {
        Self {
            first     , 
            second    ,  
            third     , 
            fourth    , 
            fifth     , 
            sixth     , 
            seventh   , 
            eighth    , 
            ninth     , 
            tenth     , 
            eleventh  , 
            twelfth   , 
        }
    }

    pub fn from_array(cusps: [f64;13]) -> Self {
        // cusps[0] is always 0
        let first      = cusps[1];
        let second     = cusps[2]; 
        let third      = cusps[3];
        let fourth     = cusps[4];
        let fifth      = cusps[5];
        let sixth      = cusps[6];
        let seventh    = cusps[7];
        let eighth     = cusps[8];
        let ninth      = cusps[9];
        let tenth      = cusps[10];
        let eleventh   = cusps[11];
        let twelfth    = cusps[12];

        Self::new(
            first     , 
            second    , 
            third     , 
            fourth    , 
            fifth     , 
            sixth     , 
            seventh   , 
            eighth    , 
            ninth     , 
            tenth     , 
            eleventh  , 
            twelfth   , 

        )
    }
}

impl fmt::Debug for ZodiacalAscMc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::swe2::split_deg2;

        f.debug_struct("ZodiacalAscMc")
         .field("mc", & split_deg2(self.asc_mc.mc,SplitDegKind::Zodiacal ))
         .field("asc", & split_deg2(self.asc_mc.ascendant,SplitDegKind::Zodiacal ))
         .finish()
    }
}

use std::fmt;
//impl fmt::Display for ZodiacalAscMc {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//
//        use crate::swe2::split_deg2;
//        let z_mc = split_deg2(self.asc_mc.mc, SplitDegKind::Zodiacal);
//        write!(f, "{}", z_mc)
//    }
//}
//
//
use crate::body::Body;
#[derive(Clone, Debug)]
pub struct ZodiacalBody {
    pub body: Body,
    pub house_position: f64,
    pub degree: SplitDegreeZodiacal,
}
impl ZodiacalBody {
    pub fn new(body: Body, house_position: f64, degree: SplitDegreeZodiacal) -> Self {
        Self {
            body,
            house_position,
            degree,
        }
    }
}

#[derive(Clone)]
pub struct ZodiacalCusp {
    pub cusp: Cusp,
}
impl ZodiacalCusp {
    pub fn new(cusp: Cusp) -> Self {
        Self {
            cusp
        }
    }
}

impl fmt::Debug for ZodiacalCusp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::swe2::split_deg2;

        f.debug_struct("ZodiacalAscMc")
         .field("first", & split_deg2(self.cusp.first,SplitDegKind::Zodiacal ))
         .field("second", & split_deg2(self.cusp.second,SplitDegKind::Zodiacal ))
         .field("third", & split_deg2(self.cusp.third,SplitDegKind::Zodiacal ))
         .field("fourth", & split_deg2(self.cusp.fourth,SplitDegKind::Zodiacal ))
         .field("fifth", & split_deg2(self.cusp.fifth,SplitDegKind::Zodiacal ))
         .field("sixth", & split_deg2(self.cusp.sixth,SplitDegKind::Zodiacal ))
         .field("seventh", & split_deg2(self.cusp.seventh,SplitDegKind::Zodiacal ))
         .field("eighth", & split_deg2(self.cusp.eighth,SplitDegKind::Zodiacal ))
         .field("ninth", & split_deg2(self.cusp.ninth,SplitDegKind::Zodiacal ))
         .field("tenth", & split_deg2(self.cusp.tenth,SplitDegKind::Zodiacal ))
         .field("eleventh", & split_deg2(self.cusp.eleventh,SplitDegKind::Zodiacal ))
         .field("twelfth", & split_deg2(self.cusp.twelfth,SplitDegKind::Zodiacal ))
         .finish()
    }
}

#[derive(Debug, Clone)]
pub struct ZodiacalInfo {
    pub houses: ZodiacalHouses,
    pub bodies: Vec<ZodiacalBody>,
}
impl ZodiacalInfo {
    pub fn new(houses: ZodiacalHouses, bodies: Vec<ZodiacalBody>) -> Self {
        Self {
            houses, 
            bodies,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ZodiacalHouses {
    pub asc_mc: ZodiacalAscMc,
    pub cusp: ZodiacalCusp,
}

impl ZodiacalHouses {
    pub fn new(asc_mc: ZodiacalAscMc, cusp: ZodiacalCusp) -> Self {
        Self {
            asc_mc,
            cusp,
        }
    }
}

#[derive(Clone)]
pub struct ZodiacalAscMc {
    pub asc_mc: AscMc,
}
impl ZodiacalAscMc {
    pub fn new(asc_mc: AscMc) -> Self {
        Self {
            asc_mc
        }
    }
}

#[derive(Debug, Clone)]
pub struct AscMc {
    pub ascendant: f64,
    pub mc: f64,
    pub armc: f64,
    pub vertex: f64,
    pub equatorial_ascendant: f64,
    pub co_ascendant_wk: f64, // walter_koch
    pub co_ascendant_mm: f64, // michael_munkasey
    pub polar_ascendant: f64,
}

impl AscMc {
    pub fn from_array(ascmc: [f64;10]) -> Self {
        let ascendant               = ascmc[SE_ASC as usize];
        let mc                      = ascmc[SE_MC as usize];
        let armc                    = ascmc[SE_ARMC as usize];
        let vertex                  = ascmc[SE_VERTEX as usize];
        let equatorial_ascendant    = ascmc[SE_EQUASC as usize];
        let co_ascendant_wk         = ascmc[SE_COASC1 as usize];
        let co_ascendant_mm         = ascmc[SE_COASC2 as usize];
        let polar_ascendant         = ascmc[SE_POLASC as usize];
        let _nascmc                  = ascmc[SE_NASCMC as usize];
        Self::new(
            ascendant, 
            mc, 
            armc, 
            vertex, 
            equatorial_ascendant, 
            co_ascendant_wk,
            co_ascendant_mm,
            polar_ascendant
        )
    }

    pub fn new(
        ascendant: f64,
        mc: f64,
        armc: f64,
        vertex: f64,
        equatorial_ascendant: f64,
        co_ascendant_wk: f64, // walter_koch
        co_ascendant_mm: f64, // michael_munkasey
        polar_ascendant: f64,
    ) -> Self {
        Self {
            ascendant,
            mc,
            armc,
            vertex,
            equatorial_ascendant,
            co_ascendant_wk,
            co_ascendant_mm,
            polar_ascendant,
        }

    }
}
