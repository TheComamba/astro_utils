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

fn UNUKALHAI() -> RealData {
    RealData {
        common_name: "Unukalhai",
        astronomical_name: "α Serpentis",
        constellation: "Serpens",
        radius: Some(Length {
            m: 13.48 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.66 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 0.88,
        apparent_magnitude: 2.63,
        temperature: Temperature { K: 4498. },
        age: None,
        right_ascension: RightAscension::new(15, 44, 16.),
        declination: Declination::new(Sgn::Pos, 6, 25, 32.),
        distance: Length {
            m: 74. * LIGHT_YEAR.m,
        },
        lifetime: Time {
            s: 1.89665739 * BILLION_YEARS.s,
        },
    }
}

fn BETA_SERPENTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Serpentis",
        constellation: "Serpens",
        right_ascension: RightAscension::new(15, 46, 11.),
        declination: Declination::new(Sgn::Pos, 15, 25, 19.),
        apparent_magnitude: 3.65,
        distance: Length {
            m: 155. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.30,
        mass: Mass {
            kg: 1.94 * SOLAR_MASS.kg,
        },
        radius: None,
        temperature: Temperature { K: 8928. },
        age: Some(Time {
            s: 0.267 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.46316038 * BILLION_YEARS.s,
        },
    }
}

fn MU_SERPENTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "μ Serpentis",
        constellation: "Serpens",
        right_ascension: RightAscension::new(15, 49, 37.),
        declination: Declination::new(Sgn::Neg, 3, 25, 49.),
        apparent_magnitude: 3.543,
        distance: Length {
            m: 170. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -0.04,
        mass: Mass {
            kg: 2.4 * SOLAR_MASS.kg,
        },
        radius: None,
        temperature: Temperature { K: 9487. },
        age: None,
        lifetime: Time {
            s: 0.800458342 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [UNUKALHAI, BETA_SERPENTIS, MU_SERPENTIS];
