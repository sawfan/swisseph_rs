use std::fs;
use std::path::PathBuf;

use chrono::Timelike;
use serde_derive::Deserialize;

use crate::zodiacal::*;
use swisseph::swe2::*;
use swisseph::Body::*;
use swisseph::HouseSystemKind::*;
use swisseph::*;
use CalandarKind::*;

// This will return the current time's horoscope for New York unless a config.toml is present, in
// which case the custom configuration will be used.
fn main() {
    // Copy examples/config.toml.example to examples/config.toml to customize.
    // Otherwise, New York's current config will be used.
    let config = get_config();

    let geolat = config.geolat;
    let geolon = config.geolon;

    let year = config.year;
    let month = config.month;
    let day = config.day;
    let hour = config.hour;
    let minute = config.minute;
    let second = config.second;
    let timezone = config.timezone;

    // let ephe_path = "/users/ephe";
    // swe::set_ephe_path(ephe_path);

    // Convert local civil time + timezone offset to UT, then to Julian Day.
    let ut_time = utc_time_zone2(year, month, day, hour, minute, second, timezone);
    let tjd = utc_to_jd2(
        ut_time.iyear_out,
        ut_time.imonth_out,
        ut_time.iday_out,
        ut_time.ihour_out,
        ut_time.imin_out,
        ut_time.dsec_out,
        Gregorian,
    )
    .unwrap();
    let tjd_ut = tjd.ut;

    let nutation = calc_ut2_ecliptic(tjd_ut, EclNut, Seflg::SPEED).unwrap();
    let eps = nutation.longitude;

    // Use (latitude, longitude) ordering for houses2 and house_pos2.
    let h = houses2(tjd_ut, geolat, geolon, Placidus);
    let armc = h.1.armc; // right ascension of MC in degrees

    let pp: Vec<ZodiacalBody> = Body::standard_bodies()
        .iter()
        .map(|b| {
            let planet_pos = calc_ut2_ecliptic(tjd_ut, b.clone(), Seflg::none()).unwrap();
            let planet_lon = planet_pos.longitude;
            let planet_lat = planet_pos.latitude;

            let hp = house_pos2(armc, geolat, eps, Placidus, planet_lon, planet_lat).unwrap();
            let body_deg = split_deg2_zodiacal(planet_lon, SplitDegKind::none());
            ZodiacalBody::new(b.clone(), hp, body_deg)
        })
        .collect();

    let z_asc_mc = ZodiacalAscMc::new(h.1.clone());
    let z_cusp = ZodiacalCusp::new(h.0);
    let zodiacal_house = ZodiacalHouses {
        asc_mc: z_asc_mc,
        cusp: z_cusp,
    };

    let zodiacal_info = ZodiacalInfo::new(zodiacal_house, pp);

    let zi = zodiacal_info.to_text();
    let combined = [zi.0.clone(), zi.1 .0.clone(), zi.1 .1.clone()].concat();

    println!("{:#?}", combined);

    swe::close();
}

fn get_config() -> Config {
    let mut config_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_file.push("examples/config.toml");

    let config = match fs::read_to_string(config_file) {
        Ok(c) => match toml::from_str(&c) {
            Ok(d) => {
                println!("Overriding defaults using config.toml");
                d
            }
            Err(_) => {
                println!("Error reading from config.toml, falling back to default");
                Config::default()
            }
        },
        Err(_) => Config::default(),
    };

    config
}

// Config struct holds the data from the `[config]` section.
fn default_timezone() -> f64 {
    0.0 // Greenwich
}

#[derive(Deserialize, Debug)]
struct Config {
    geolat: f64,
    geolon: f64,

    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: f64,

    // Timezone offset from UTC in hours (e.g. -5.0 for UTC-5)
    #[serde(default = "default_timezone")]
    timezone: f64,
}

impl Default for Config {
    fn default() -> Self {
        // New York: 40.7128° N, 74.0060° W
        let geolat = 40.43;
        let geolon = -74.00;

        use chrono::Datelike;
        let current_date = chrono::Utc::now();
        let year = current_date.year();
        let month = current_date.month() as i32;
        let day = current_date.day() as i32;
        let hour = current_date.hour() as i32;
        let minute = current_date.minute() as i32;
        let second = current_date.second() as f64;
        let timezone = default_timezone();

        Config {
            geolat,
            geolon,
            year,
            month,
            day,
            hour,
            minute,
            second,
            timezone,
        }
    }
}
