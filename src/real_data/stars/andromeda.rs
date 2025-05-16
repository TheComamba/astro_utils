use astro_coords::ra_and_dec::*;
use uom::si::{f64::{Length, Mass, ThermodynamicTemperature, Time}, length::light_year, thermodynamic_temperature::kelvin};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{solar_radii, LIGHT_YEAR, SOLAR_RADIUS},
        mass::{solar_mass, SOLAR_MASS},
        time::{gigayear, BILLION_YEARS},
    },
};

fn ALPHERATZ() -> RealData {
    RealData {
        common_name: "Alpheratz",
        astronomical_name: "α Andromedae",
        constellation: "Andromeda",
        radius: Some(Length::new::<solar_radii>(2.7)),
        mass: Mass::new::<solar_mass>(3.8),
        absolute_magnitude: -0.30,
        apparent_magnitude: 2.07,
        temperature: ThermodynamicTemperature::new::<kelvin>(13_800.),
        age: Some(Time::new::<gigayear>(0.06)),
        lifetime: Time::new::<gigayear>(0.220601963),
        right_ascension: RightAscension::new(0, 8, 23.),
        declination: Declination::new(Sgn::Pos, 29, 5, 26.),
        distance: Length::new::<light_year>(97.),
    }
}

fn MIRACH() -> RealData {
    RealData {
        common_name: "Mirach",
        astronomical_name: "β Andromedae",
        constellation: "Andromeda",
        radius: Some(Length {
            m: 100. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.49 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -1.86,
        apparent_magnitude: 2.07,
        temperature: Temperature { K: 3842. },
        age: None,
        lifetime: Time {
            s: 0.800458342 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(1, 9, 44.),
        declination: Declination::new(Sgn::Pos, 35, 37, 14.),
        distance: Length {
            m: 199. * LIGHT_YEAR.m,
        },
    }
}

fn ALMACH() -> RealData {
    RealData {
        common_name: "Almach",
        astronomical_name: "γ Andromedae",
        constellation: "Andromeda",
        radius: Some(Length {
            m: 80. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 23.7 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -3.08,
        apparent_magnitude: 2.1,
        temperature: Temperature { K: 4250. },
        age: Some(Time {
            s: 0.0065 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.008063854 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(2, 3, 54.),
        declination: Declination::new(Sgn::Pos, 42, 19, 47.),
        distance: Length {
            m: 355. * LIGHT_YEAR.m,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [ALPHERATZ, MIRACH, ALMACH];
