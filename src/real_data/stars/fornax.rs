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

fn DALIM() -> RealData {
    RealData {
        common_name: "Dalim",
        astronomical_name: "α Fornacis",
        constellation: "Fornax",
        right_ascension: RightAscension::new(3, 12, 5.),
        declination: Declination::new(Sgn::Neg, 28, 59, 15.),
        apparent_magnitude: 3.85,
        distance: Length {
            m: 45.66 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 3.08,
        mass: Mass {
            kg: 1.33 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 2.04 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 6240. },
        age: Some(Time {
            s: 2.9 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 3.46068223 * BILLION_YEARS.s,
        },
    }
}

fn BETA_FORNACIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Fornacis",
        constellation: "Fornax",
        right_ascension: RightAscension::new(2, 49, 5.),
        declination: Declination::new(Sgn::Neg, 32, 24, 21.),
        apparent_magnitude: 4.46,
        distance: Length {
            m: 178. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.894,
        mass: Mass {
            kg: 1.53 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 11.02 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4820. },
        age: None,
        lifetime: Time {
            s: 2.29668629 * BILLION_YEARS.s,
        },
    }
}

fn NU_FORNACIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ν Fornacis",
        constellation: "Fornax",
        right_ascension: RightAscension::new(2, 4, 29.),
        declination: Declination::new(Sgn::Neg, 29, 17, 49.),
        apparent_magnitude: 4.69,
        distance: Length {
            m: 370. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -0.6,
        mass: Mass {
            kg: 3.65 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 3.44 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 13_400. },
        age: None,
        lifetime: Time {
            s: 0.254814649 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [DALIM, BETA_FORNACIS, NU_FORNACIS];
