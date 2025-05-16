use astro_coords::ra_and_dec::*;
use simple_si_units::base::{Length, Mass, Temperature, Time};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

fn BETA_TRIANGULI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Trianguli",
        constellation: "Triangulum",
        right_ascension: RightAscension::new(2, 9, 33.),
        declination: Declination::new(Sgn::Pos, 34, 59, 14.),
        apparent_magnitude: 3.,
        distance: Length {
            m: 127. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.05,
        mass: Mass {
            kg: 3.5 * SOLAR_MASS.kg,
        },
        radius: None,
        temperature: Temperature { K: 8186. },
        age: Some(Time {
            s: 0.29 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.297402042 * BILLION_YEARS.s,
        },
    }
}

fn ALPHA_TRIANGULI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Trianguli",
        constellation: "Triangulum",
        right_ascension: RightAscension::new(1, 53, 5.),
        declination: Declination::new(Sgn::Pos, 29, 34, 44.),
        apparent_magnitude: 3.42,
        distance: Length {
            m: 63.3 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 1.98,
        mass: Mass {
            kg: 1.70 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 3.22 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 6288. },
        age: Some(Time {
            s: 1.6 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.73766023 * BILLION_YEARS.s,
        },
    }
}

fn GAMMA_TRIANGULI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Trianguli",
        constellation: "Triangulum",
        right_ascension: RightAscension::new(2, 17, 19.),
        declination: Declination::new(Sgn::Pos, 33, 50, 50.),
        apparent_magnitude: 4.01,
        distance: Length {
            m: 112.3 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 1.35,
        mass: Mass {
            kg: 2.7 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 1.96 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 9440. },
        age: Some(Time {
            s: 0.3 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.63513384 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [BETA_TRIANGULI, ALPHA_TRIANGULI, GAMMA_TRIANGULI];
