use libswisseph_sys::*;
use strum_macros::{EnumString, EnumIter, FromRepr};

#[derive(Debug, Clone, EnumIter, EnumString, FromRepr, Default)]
#[repr(isize)]
pub enum Body {
    #[default]
    EclNut              = SE_ECL_NUT        as isize,
    Sun                 = SE_SUN            as isize,          
    Moon                = SE_MOON           as isize,          
    Mercury             = SE_MERCURY        as isize,          
    Venus               = SE_VENUS          as isize,          
    Mars                = SE_MARS           as isize,          
    Jupiter             = SE_JUPITER        as isize,          
    Saturn              = SE_SATURN         as isize,          
    Uranus              = SE_URANUS         as isize,          
    Neptune             = SE_NEPTUNE        as isize,          
    Pluto               = SE_PLUTO          as isize,          
    MeanNode            = SE_MEAN_NODE      as isize,          
    TrueNode            = SE_TRUE_NODE      as isize,          
    MeanApog            = SE_MEAN_APOG      as isize,          
    OscuApog            = SE_OSCU_APOG      as isize,          
    Earth               = SE_EARTH          as isize,          
    Chiron              = SE_CHIRON         as isize,          
    Pholus              = SE_PHOLUS         as isize,          
    Ceres               = SE_CERES          as isize,          
    Pallas              = SE_PALLAS         as isize,          
    Juno                = SE_JUNO           as isize,          
    Vesta               = SE_VESTA          as isize,          
    IntpApog            = SE_INTP_APOG      as isize,          
    IntpPerg            = SE_INTP_PERG      as isize,          
    //NPlanets            = SE_NPLANETS       as isize,          
    //FictOffset      = SE_FICT_OFFSET    as isize,          
    //NFictElem       = SE_NFICT_ELEM     as isize,          
    //PLMoonOffset        = SE_PLMOON_OFFSET  as isize,
    //AstOffset           = SE_AST_OFFSET     as isize,      

    // Hamburger or Uranian "planets" 
    Cupido              =  SE_CUPIDO           as isize,   
    Hades               =  SE_HADES            as isize,
    Zeus                =  SE_ZEUS             as isize,
    Kronos              =  SE_KRONOS           as isize,
    Apollon             =  SE_APOLLON          as isize,
    Admetos             =  SE_ADMETOS          as isize,
    Vulkanus            =  SE_VULKANUS         as isize,
    Poseidon            =  SE_POSEIDON         as isize,

    // other fictitious bodies
    Isis                =   SE_ISIS              as isize, 
    Nibiru              =   SE_NIBIRU            as isize, 
    Harrington          =   SE_HARRINGTON        as isize, 
    NeptuneLeverrier    =   SE_NEPTUNE_LEVERRIER as isize, 
    NeptuneAdams        =   SE_NEPTUNE_ADAMS     as isize, 
    PlutoLowell         =   SE_PLUTO_LOWELL      as isize, 
    PlutoPickering      =   SE_PLUTO_PICKERING   as isize, 

    //Body numbers of other asteroids are above SE_AST_OFFSET (= 10000) 
    //and have to be constructed as follows:
    //ipl = SE_AST_OFFSET + minor_planet_catalogue_number;
    //The names of the asteroids and their catalogue numbers can be found in seasnam.txt.
    Astraea             = (SE_AST_OFFSET + 5   ) as isize,
    Hebe                = (SE_AST_OFFSET + 6   ) as isize,  
    Iris                = (SE_AST_OFFSET + 7   ) as isize,  
    Flora               = (SE_AST_OFFSET + 8   ) as isize,  
    Metis               = (SE_AST_OFFSET + 9   ) as isize,  
    Hygiea              = (SE_AST_OFFSET + 10  ) as isize,  
    Urania              = (SE_AST_OFFSET + 30  ) as isize,  
    IsisAstroid         = (SE_AST_OFFSET + 42  ) as isize,  
    Hilda               = (SE_AST_OFFSET + 153 ) as isize,  
    Philosophia         = (SE_AST_OFFSET + 227 ) as isize,  
    Sophia              = (SE_AST_OFFSET + 251 ) as isize,  
    Aletheia            = (SE_AST_OFFSET + 259 ) as isize,  
    Sapientia           = (SE_AST_OFFSET + 275 ) as isize,  
    Thule               = (SE_AST_OFFSET + 279 ) as isize,  
    Ursula              = (SE_AST_OFFSET + 375 ) as isize,  
    Eros                = (SE_AST_OFFSET + 433 ) as isize,  
    CupidoAstroid       = (SE_AST_OFFSET + 763 ) as isize,  
    Hidalgo             = (SE_AST_OFFSET + 944 ) as isize,  
    Lilith              = (SE_AST_OFFSET + 1181) as isize,  
    Amor                = (SE_AST_OFFSET + 1221) as isize,  
    Kama                = (SE_AST_OFFSET + 1387) as isize,  
    Aphrodite           = (SE_AST_OFFSET + 1388) as isize,  
    Apollo              = (SE_AST_OFFSET + 1862) as isize,  
    Damocles            = (SE_AST_OFFSET + 3553) as isize,  
    Cruithne            = (SE_AST_OFFSET + 3753) as isize,  
    PoseidonAstroid     = (SE_AST_OFFSET + 4341)  as isize,   
    Vulcano             = (SE_AST_OFFSET + 4464)  as isize,   
    ZeusAstroid         = (SE_AST_OFFSET + 5731)  as isize,   
    Nessus              = (SE_AST_OFFSET + 7066)  as isize,   

    // TODO: 
    // Their Swiss Ephemeris body numbers are between SE_PLMOON_OFFSET (= 9000) 
    // and SE_AST_OFFSET (= 10000) and are constructed as follows: 

    //ipl = SE_PLMOON_OFFSET + planet_number * 100 + moon number in JPL Horizons;
    //e.g., Jupiter moon Io: ipl = SE_PLMOON_OFFSET + SE_JUPITER (= 5) * 100 + 1 (= 9501).
    //Centers of body (COB) are calculated the same way, i.e. like a planetary moon but with the “moon number” 99;
    //e.g. Jupiter center of body: ipl = SE_PLMOON_OFFSET + SE_JUPITER * 100 + 99 (= 9599)
    //Moons of Mars:      9401 – 9402
    //Moons of Jupiter:   9501 – 95xx;        Center of body: 9599
    //Moons of Saturn:    9601 – 96xx;        Center of body: 9699
    //Moons of Uranus:    9701 – 97xx;        Center of body: 9799
    //Moons of Neptune:   9801 – 98xx;        Center of body: 9899
    //Moons of Pluto:     9901 – 99xx;        Center of body: 9999

    // A full list of existing planetary moons is found here: 
    // https://en.wikipedia.org/wiki/List_of_natural_satellites .

    // The ephemeris files of the planetary moons and COB are in the subdirectory sat. 
    // Like the subdirectories of asteroids, the directory sat must be created in the 
    // path which is defined using the function swe_set_ephe_path().
    // The ephemeris files can be downloaded from here:
    // https://www.astro.com/ftp/swisseph/ephe/sat/.
    // The list of objects available in the Swiss Ephemeris is:
    //9401  Phobos/Mars
    //9402  Deimos/Mars
    //9501  Io/Jupiter
    //9502  Europa/Jupiter
    //9503  Ganymede/Jupiter
    //9504  Callisto/Jupiter
    //9599  Jupiter/COB
    //9601  Mimas/Saturn
    //9602  Enceladus/Saturn
    //9603  Tethys/Saturn
    //9604  Dione/Saturn
    //9605  Rhea/Saturn
    //9606  Titan/Saturn
    //9607  Hyperion/Saturn
    //9608  Iapetus/Saturn
    //9699  Saturn/COB
    //9701  Ariel/Uranus
    //9702  Umbriel/Uranus
    //9703  Titania/Uranus
    //9704  Oberon/Uranus
    //9705  Miranda/Uranus
    //9799  Uranus/COB
    //9801  Triton/Neptune
    //9802  Triton/Nereid
    //9808  Proteus/Neptune
    //9899  Neptune/COB
    //9901  Charon/Pluto
    //9902  Nix/Pluto
    //9903  Hydra/Pluto
    //9904  Kerberos/Pluto
    //9905  Styx/Pluto
    //9999  Pluto/COB
}

impl Body {
    pub fn standard_bodies() -> Vec<Body> {
        let b: Vec<_> = (SE_SUN..SE_TRUE_NODE).collect();

        let bodies: Vec<Body> = b.iter().map(|x| {
            Body::from_repr(*x as isize).unwrap()
        }).collect();

        bodies
    }

    pub fn standard_bodies_with_chiron() -> Vec<Body> {
        let mut b: Vec<_> = (SE_SUN..SE_TRUE_NODE).collect();
        b.push(SE_CHIRON);

        let bodies: Vec<Body> = b.iter().map(|x| {
            Body::from_repr(*x as isize).unwrap()
        }).collect();

        bodies
    }

    pub fn to_text(&self) -> String {
        crate::body_emoji::body_emoji(self).clone()
    }

    pub fn to_emoji(&self) -> String {
        crate::body_emoji::body_emoji(self).clone()
    }
}

