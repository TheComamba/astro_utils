use astro_coords::ra_and_dec::*;

use crate::{
    stars::real_data::RealData,
    units::{
        length::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

fn ALPHA_ANTLIAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Antliae",
        constellation: "Antlia",
        radius: Some(Length {
            m: 41. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.2 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -0.973,
        apparent_magnitude: 4.28,
        temperature: Temperature { K: 4070. },
        age: None,
        lifetime: Time {
            s: 1.03650581 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(10, 27, 9.),
        declination: Declination::new(Sgn::Neg, 31, 4, 4.),
        distance: Length {
            m: 366.3 * LIGHT_YEAR.m,
        },
    }
}

fn EPSILON_ANTLIAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Antliae",
        constellation: "Antlia",
        radius: Some(Length {
            m: 56.3 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2. * SOLAR_MASS.kg,
        },
        absolute_magnitude: -2.15,
        apparent_magnitude: 4.51,
        temperature: Temperature { K: 4237. },
        age: None,
        lifetime: Time {
            s: 1.36020165 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(9, 29, 15.),
        declination: Declination::new(Sgn::Neg, 35, 57, 5.),
        distance: Length {
            m: 699.6 * LIGHT_YEAR.m,
        },
    }
}

fn IOTA_ANTLIAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ι Antliae",
        constellation: "Antlia",
        radius: Some(Length {
            m: 12.1 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.55 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 0.674,
        apparent_magnitude: 4.60,
        temperature: Temperature { K: 4892. },
        age: Some(Time {
            s: 2.2 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 2.29668629 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(10, 56, 43.),
        declination: Declination::new(Sgn::Neg, 37, 8, 16.),
        distance: Length {
            m: 198.8 * LIGHT_YEAR.m,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [ALPHA_ANTLIAE, EPSILON_ANTLIAE, IOTA_ANTLIAE];
