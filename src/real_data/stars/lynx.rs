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

const ALPHA_LYNCIS: RealData = RealData {
    common_name: "",
    astronomical_name: "α Lyncis",
    constellation: "Lynx",
    radius: Some(Distance {
        m: 54.5 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2. * SOLAR_MASS.kg,
    },
    absolute_magnitude: -1.02,
    apparent_magnitude: 3.15,
    temperature: Temperature { K: 3882. },
    right_ascension: RightAscension::new(9, 21, 3),
    declination: Declination::new(Sgn::Pos, 34, 23, 33),
    distance: Distance {
        m: 221.9 * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 1.3 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.36020165 * BILLION_YEARS.s,
    },
};

const THIRTYEIGHT_LYNCIS: RealData = RealData {
    common_name: "",
    astronomical_name: "38 Lyncis",
    constellation: "Lynx",
    right_ascension: RightAscension::new(9, 18, 51),
    declination: Declination::new(Sgn::Pos, 36, 48, 9),
    apparent_magnitude: 3.82,
    distance: Distance {
        m: 117. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.98,
    mass: Mass {
        kg: 1.9 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 3.07 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8862. },
    age: Some(Time {
        s: 0.213 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.54706939 * BILLION_YEARS.s,
    },
};

const THIRTYONE_LYNCIS: RealData = RealData {
    common_name: "",
    astronomical_name: "31 Lyncis",
    constellation: "Lynx",
    right_ascension: RightAscension::new(8, 22, 50),
    declination: Declination::new(Sgn::Pos, 43, 11, 17),
    apparent_magnitude: 4.25,
    distance: Distance {
        m: 380. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.09,
    mass: Mass {
        kg: 1.95 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 53.27 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 3921. },
    age: Some(Time {
        s: 1.32 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.46316038 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_LYNCIS, THIRTYEIGHT_LYNCIS, THIRTYONE_LYNCIS];
