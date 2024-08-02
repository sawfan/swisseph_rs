#[derive(Debug, Default, Clone)]
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
    
    #[default]
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

