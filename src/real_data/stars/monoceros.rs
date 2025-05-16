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

fn ALPHA_MONOCEROTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Monocerotis",
        constellation: "Monoceros",
        right_ascension: RightAscension::new(7, 41, 15.),
        declination: Declination::new(Sgn::Neg, 9, 33, 4.),
        apparent_magnitude: 3.94,
        distance: Length {
            m: 148. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.71,
        mass: Mass {
            kg: 2.02 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 10.1 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4879. },
        age: Some(Time {
            s: 1.18 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.36020165 * BILLION_YEARS.s,
        },
    }
}

fn GAMMA_MONOCEROTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Monocerotis",
        constellation: "Monoceros",
        right_ascension: RightAscension::new(6, 14, 51.),
        declination: Declination::new(Sgn::Neg, 6, 16, 29.),
        apparent_magnitude: 3.96,
        distance: Length {
            m: 500. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -1.93,
        mass: Mass {
            kg: 1.2 * SOLAR_MASS.kg,
        },
        radius: None,
        temperature: Temperature { K: 4375. },
        age: None,
        lifetime: Time {
            s: 5.06543331 * BILLION_YEARS.s,
        },
    }
}

fn DELTA_MONOCEROTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Monocerotis",
        constellation: "Monoceros",
        right_ascension: RightAscension::new(7, 11, 52.),
        declination: Declination::new(Sgn::Neg, 0, 29, 34.),
        apparent_magnitude: 4.15,
        distance: Length {
            m: 384. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -1.20,
        mass: Mass {
            kg: 2.4 * SOLAR_MASS.kg,
        },
        radius: None,
        temperature: Temperature { K: 9462. },
        age: Some(Time {
            s: 0.405 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.800458342 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [ALPHA_MONOCEROTIS, GAMMA_MONOCEROTIS, DELTA_MONOCEROTIS];
