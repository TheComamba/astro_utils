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

fn GAMMA_MICROSCOPII() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Microscopii",
        constellation: "Microscopium",
        right_ascension: RightAscension::new(21, 1, 17.),
        declination: Declination::new(Sgn::Neg, 32, 15, 28.),
        apparent_magnitude: 4.680,
        distance: Length {
            m: 223. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.49,
        mass: Mass {
            kg: 2.5 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 10. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 5050. },
        age: Some(Time {
            s: 0.620 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.800458342 * BILLION_YEARS.s,
        },
    }
}

fn EPSILON_MICROSCOPII() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Microscopii",
        constellation: "Microscopium",
        right_ascension: RightAscension::new(21, 17, 56.),
        declination: Declination::new(Sgn::Neg, 32, 10, 21.),
        apparent_magnitude: 4.71,
        distance: Length {
            m: 166. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.97,
        mass: Mass {
            kg: 2.18 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 2.2 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 9126. },
        age: Some(Time {
            s: 0.525 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.03650581 * BILLION_YEARS.s,
        },
    }
}

fn THETA1_MICROSCOPII() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "θ¹ Microscopii",
        constellation: "Microscopium",
        right_ascension: RightAscension::new(21, 20, 46.),
        declination: Declination::new(Sgn::Neg, 40, 48, 34.),
        apparent_magnitude: 4.82,
        distance: Length {
            m: 179. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 1.03,
        mass: Mass {
            kg: 2.32 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 2.35 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 9240. },
        age: Some(Time {
            s: 0.437 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.916355612 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] =
    [GAMMA_MICROSCOPII, EPSILON_MICROSCOPII, THETA1_MICROSCOPII];
