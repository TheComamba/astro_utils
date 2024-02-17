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

const KAUS_AUSTRALIS: RealData = RealData {
    common_name: "Kaus Australis",
    astronomical_name: "Epsilon Sagittarii",
    constellation: "Sagittarius",
    radius: Some(Distance {
        m: 6.8 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.515 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.44,
    apparent_magnitude: 1.79,
    temperature: Some(Temperature { K: 9960. }),
    age: Some(Time {
        s: 0.232 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(18, 24, 10),
    declination: Declination::new(Sgn::Neg, 34, 23, 5),
    distance: Distance {
        m: 145. * LIGHT_YEAR.m,
    },
};

const NUNKI: RealData = RealData {
    common_name: "Nunki",
    astronomical_name: "Sigma Sagittarii",
    constellation: "Sagittarius",
    radius: Some(Distance {
        m: 4.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.14,
    apparent_magnitude: 2.05,
    temperature: Some(Temperature { K: 18_890. }),
    age: Some(Time {
        s: 0.0314 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(18, 55, 16),
    declination: Declination::new(Sgn::Neg, 26, 17, 49),
    distance: Distance {
        m: 224. * LIGHT_YEAR.m,
    },
};

const NAMALWARID: RealData = RealData {
    common_name: "Namalwarid",
    astronomical_name: "Eta Sagittarii",
    constellation: "Sagittarius",
    radius: None,
    mass: None,
    absolute_magnitude: -0.201,
    apparent_magnitude: 3.1,
    temperature: None,
    age: None,
    right_ascension: RightAscension::new(18, 17, 38),
    declination: Declination::new(Sgn::Neg, 36, 45, 42),
    distance: Distance {
        m: 149.1 * LIGHT_YEAR.m,
    },
};

const KAUS_MEDIA: RealData = RealData {
    common_name: "Kaus Media",
    astronomical_name: "Delta Sagittarii",
    constellation: "Sagittarius",
    radius: Some(Distance {
        m: 16. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.21 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.14,
    apparent_magnitude: 2.72,
    temperature: Some(Temperature { K: 4203. }),
    age: Some(Time {
        s: 0.26 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(18, 20, 60),
    declination: Declination::new(Sgn::Neg, 29, 49, 41),
    distance: Distance {
        m: 305.5 * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 4] = [KAUS_AUSTRALIS, NUNKI, NAMALWARID, KAUS_MEDIA];
