use astro_coords::ra_and_dec::*;
use simple_si_units::base::{Distance, Mass, Temperature, Time};

use crate::{
    stars::real_data::RealData,
    units::{distance::LIGHT_YEAR, mass::SOLAR_MASS, time::BILLION_YEARS},
};

const GAMMA2_NORMAE: RealData = RealData {
    common_name: "",
    astronomical_name: "γ² Normae",
    constellation: "Norma",
    right_ascension: RightAscension::new(16, 19, 50.),
    declination: Declination::new(Sgn::Neg, 50, 9, 20.),
    apparent_magnitude: 4.02,
    distance: Distance {
        m: 129. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.057,
    mass: Mass {
        kg: 2.16 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 4699. },
    age: None,
    lifetime: Time {
        s: 1.09929685 * BILLION_YEARS.s,
    },
};

const EPSILON_NORMAE: RealData = RealData {
    common_name: "",
    astronomical_name: "ε Normae",
    constellation: "Norma",
    right_ascension: RightAscension::new(16, 27, 11.),
    declination: Declination::new(Sgn::Neg, 47, 33, 17.),
    apparent_magnitude: 4.46,
    distance: Distance {
        m: 399.5 * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.982,
    mass: Mass {
        kg: 6.4 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 10_888. },
    age: Some(Time {
        s: 0.0501 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.063411557 * BILLION_YEARS.s,
    },
};

const IOTA1_NORMAE: RealData = RealData {
    common_name: "",
    astronomical_name: "ι¹ Normae",
    constellation: "Norma",
    right_ascension: RightAscension::new(16, 3, 32.),
    declination: Declination::new(Sgn::Neg, 57, 46, 30.),
    apparent_magnitude: 4.69,
    distance: Distance {
        m: 128. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.46,
    mass: Mass {
        kg: 1.94 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 7842. },
    age: Some(Time {
        s: 0.731 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.46316038 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [GAMMA2_NORMAE, EPSILON_NORMAE, IOTA1_NORMAE];
