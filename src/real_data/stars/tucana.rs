use astro_coords::ra_and_dec::*;
use simple_si_units::base::{Length, Mass, Temperature, Time};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

const ALPHA_TUCANAE: RealData = RealData {
    common_name: "",
    astronomical_name: "α Tucanae",
    constellation: "Tucana",
    radius: Some(Length {
        m: 37. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.5 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -1.05,
    apparent_magnitude: 2.87,
    temperature: Temperature { K: 4300. },
    age: None,
    right_ascension: RightAscension::new(22, 18, 30.),
    declination: Declination::new(Sgn::Neg, 60, 15, 35.),
    distance: Length {
        m: 198.5 * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.800458342 * BILLION_YEARS.s,
    },
};

const GAMMA_TUCANAE: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Tucanae",
    constellation: "Tucana",
    right_ascension: RightAscension::new(23, 17, 26.),
    declination: Declination::new(Sgn::Neg, 58, 14, 9.),
    apparent_magnitude: 3.99,
    distance: Length {
        m: 75. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.18,
    mass: Mass {
        kg: 1.55 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 2.2 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 6679. },
    age: Some(Time {
        s: 1.414 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 2.29668629 * BILLION_YEARS.s,
    },
};

const ZETA_TUCANAE: RealData = RealData {
    common_name: "",
    astronomical_name: "ζ Tucanae",
    constellation: "Tucana",
    right_ascension: RightAscension::new(0, 20, 4.),
    declination: Declination::new(Sgn::Neg, 64, 52, 29.),
    apparent_magnitude: 4.23,
    distance: Length {
        m: 28.01 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 4.67,
    mass: Mass {
        kg: 0.99 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 1.08 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5970. },
    age: Some(Time {
        s: 2.5 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 9.81519157 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_TUCANAE, GAMMA_TUCANAE, ZETA_TUCANAE];
