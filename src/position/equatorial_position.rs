
// SEFLG_EQUATORIAL
#[derive(Debug)]
pub struct EquatorialPosition {
    pub right_ascension: f64,
    pub declination: f64,
    pub distance_in_au: f64,
    pub speed_in_right_ascension: f64, // (deg/day)
    pub speed_in_declination: f64,     // (deg/day)
    pub speed_in_distance: f64,        // (AU/day)
}

