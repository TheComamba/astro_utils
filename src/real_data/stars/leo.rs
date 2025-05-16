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

fn REGULUS() -> RealData {
    RealData {
        common_name: "Regulus",
        astronomical_name: "α Leonis",
        constellation: "Leo",
        radius: Some(Length {
            m: 4.35 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 3.8 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -0.52,
        apparent_magnitude: 1.36,
        temperature: Temperature { K: 11_668. },
        age: Some(Time {
            s: 0.100 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.220601963 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(10, 8, 22.),
        declination: Declination::new(Sgn::Pos, 11, 58, 2.),
        distance: Length {
            m: 77. * LIGHT_YEAR.m,
        },
    }
}

fn ALGIEBA() -> RealData {
    RealData {
        common_name: "Algieba",
        astronomical_name: "γ Leonis",
        constellation: "Leo",
        radius: Some(Length {
            m: 31.88 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.23 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -0.92,
        apparent_magnitude: 2.01,
        temperature: Temperature { K: 4470. },
        right_ascension: RightAscension::new(10, 19, 58.),
        declination: Declination::new(Sgn::Pos, 19, 50, 29.),
        distance: Length {
            m: 126. * LIGHT_YEAR.m,
        },
        age: None,
        lifetime: Time {
            s: 4.45521207 * BILLION_YEARS.s,
        },
    }
}

fn DENEBOLA() -> RealData {
    RealData {
        common_name: "Denebola",
        astronomical_name: "β Leonis",
        constellation: "Leo",
        radius: Some(Length {
            m: 1.728 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.78 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 1.92,
        apparent_magnitude: 2.14,
        temperature: Temperature { K: 8500. },
        age: Some(Time {
            s: 0.25 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.46605285 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(11, 49, 3.),
        declination: Declination::new(Sgn::Pos, 14, 34, 19.),
        distance: Length {
            m: 36. * LIGHT_YEAR.m,
        },
    }
}

fn ZOSMA() -> RealData {
    RealData {
        common_name: "Zosma",
        astronomical_name: "δ Leonis",
        constellation: "Leo",
        radius: Some(Length {
            m: 2.14 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.2 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 1.32,
        apparent_magnitude: 2.56,
        temperature: Temperature { K: 8_296. },
        age: Some(Time {
            s: 0.65 * BILLION_YEARS.s,
        }),
        right_ascension: RightAscension::new(11, 14, 7.),
        declination: Declination::new(Sgn::Pos, 20, 31, 25.),
        distance: Length {
            m: 58. * LIGHT_YEAR.m,
        },
        lifetime: Time {
            s: 1.03650581 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 4] = [REGULUS, ALGIEBA, DENEBOLA, ZOSMA];
