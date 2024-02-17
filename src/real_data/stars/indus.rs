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

const ALPHA_INDI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Indi",
    constellation: "Indus",
    right_ascension: RightAscension::new(20, 37, 34),
    declination: Declination::new(Sgn::Neg, 47, 17, 29),
    apparent_magnitude: 3.11,
    distance: Distance {
        m: 98.3 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.65,
    mass: Some(Mass {
        kg: 2.0 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 12. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4893. }),
    age: Some(Time {
        s: 1. * BILLION_YEARS.s,
    }),
};

const BETA_INDI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Indi",
    constellation: "Indus",
    right_ascension: RightAscension::new(20, 54, 49),
    declination: Declination::new(Sgn::Neg, 58, 27, 15),
    apparent_magnitude: 3.67,
    distance: Distance {
        m: 600. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.664,
    mass: Some(Mass {
        kg: 6.7 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 55.58 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4541. }),
    age: Some(Time {
        s: 0.0532 * BILLION_YEARS.s,
    }),
};

const ETA_INDI: RealData = RealData {
    common_name: "",
    astronomical_name: "Eta Indi",
    constellation: "Indus",
    right_ascension: RightAscension::new(20, 44, 2),
    declination: Declination::new(Sgn::Neg, 51, 55, 15),
    apparent_magnitude: 4.52,
    distance: Distance {
        m: 78.8 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.59,
    mass: Some(Mass {
        kg: 1.6 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.27 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 7694. }),
    age: Some(Time {
        s: 0.1 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_INDI, BETA_INDI, ETA_INDI];
