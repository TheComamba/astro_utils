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

const DENEB: RealData = RealData {
    common_name: "Deneb",
    astronomical_name: "Alpha Cygni",
    constellation: "Cygnus",
    radius: Some(Distance {
        m: 203. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 19. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -7.13,
    apparent_magnitude: 1.25,
    temperature: Some(Temperature { K: 8515. }),
    age: None,
    right_ascension: RightAscension::new(20, 41, 26),
    declination: Declination::new(Sgn::Pos, 45, 16, 49),
    distance: Distance {
        m: 1548. * LIGHT_YEAR.m,
    },
};

const SADIR: RealData = RealData {
    common_name: "Sadir",
    astronomical_name: "Gamma Cygni",
    constellation: "Cygnus",
    radius: Some(Distance {
        m: 150. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 12.11 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -6.12,
    apparent_magnitude: 2.23,
    temperature: Some(Temperature { K: 5790. }),
    age: Some(Time {
        s: 0.012 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(20, 22, 14),
    declination: Declination::new(Sgn::Pos, 40, 15, 24),
    distance: Distance {
        m: 1522. * LIGHT_YEAR.m,
    },
};

const ALJANAH: RealData = RealData {
    common_name: "Aljanah",
    astronomical_name: "Epsilon Cygni",
    constellation: "Cygnus",
    radius: Some(Distance {
        m: 10.82 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.76,
    apparent_magnitude: 2.48,
    temperature: Some(Temperature { K: 4710. }),
    age: Some(Time {
        s: 1.5 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(20, 46, 13),
    declination: Declination::new(Sgn::Pos, 33, 58, 13),
    distance: Distance {
        m: 72. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 3] = [DENEB, SADIR, ALJANAH];
