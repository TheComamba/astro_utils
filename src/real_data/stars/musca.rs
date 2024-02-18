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

const ALPHA_MUSCAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Muscae",
    constellation: "Musca",
    right_ascension: RightAscension::new(12, 37, 11),
    declination: Declination::new(Sgn::Neg, 69, 8, 8),
    apparent_magnitude: 2.69,
    distance: Distance {
        m: 315. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.2,
    mass: Some(Mass {
        kg: 8.8 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 4.8 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 21_400. },
    age: Some(Time {
        s: 18.3 * BILLION_YEARS.s,
    }),
};

const BETA_MUSCAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Muscae",
    constellation: "Musca",
    right_ascension: RightAscension::new(12, 46, 17),
    declination: Declination::new(Sgn::Neg, 68, 6, 29),
    apparent_magnitude: 3.05,
    distance: Distance {
        m: 340. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.06,
    mass: Some(Mass {
        kg: 7.35 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Temperature { K: 21_000. },
    age: Some(Time {
        s: 0.0151 * BILLION_YEARS.s,
    }),
};

const DELTA_MUSCAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Muscae",
    constellation: "Musca",
    right_ascension: RightAscension::new(13, 2, 16),
    declination: Declination::new(Sgn::Neg, 71, 32, 56),
    apparent_magnitude: 3.61,
    distance: Distance {
        m: 91. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.38,
    mass: None,
    radius: None,
    temperature: Temperature { K: 4_400. },
    age: None,
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_MUSCAE, BETA_MUSCAE, DELTA_MUSCAE];
