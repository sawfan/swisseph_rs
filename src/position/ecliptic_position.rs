
#[derive(Debug)]
pub struct EclipticPosition {
    pub longitude: f64,
    pub latitude: f64,
    pub distance_in_au: f64,
    pub speed_in_longitude: f64, // (deg/day)
    pub speed_in_latitude: f64,  // (deg/day)
    pub speed_in_distance: f64,  // (AU/day)
}

impl EclipticPosition {

    pub fn with_array(
        a: [f64; 6]
    ) -> Self {
        Self::new(a[0], a[1], a[2], a[3], a[4], a[5])
    }

    pub fn new(
        longitude: f64,
        latitude: f64,
        distance_in_au: f64,
        speed_in_longitude: f64, // (deg/day)
        speed_in_latitude: f64,  // (deg/day)
        speed_in_distance: f64,  // (AU/day)
    ) -> Self {
        Self {
            longitude,
            latitude,
            distance_in_au,
            speed_in_longitude, 
            speed_in_latitude,  
            speed_in_distance,  
        }
    }

}
