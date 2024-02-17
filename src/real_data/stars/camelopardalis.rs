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

const BETA_CAMELOPARDALIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Camelopardalis",
    constellation: "Camelopardalis",
    radius: Some(Distance {
        m: 58. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 6.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.1,
    apparent_magnitude: 4.02,
    temperature: Some(Temperature { K: 5300. }),
    age: Some(Time {
        s: 0.053 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 3, 25),
    declination: Declination::new(Sgn::Pos, 60, 26, 32),
    distance: Distance {
        m: 870. * LIGHT_YEAR.m,
    },
};

const CS_CAMELOPARDALIS: RealData = RealData {
    common_name: "",
    astronomical_name: "CS Camelopardalis",
    constellation: "Camelopardalis",
    radius: Some(Distance {
        m: 85.7 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 19. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -7.1,
    apparent_magnitude: 4.22,
    temperature: Some(Temperature { K: 10_800. }),
    age: Some(Time {
        s: 0.016_5 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(3, 29, 4),
    declination: Declination::new(Sgn::Pos, 59, 56, 25),
    distance: Distance {
        m: 3100. * LIGHT_YEAR.m,
    },
};

const ALPHA_CAMELOPARDALIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Camelopardalis",
    constellation: "Camelopardalis",
    radius: Some(Distance {
        m: 32.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 37.6 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -7.1,
    apparent_magnitude: 4.29,
    temperature: Some(Temperature { K: 29_000. }),
    age: Some(Time {
        s: 0.002 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 54, 3),
    declination: Declination::new(Sgn::Pos, 66, 20, 34),
    distance: Distance {
        m: 6_000. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 3] =
    [BETA_CAMELOPARDALIS, CS_CAMELOPARDALIS, ALPHA_CAMELOPARDALIS];
