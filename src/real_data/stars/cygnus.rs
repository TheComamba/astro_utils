use astro_coords::ra_and_dec::*;
use uom::si::{f64::{Length, Mass, ThermodynamicTemperature, Time}, length::light_year, thermodynamic_temperature::kelvin};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

fn DENEB() -> RealData {
    RealData {
        common_name: "Deneb",
        astronomical_name: "α Cygni",
        constellation: "Cygnus",
        radius: Some(Length {
            m: 203. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 19. * SOLAR_MASS.kg,
        },
        absolute_magnitude: -7.13,
        apparent_magnitude: 1.25,
        temperature: Temperature { K: 8515. },
        right_ascension: RightAscension::new(20, 41, 26.),
        declination: Declination::new(Sgn::Pos, 45, 16, 49.),
        distance: Length {
            m: 1548. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.011 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.011037517 * BILLION_YEARS.s,
        },
    }
}

fn SADIR() -> RealData {
    RealData {
        common_name: "Sadir",
        astronomical_name: "γ Cygni",
        constellation: "Cygnus",
        radius: Some(Length {
            m: 150. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 12.11 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -6.12,
        apparent_magnitude: 2.23,
        temperature: Temperature { K: 5790. },
        age: Some(Time {
            s: 0.012 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.019450199 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(20, 22, 14.),
        declination: Declination::new(Sgn::Pos, 40, 15, 24.),
        distance: Length {
            m: 1522. * LIGHT_YEAR.m,
        },
    }
}

fn ALJANAH() -> RealData {
    RealData {
        common_name: "Aljanah",
        astronomical_name: "ε Cygni",
        constellation: "Cygnus",
        radius: Some(Length {
            m: 10.82 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2. * SOLAR_MASS.kg,
        },
        absolute_magnitude: 0.76,
        apparent_magnitude: 2.48,
        temperature: Temperature { K: 4710. },
        age: Some(Time {
            s: 1.3 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.36020165 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(20, 46, 13.),
        declination: Declination::new(Sgn::Pos, 33, 58, 13.),
        distance: Length {
            m: 72. * LIGHT_YEAR.m,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [DENEB, SADIR, ALJANAH];
