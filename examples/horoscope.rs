use std::fs;
use std::path::PathBuf;

use serde_derive::Deserialize;

use swisseph::*;
use swisseph::swe2::*;
use swisseph::HouseSystemKind::*;
use swisseph::Body::*;
use CalandarKind::*;

fn main() {
    // copy examples/config.toml.example to examples/config.toml to customize.
    // otherwise, New York's current day config will be used
    let config = get_config();

    let geolat = config.geolat;
    let geolon = config.geolon;

    let year = config.year;
    let month = config.month;
    let day = config.day;
    let hour = config.hour;
    let minute = config.minute;
    let second = config.second;

    let tjd = utc_to_jd2(year, month, day, hour, minute, second, Gregorian).unwrap();
    let tjd_ut = tjd.ut;

    let nutation = calc_ut2_ecliptic(tjd_ut, EclNut, Seflg::SPEED).unwrap();
    let eps = nutation.longitude;

    let h = houses2(tjd_ut, geolon, geolat, Placidus);
    let armc = h.1.armc; // should be in degrees
    let pp: Vec<ZodiacalBody> = Body::standard_bodies().iter().map(|b| {
        let planet_pos = calc_ut2_ecliptic(tjd_ut, b.clone(), Seflg::none()).unwrap();
        let planet_lon = planet_pos.longitude;
        let planet_lat = planet_pos.latitude;
        let hp = house_pos2(armc, geolon, eps, Placidus, planet_lon, planet_lat).unwrap();
        let body_deg = split_deg2_zodiacal(planet_lon, SplitDegKind::none());
        let zb = ZodiacalBody::new(b.clone(), hp, body_deg);

        zb
    }).collect();

    let z_asc_mc = ZodiacalAscMc::new(h.1.clone());
    let z_cusp = ZodiacalCusp::new(h.0);
    let zodiacal_house = ZodiacalHouses {asc_mc: z_asc_mc, cusp: z_cusp};

    let zodiacal_info = ZodiacalInfo::new(zodiacal_house, pp);

    let zi = zodiacal_info.to_text();
    let combined = [zi.0.clone(), zi.1.0.clone(), zi.1.1.clone()].concat();

    println!("{:#?}", combined);
}

fn get_config() -> Config {
    let mut config_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_file.push("examples/config.toml");

    let config = match fs::read_to_string(config_file) {
        Ok(c) => {
            match toml::from_str(&c) {
                Ok(d) => d,
                Err(_) => {
                    Config::default()
                }
            }
        },
        Err(_) => {
            Config::default()
        }
    };

    config
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
struct Config {
    geolat: f64,
    geolon: f64,

    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: f64,
}

impl Default for Config {
    fn default() -> Self { 

        // New York: 40.7128° N, 74.0060° W
        let geolat = -74.0060;
        let geolon = 40.7128;

        use chrono::Datelike;
        let current_date = chrono::Utc::now();
        let year = current_date.year();
        let month = current_date.month() as i32;
        let day = current_date.day() as i32;       

        let hour = 12;
        let minute = 0;
        let second = 0.;

        Config {geolat, geolon, year, month, day, hour, minute, second}
    }
}

