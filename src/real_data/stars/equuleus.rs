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

const ALPHA_EQUULEI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Equulei",
    constellation: "Equuleus",
    right_ascension: RightAscension::new(21, 15, 49),
    declination: Declination::new(Sgn::Pos, 5, 14, 52),
    apparent_magnitude: 3.919,
    distance: Distance {
        m: 190. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.17,
    mass: Some(Mass {
        kg: 2.3 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 9.2 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5100. }),
    age: None,
};

const DELTA_EQUULEI: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Equulei",
    constellation: "Equuleus",
    right_ascension: RightAscension::new(21, 14, 29),
    declination: Declination::new(Sgn::Pos, 10, 0, 25),
    apparent_magnitude: 5.19,
    distance: Distance {
        m: 59.4 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.1,
    mass: Some(Mass {
        kg: 1.192 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.30 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 6200. }),
    age: Some(Time {
        s: 3. * BILLION_YEARS.s,
    }),
};

const GAMMA_EQUULEI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Equulei",
    constellation: "Equuleus",
    right_ascension: RightAscension::new(21, 10, 21),
    declination: Declination::new(Sgn::Pos, 10, 7, 54),
    apparent_magnitude: 4.6,
    distance: Distance {
        m: 118. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.9,
    mass: Some(Mass {
        kg: 1.78 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.11 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 7550. }),
    age: Some(Time {
        s: 1. * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_EQUULEI, DELTA_EQUULEI, GAMMA_EQUULEI];
