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

impl EquatorialPosition {
    pub fn with_array(a: [f64; 6]) -> Self {
        Self::new(a[0], a[1], a[2], a[3], a[4], a[5])
    }

    pub fn new(
        right_ascension: f64,
        declination: f64,
        distance_in_au: f64,
        speed_in_right_ascension: f64,
        speed_in_declination: f64,
        speed_in_distance: f64,
    ) -> Self {
        Self {
            right_ascension,
            declination,
            distance_in_au,
            speed_in_right_ascension,
            speed_in_declination,
            speed_in_distance,
        }
    }
}
