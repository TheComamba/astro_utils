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

const SIRIUS: RealData = RealData {
    common_name: "Sirius",
    astronomical_name: "Alpha Canis Majoris",
    constellation: "Canis Major",
    radius: Some(Distance {
        m: 1.711 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.063 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.45,
    apparent_magnitude: -1.44,
    temperature: Temperature { K: 9940. },
    age: Some(Time {
        s: 0.242 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(6, 45, 9),
    declination: Declination::new(Sgn::Neg, 16, 42, 58),
    distance: Distance {
        m: 9. * LIGHT_YEAR.m,
    },
};

const ADHARA: RealData = RealData {
    common_name: "Adhara",
    astronomical_name: "Epsilon Canis Majoris",
    constellation: "Canis Major",
    radius: Some(Distance {
        m: 13.9 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 12.6 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.10,
    apparent_magnitude: 1.5,
    temperature: Temperature { K: 22_900. },
    age: Some(Time {
        s: 0.0225 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(6, 58, 38),
    declination: Declination::new(Sgn::Neg, 28, 58, 19),
    distance: Distance {
        m: 431. * LIGHT_YEAR.m,
    },
};

const WEZEN: RealData = RealData {
    common_name: "Wezen",
    astronomical_name: "Delta Canis Majoris",
    constellation: "Canis Major",
    radius: Some(Distance {
        m: 215. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 16.9 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -6.87,
    apparent_magnitude: 1.83,
    temperature: Temperature { K: 6390. },
    age: Some(Time {
        s: 0.012 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(7, 8, 23),
    declination: Declination::new(Sgn::Neg, 26, 23, 36),
    distance: Distance {
        m: 1791. * LIGHT_YEAR.m,
    },
};

const MIRZAM: RealData = RealData {
    common_name: "Mirzam",
    astronomical_name: "Beta Canis Majoris",
    constellation: "Canis Major",
    radius: Some(Distance {
        m: 9.7 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 13.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.95,
    apparent_magnitude: 1.98,
    temperature: Temperature { K: 25_000. },
    age: Some(Time {
        s: 0.0124 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(6, 22, 42),
    declination: Declination::new(Sgn::Neg, 17, 57, 21),
    distance: Distance {
        m: 499. * LIGHT_YEAR.m,
    },
};

const ALUDRA: RealData = RealData {
    common_name: "Aludra",
    astronomical_name: "Eta Canis Majoris",
    constellation: "Canis Major",
    radius: Some(Distance {
        m: 54. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 18.19 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -7.51,
    apparent_magnitude: 2.45,
    temperature: Temperature { K: 15_500. },
    age: Some(Time {
        s: 0.0083 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(7, 24, 6),
    declination: Declination::new(Sgn::Neg, 29, 18, 11),
    distance: Distance {
        m: 3196. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 5] = [SIRIUS, ADHARA, WEZEN, MIRZAM, ALUDRA];
