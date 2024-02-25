use crate::{
    coordinates::{
        declination::{Declination, Sgn},
        right_ascension::RightAscension,
    },
    stars::{fate::StarFate, real_data::RealData},
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};
use simple_si_units::base::{Distance, Mass, Temperature, Time};

const ARCTURUS: RealData = RealData {
    common_name: "Arcturus",
    astronomical_name: "Alpha Boötis",
    constellation: "Boötes",
    radius: Some(Distance {
        m: 25.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.08 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.31,
    apparent_magnitude: -0.05,
    temperature: Temperature { K: 4286. },
    age: Some(Time {
        s: 7.1 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 7.2 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
    right_ascension: RightAscension::new(14, 15, 40),
    declination: Declination::new(Sgn::Pos, 19, 10, 56),
    distance: Distance {
        m: 37. * LIGHT_YEAR.m,
    },
};

const IZAR: RealData = RealData {
    common_name: "Izar",
    astronomical_name: "Epsilon Boötis",
    constellation: "Boötes",
    radius: Some(Distance {
        m: 33. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 4.6 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.69,
    apparent_magnitude: 2.35,
    temperature: Temperature { K: 4550. },
    age: Some(Time {
        s: 0.0374 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.220 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
    right_ascension: RightAscension::new(14, 44, 59),
    declination: Declination::new(Sgn::Pos, 27, 4, 27),
    distance: Distance {
        m: 210. * LIGHT_YEAR.m,
    },
};

const GAMMA_BOOTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Boötis",
    constellation: "Boötes",
    right_ascension: RightAscension::new(14, 32, 5),
    declination: Declination::new(Sgn::Pos, 38, 18, 30),
    apparent_magnitude: 3.03,
    distance: Distance {
        m: 86.8 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.93,
    mass: Some(Mass {
        kg: 2.10 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 5.16 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 7800. },
    age: Some(Time {
        s: 0.9 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.670 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
};

const DELTA_BOOTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Boötis",
    constellation: "Boötes",
    right_ascension: RightAscension::new(15, 15, 30),
    declination: Declination::new(Sgn::Pos, 33, 18, 53),
    apparent_magnitude: 3.482,
    distance: Distance {
        m: 121.8 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.7,
    mass: Some(Mass {
        kg: 1.1 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 10.5 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4847. },
    age: None,
    lifetime: Time {
        s: 7.9 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
};

const BETA_BOOTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Boötis",
    constellation: "Boötes",
    right_ascension: RightAscension::new(15, 1, 57),
    declination: Declination::new(Sgn::Pos, 40, 23, 26),
    apparent_magnitude: 3.488,
    distance: Distance {
        m: 225. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.7,
    mass: Some(Mass {
        kg: 3.4 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 21.5 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4932. },
    age: Some(Time {
        s: 0.240 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 7.9 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
};

const MUPHRID: RealData = RealData {
    common_name: "Muphrid",
    astronomical_name: "Eta Boötis",
    constellation: "Boötes",
    right_ascension: RightAscension::new(13, 54, 41),
    declination: Declination::new(Sgn::Pos, 18, 23, 52),
    apparent_magnitude: 2.680,
    distance: Distance {
        m: 37.2 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.41,
    mass: Some(Mass {
        kg: 1.71 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.672 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 6100. },
    age: Some(Time {
        s: 2.7 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 9.4 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
};

pub(crate) const STARS: [RealData; 6] = [
    ARCTURUS,
    IZAR,
    GAMMA_BOOTIS,
    DELTA_BOOTIS,
    BETA_BOOTIS,
    MUPHRID,
];
