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

const RASALHAGUE: RealData = RealData {
    common_name: "Rasalhague",
    astronomical_name: "Alpha Ophiuchi",
    constellation: "Ophiuchus",
    radius: Some(Distance {
        m: 2.6 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.4 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 1.30,
    apparent_magnitude: 2.08,
    temperature: Temperature { K: 8000. },
    age: Some(Time {
        s: 0.77 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(17, 34, 56),
    declination: Declination::new(Sgn::Pos, 12, 33, 37),
    distance: Distance {
        m: 47. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.800458342 * BILLION_YEARS.s,
    },
};

const SABIK: RealData = RealData {
    common_name: "Sabik",
    astronomical_name: "Eta Ophiuchi",
    constellation: "Ophiuchus",
    radius: None,
    mass: Mass {
        kg: 2.966 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 0.37,
    apparent_magnitude: 2.43,
    temperature: Temperature { K: 8900. },
    age: None,
    right_ascension: RightAscension::new(17, 10, 23),
    declination: Declination::new(Sgn::Neg, 15, 43, 30),
    distance: Distance {
        m: 84. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.420724107 * BILLION_YEARS.s,
    },
};

const HAN: RealData = RealData {
    common_name: "Han",
    astronomical_name: "Zeta Ophiuchi",
    constellation: "Ophiuchus",
    radius: Some(Distance {
        m: 8.5 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 20.2 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.20,
    apparent_magnitude: 2.54,
    temperature: Temperature { K: 34_300. },
    age: Some(Time {
        s: 3. * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(16, 37, 10),
    declination: Declination::new(Sgn::Neg, 10, 34, 2),
    distance: Distance {
        m: 458. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.009767659 * BILLION_YEARS.s,
    },
};

const YED_PRIOR: RealData = RealData {
    common_name: "Yed Prior",
    astronomical_name: "Delta Ophiuchi",
    constellation: "Ophiuchus",
    radius: Some(Distance {
        m: 59. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.5 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.90,
    apparent_magnitude: 2.73,
    temperature: Temperature { K: 3679. },
    age: None,
    right_ascension: RightAscension::new(16, 14, 21),
    declination: Declination::new(Sgn::Neg, 3, 41, 40),
    distance: Distance {
        m: 171. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 2.54186931 * BILLION_YEARS.s,
    },
};

const CEBALRAI: RealData = RealData {
    common_name: "Cebalrai",
    astronomical_name: "Beta Ophiuchi",
    constellation: "Ophiuchus",
    radius: Some(Distance {
        m: 12.42 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.13 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 0.77,
    apparent_magnitude: 2.76,
    temperature: Temperature { K: 4467. },
    right_ascension: RightAscension::new(17, 43, 28),
    declination: Declination::new(Sgn::Pos, 4, 34, 2),
    distance: Distance {
        m: 81.8 * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 3.82 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 5.9461393 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 5] = [RASALHAGUE, SABIK, HAN, YED_PRIOR, CEBALRAI];
