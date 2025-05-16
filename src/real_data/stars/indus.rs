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

fn ALPHA_INDI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Indi",
        constellation: "Indus",
        right_ascension: RightAscension::new(20, 37, 34.),
        declination: Declination::new(Sgn::Neg, 47, 17, 29.),
        apparent_magnitude: 3.11,
        distance: Length {
            m: 98.3 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.65,
        mass: Mass {
            kg: 2.0 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 12. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4893. },
        age: Some(Time {
            s: 1. * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.36020165 * BILLION_YEARS.s,
        },
    }
}

fn BETA_INDI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Indi",
        constellation: "Indus",
        right_ascension: RightAscension::new(20, 54, 49.),
        declination: Declination::new(Sgn::Neg, 58, 27, 15.),
        apparent_magnitude: 3.67,
        distance: Length {
            m: 600. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -2.664,
        mass: Mass {
            kg: 6.7 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 55.58 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4541. },
        age: Some(Time {
            s: 0.0532 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.063411557 * BILLION_YEARS.s,
        },
    }
}

fn ETA_INDI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "η Indi",
        constellation: "Indus",
        right_ascension: RightAscension::new(20, 44, 2.),
        declination: Declination::new(Sgn::Neg, 51, 55, 15.),
        apparent_magnitude: 4.52,
        distance: Length {
            m: 78.8 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 2.59,
        mass: Mass {
            kg: 1.6 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 2.27 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 7694. },
        age: Some(Time {
            s: 0.1 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 2.08398753 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [ALPHA_INDI, BETA_INDI, ETA_INDI];
