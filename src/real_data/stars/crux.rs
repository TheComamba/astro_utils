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

const ACRUX: RealData = RealData {
    common_name: "Acrux",
    astronomical_name: "Alpha Crucis",
    constellation: "Crux",
    radius: Some(Distance {
        m: 7.8 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 17.8 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -4.19,
    apparent_magnitude: 0.77,
    temperature: Temperature { K: 24_000. },
    age: Some(Time {
        s: 0.0108 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.011037517 * BILLION_YEARS.s,
    },
    right_ascension: RightAscension::new(12, 26, 36),
    declination: Declination::new(Sgn::Neg, 63, 5, 57),
    distance: Distance {
        m: 321. * LIGHT_YEAR.m,
    },
};

const MIMOSA: RealData = RealData {
    common_name: "Mimosa",
    astronomical_name: "Beta Crucis",
    constellation: "Crux",
    radius: Some(Distance {
        m: 8.4 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 16. * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.92,
    apparent_magnitude: 1.25,
    temperature: Temperature { K: 27_000. },
    age: Some(Time {
        s: 0.010 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 47, 43),
    declination: Declination::new(Sgn::Neg, 59, 41, 20),
    distance: Distance {
        m: 352. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.012799766 * BILLION_YEARS.s,
    },
};

const GACRUX: RealData = RealData {
    common_name: "Gacrux",
    astronomical_name: "Gamma Crucis",
    constellation: "Crux",
    radius: Some(Distance {
        m: 120. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.5 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.56,
    apparent_magnitude: 1.59,
    temperature: Temperature { K: 3689. },
    age: None,
    right_ascension: RightAscension::new(12, 31, 10),
    declination: Declination::new(Sgn::Neg, 57, 6, 48),
    distance: Distance {
        m: 88. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 2.54186931 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ACRUX, MIMOSA, GACRUX];
