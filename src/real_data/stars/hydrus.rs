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

fn ALPHA_HYDRI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Hydri",
        constellation: "Hydrus",
        right_ascension: RightAscension::new(1, 58, 46.),
        declination: Declination::new(Sgn::Neg, 61, 34, 11.),
        apparent_magnitude: 2.9,
        distance: Length {
            m: 71.8 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 1.153,
        mass: Mass {
            kg: 2. * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 3.040 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 7087. },
        age: Some(Time {
            s: 0.810 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.36020165 * BILLION_YEARS.s,
        },
    }
}

fn BETA_HYDRI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Hydri",
        constellation: "Hydrus",
        right_ascension: RightAscension::new(0, 25, 45.),
        declination: Declination::new(Sgn::Neg, 77, 15, 15.),
        apparent_magnitude: 2.8,
        distance: Length {
            m: 24.33 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 3.45,
        mass: Mass {
            kg: 1.08 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 1.809 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 5872. },
        age: Some(Time {
            s: 6.4 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 6.97272616 * BILLION_YEARS.s,
        },
    }
}

fn GAMMA_HYDRI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Hydri",
        constellation: "Hydrus",
        radius: Some(Length {
            m: 62. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1. * SOLAR_MASS.kg,
        },
        absolute_magnitude: -0.83,
        apparent_magnitude: 3.26,
        temperature: Temperature { K: 3499. },
        right_ascension: RightAscension::new(3, 47, 14.),
        declination: Declination::new(Sgn::Neg, 74, 14, 20.),
        distance: Length {
            m: 214. * LIGHT_YEAR.m,
        },
        age: None,
        lifetime: Time {
            s: 9.81519157 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [ALPHA_HYDRI, BETA_HYDRI, GAMMA_HYDRI];
