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

fn ENIF() -> RealData {
    RealData {
        common_name: "Enif",
        astronomical_name: "ε Pegasi",
        constellation: "Pegasus",
        radius: Some(Length {
            m: 211. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 7.07 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -4.19,
        apparent_magnitude: 2.38,
        temperature: Temperature { K: 3963. },
        age: Some(Time {
            s: 0.020 * BILLION_YEARS.s,
        }),
        right_ascension: RightAscension::new(21, 44, 11.),
        declination: Declination::new(Sgn::Pos, 9, 52, 30.),
        distance: Length {
            m: 672. * LIGHT_YEAR.m,
        },
        lifetime: Time {
            s: 0.052267043 * BILLION_YEARS.s,
        },
    }
}

fn SCHEAT() -> RealData {
    RealData {
        common_name: "Scheat",
        astronomical_name: "β Pegasi",
        constellation: "Pegasus",
        radius: Some(Length {
            m: 95. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.1 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -1.49,
        apparent_magnitude: 2.44,
        temperature: Temperature { K: 3689. },
        age: None,
        right_ascension: RightAscension::new(23, 3, 46.),
        declination: Declination::new(Sgn::Pos, 28, 4, 58.),
        distance: Length {
            m: 199. * LIGHT_YEAR.m,
        },
        lifetime: Time {
            s: 1.17901142 * BILLION_YEARS.s,
        },
    }
}

fn MARKAB() -> RealData {
    RealData {
        common_name: "Markab",
        astronomical_name: "α Pegasi",
        constellation: "Pegasus",
        radius: Some(Length {
            m: 4.62 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 3.5 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -0.67,
        apparent_magnitude: 2.49,
        temperature: Temperature { K: 10_100. },
        age: Some(Time {
            s: 0.2 * BILLION_YEARS.s,
        }),
        right_ascension: RightAscension::new(23, 4, 46.),
        declination: Declination::new(Sgn::Pos, 15, 12, 19.),
        distance: Length {
            m: 140. * LIGHT_YEAR.m,
        },
        lifetime: Time {
            s: 0.297402042 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [ENIF, SCHEAT, MARKAB];
