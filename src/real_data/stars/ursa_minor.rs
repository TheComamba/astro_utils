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

const POLARIS: RealData = RealData {
    common_name: "Polaris",
    astronomical_name: "Alpha Ursae Minoris",
    constellation: "Ursa Minor",
    radius: Some(Distance {
        m: 37.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 5.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.64,
    apparent_magnitude: 1.97,
    temperature: Some(Temperature { K: 6015. }),
    age: Some(Time {
        s: 0.05 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(2, 31, 49),
    declination: Declination::new(Sgn::Pos, 89, 15, 51),
    distance: Distance {
        m: 431. * LIGHT_YEAR.m,
    },
};

const KOCHAB: RealData = RealData {
    common_name: "Kochab",
    astronomical_name: "Beta Ursae Minoris",
    constellation: "Ursa Minor",
    radius: Some(Distance {
        m: 42.06 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.87,
    apparent_magnitude: 2.07,
    temperature: Some(Temperature { K: 4030. }),
    age: None,
    right_ascension: RightAscension::new(14, 50, 42),
    declination: Declination::new(Sgn::Pos, 74, 9, 20),
    distance: Distance {
        m: 126. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
