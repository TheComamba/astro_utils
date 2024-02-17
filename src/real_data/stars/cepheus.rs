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

const ALDERAMIN: RealData = RealData {
    common_name: "Alderamin",
    astronomical_name: "Alpha Cephei",
    constellation: "Cepheus",
    radius: Some(Distance {
        m: 2.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.58,
    apparent_magnitude: 2.45,
    temperature: Some(Temperature { K: 7700. }),
    age: Some(Time {
        s: 0.82 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(21, 18, 35),
    declination: Declination::new(Sgn::Pos, 62, 35, 8),
    distance: Distance {
        m: 49. * LIGHT_YEAR.m,
    },
};

const ERAKIS: RealData = RealData {
    common_name: "Erakis",
    astronomical_name: "Mu Cephei",
    constellation: "Cepheus",
    radius: Some(Distance {
        m: 972. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 19.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -6.5,
    apparent_magnitude: 3.43,
    temperature: Some(Temperature { K: 3551. }),
    age: Some(Time {
        s: 0.01 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(21, 43, 30),
    declination: Declination::new(Sgn::Pos, 58, 46, 48),
    distance: Distance {
        m: 3066. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
