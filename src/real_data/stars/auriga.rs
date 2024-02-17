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

const CAPELLA: RealData = RealData {
    common_name: "Capella",
    astronomical_name: "Alpha Aurigae",
    constellation: "Auriga",
    radius: Some(Distance {
        m: 11.98 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.5687 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.48,
    apparent_magnitude: 0.08,
    temperature: Some(Temperature { K: 4970. }),
    age: Some(Time {
        s: 0.620 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 16, 41),
    declination: Declination::new(Sgn::Pos, 45, 59, 53),
    distance: Distance {
        m: 42. * LIGHT_YEAR.m,
    },
};

const MENKALINAN: RealData = RealData {
    common_name: "Menkalinan",
    astronomical_name: "Beta Aurigae",
    constellation: "Auriga",
    radius: Some(Distance {
        m: 2.77 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.389 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.10,
    apparent_magnitude: 1.9,
    temperature: Some(Temperature { K: 9350. }),
    age: Some(Time {
        s: 0.570 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 59, 32),
    declination: Declination::new(Sgn::Pos, 44, 56, 51),
    distance: Distance {
        m: 82. * LIGHT_YEAR.m,
    },
};

const HASSALEH: RealData = RealData {
    common_name: "Hassaleh",
    astronomical_name: "Iota Aurigae",
    constellation: "Auriga",
    radius: Some(Distance {
        m: 127. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7.1 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.20,
    apparent_magnitude: 2.69,
    temperature: Some(Temperature { K: 4160. }),
    age: Some(Time {
        s: 0.04 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 56, 60),
    declination: Declination::new(Sgn::Pos, 33, 9, 58),
    distance: Distance {
        m: 490. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 3] = [CAPELLA, MENKALINAN, HASSALEH];
