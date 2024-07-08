use astro_coords::{
    declination::{Declination, Sgn},
    right_ascension::RightAscension,
};
use simple_si_units::base::{Distance, Mass, Temperature, Time};

use crate::{
    stars::real_data::RealData,
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

const ALPHA_VULPECULAE: RealData = RealData {
    common_name: "",
    astronomical_name: "α Vulpeculae",
    constellation: "Vulpecula",
    right_ascension: RightAscension::new(19, 28, 42),
    declination: Declination::new(Sgn::Pos, 24, 39, 54),
    apparent_magnitude: 4.40,
    distance: Distance {
        m: 291. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.36,
    mass: Mass {
        kg: 0.97 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 43.14 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 3690. },
    age: Some(Time {
        s: 11.3 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 11.7800188 * BILLION_YEARS.s,
    },
};

const TWENTYTHREE_VULPECULAE: RealData = RealData {
    common_name: "",
    astronomical_name: "23 Vulpeculae",
    constellation: "Vulpecula",
    right_ascension: RightAscension::new(20, 15, 46),
    declination: Declination::new(Sgn::Pos, 27, 48, 51),
    apparent_magnitude: 4.52,
    distance: Distance {
        m: 327. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.58,
    mass: Mass {
        kg: 2.4 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 4429. },
    age: None,
    lifetime: Time {
        s: 0.800458342 * BILLION_YEARS.s,
    },
};

const THIRTYONE_VULPECULAE: RealData = RealData {
    common_name: "",
    astronomical_name: "31 Vulpeculae",
    constellation: "Vulpecula",
    right_ascension: RightAscension::new(20, 52, 8),
    declination: Declination::new(Sgn::Pos, 27, 5, 49),
    apparent_magnitude: 4.56,
    distance: Distance {
        m: 216.5 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.449,
    mass: Mass {
        kg: 2.4 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 8.01 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5261. },
    age: Some(Time {
        s: 0.7 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.800458342 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [
    ALPHA_VULPECULAE,
    TWENTYTHREE_VULPECULAE,
    THIRTYONE_VULPECULAE,
];
