use astro_coords::ra_and_dec::*;
use simple_si_units::base::{Length, Mass, Temperature, Time};

use crate::{
    stars::real_data::RealData,
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

const GHURAB: RealData = RealData {
    common_name: "Ghurab",
    astronomical_name: "γ Corvi",
    constellation: "Corvus",
    radius: None,
    mass: Mass {
        kg: 4.2 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.94,
    apparent_magnitude: 2.58,
    temperature: Temperature { K: 12_000. },
    age: Some(Time {
        s: 0.160 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 15, 48.),
    declination: Declination::new(Sgn::Neg, 17, 32, 31.),
    distance: Length {
        m: 165. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.170765802 * BILLION_YEARS.s,
    },
};

const KRAZ: RealData = RealData {
    common_name: "Kraz",
    astronomical_name: "β Corvi",
    constellation: "Corvus",
    radius: Some(Length {
        m: 16. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 3.7 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.61,
    apparent_magnitude: 2.65,
    temperature: Temperature { K: 5100. },
    age: Some(Time {
        s: 0.206 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 34, 23.),
    declination: Declination::new(Sgn::Neg, 23, 23, 48.),
    distance: Length {
        m: 146. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.254814649 * BILLION_YEARS.s,
    },
};

const ALGORAB: RealData = RealData {
    common_name: "Algorab",
    astronomical_name: "δ Corvi",
    constellation: "Corvus",
    right_ascension: RightAscension::new(12, 29, 52.),
    declination: Declination::new(Sgn::Neg, 16, 30, 56.),
    apparent_magnitude: 2.94,
    distance: Length {
        m: 87.85 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.787,
    mass: Mass {
        kg: 2.74 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 10_400. },
    age: Some(Time {
        s: 0.260 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.513076303 * BILLION_YEARS.s,
    },
};

const EPSILON_CORVI: RealData = RealData {
    common_name: "",
    astronomical_name: "ε Corvi",
    constellation: "Corvus",
    right_ascension: RightAscension::new(12, 10, 7.),
    declination: Declination::new(Sgn::Neg, 22, 37, 11.),
    apparent_magnitude: 3.024,
    distance: Length {
        m: 318. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.82,
    mass: Mass {
        kg: 3.2 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 52. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4320. },
    age: None,
    lifetime: Time {
        s: 0.351318702 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 4] = [GHURAB, KRAZ, ALGORAB, EPSILON_CORVI];
