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

fn BETA_CAMELOPARDALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Camelopardalis",
        constellation: "Camelopardalis",
        radius: Some(Length {
            m: 58. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 6.5 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -3.1,
        apparent_magnitude: 4.02,
        temperature: Temperature { K: 5300. },
        right_ascension: RightAscension::new(5, 3, 25.),
        declination: Declination::new(Sgn::Pos, 60, 26, 32.),
        distance: Length {
            m: 870. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.053 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.063411557 * BILLION_YEARS.s,
        },
    }
}

fn CS_CAMELOPARDALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "CS Camelopardalis",
        constellation: "Camelopardalis",
        radius: Some(Length {
            m: 85.7 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 19. * SOLAR_MASS.kg,
        },
        absolute_magnitude: -6.39,
        apparent_magnitude: 4.21,
        temperature: Temperature { K: 10_800. },
        right_ascension: RightAscension::new(3, 29, 4.),
        declination: Declination::new(Sgn::Pos, 59, 56, 25.),
        distance: Length {
            m: 4289. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.011 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.011037517 * BILLION_YEARS.s,
        },
    }
}

fn ALPHA_CAMELOPARDALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Camelopardalis",
        constellation: "Camelopardalis",
        radius: Some(Length {
            m: 32.5 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 37.6 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -7.1,
        apparent_magnitude: 4.29,
        temperature: Temperature { K: 29_000. },
        right_ascension: RightAscension::new(4, 54, 3.),
        declination: Declination::new(Sgn::Pos, 66, 20, 34.),
        distance: Length {
            m: 6_000. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.002 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.005279908 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] =
    [BETA_CAMELOPARDALIS, CS_CAMELOPARDALIS, ALPHA_CAMELOPARDALIS];
