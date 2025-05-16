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

fn ALKES() -> RealData {
    RealData {
        common_name: "Alkes",
        astronomical_name: "α Crateris",
        constellation: "Crater",
        right_ascension: RightAscension::new(10, 59, 46.),
        declination: Declination::new(Sgn::Neg, 18, 17, 56.),
        apparent_magnitude: 4.08,
        distance: Length {
            m: 174.2 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.44,
        mass: Mass {
            kg: 1.81 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 12.32 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4691. },
        age: Some(Time {
            s: 1.4 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.46605285 * BILLION_YEARS.s,
        },
    }
}

fn BETA_CRATERIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Crateris",
        constellation: "Crater",
        right_ascension: RightAscension::new(11, 11, 39.),
        declination: Declination::new(Sgn::Neg, 22, 49, 33.),
        apparent_magnitude: 4.46,
        distance: Length {
            m: 296. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -0.62,
        mass: Mass {
            kg: 2.6 * SOLAR_MASS.kg,
        },
        radius: None,
        temperature: Temperature { K: 8830. },
        age: None,
        lifetime: Time {
            s: 0.63513384 * BILLION_YEARS.s,
        },
    }
}

fn GAMMA_CRATERIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Crateris",
        constellation: "Crater",
        right_ascension: RightAscension::new(11, 24, 53.),
        declination: Declination::new(Sgn::Neg, 17, 41, 2.),
        apparent_magnitude: 4.06,
        distance: Length {
            m: 85.6 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 2.05,
        mass: Mass {
            kg: 1.81 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 1.3 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 8020. },
        age: Some(Time {
            s: 0.757 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.46605285 * BILLION_YEARS.s,
        },
    }
}

fn DELTA_CRATERIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Crateris",
        constellation: "Crater",
        right_ascension: RightAscension::new(11, 19, 20.),
        declination: Declination::new(Sgn::Neg, 14, 46, 42.),
        apparent_magnitude: 3.56,
        distance: Length {
            m: 194.6 * LIGHT_YEAR.m,
        },
        absolute_magnitude: -0.321,
        mass: Mass {
            kg: 1.56 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 22.44 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4510. },
        age: Some(Time {
            s: 2.2 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 2.29668629 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 4] = [ALKES, BETA_CRATERIS, GAMMA_CRATERIS, DELTA_CRATERIS];
