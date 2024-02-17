use crate::{
    coordinates::{
        declination::{Declination, Sgn},
        right_ascension::RightAscension,
    },
    stars::real_data::RealData,
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};
use simple_si_units::base::{Distance, Mass, Temperature, Time};

const ALDEBARAN: RealData = RealData {
    common_name: "Aldebaran",
    astronomical_name: "Alpha Tauri",
    constellation: "Taurus",
    radius: Some(Distance {
        m: 45.1 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.16 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.63,
    apparent_magnitude: 0.87,
    temperature: Some(Temperature { K: 3900. }),
    age: Some(Time {
        s: 6.4 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 35, 55),
    declination: Declination::new(Sgn::Pos, 16, 30, 33),
    distance: Distance {
        m: 65. * LIGHT_YEAR.m,
    },
};

const ALNATH: RealData = RealData {
    common_name: "Alnath",
    astronomical_name: "Beta Tauri",
    constellation: "Taurus",
    radius: Some(Distance {
        m: 4.2 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 5.0 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.37,
    apparent_magnitude: 1.65,
    temperature: Some(Temperature { K: 13_824. }),
    age: Some(Time {
        s: 0.1 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 26, 18),
    declination: Declination::new(Sgn::Pos, 28, 36, 27),
    distance: Distance {
        m: 131. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
