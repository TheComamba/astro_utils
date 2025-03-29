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

const ARCTURUS: RealData = RealData {
    common_name: "Arcturus",
    astronomical_name: "α Boötis",
    constellation: "Boötes",
    radius: Some(Length {
        m: 25.4 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.08 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.31,
    apparent_magnitude: -0.05,
    temperature: Temperature { K: 4286. },
    right_ascension: RightAscension::new(14, 15, 40.),
    declination: Declination::new(Sgn::Pos, 19, 10, 56.),
    distance: Length {
        m: 37. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 6.9 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 6.97272616 * BILLION_YEARS.s,
    },
};

const IZAR: RealData = RealData {
    common_name: "Izar",
    astronomical_name: "ε Boötis",
    constellation: "Boötes",
    radius: Some(Length {
        m: 33. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 4.6 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -1.69,
    apparent_magnitude: 2.35,
    temperature: Temperature { K: 4550. },
    right_ascension: RightAscension::new(14, 44, 59.),
    declination: Declination::new(Sgn::Pos, 27, 4, 27.),
    distance: Length {
        m: 210. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.0374 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.136126994 * BILLION_YEARS.s,
    },
};

const GAMMA_BOOTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Boötis",
    constellation: "Boötes",
    right_ascension: RightAscension::new(14, 32, 5.),
    declination: Declination::new(Sgn::Pos, 38, 18, 30.),
    apparent_magnitude: 3.03,
    distance: Length {
        m: 86.8 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.93,
    mass: Mass {
        kg: 2.10 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 5.16 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 7800. },
    age: Some(Time {
        s: 0.9 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.17901142 * BILLION_YEARS.s,
    },
};

const DELTA_BOOTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "δ Boötis",
    constellation: "Boötes",
    right_ascension: RightAscension::new(15, 15, 30.),
    declination: Declination::new(Sgn::Pos, 33, 18, 53.),
    apparent_magnitude: 3.482,
    distance: Length {
        m: 121.8 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.7,
    mass: Mass {
        kg: 1.1 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 10.5 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4847. },
    age: None,
    lifetime: Time {
        s: 6.97272616 * BILLION_YEARS.s,
    },
};

const BETA_BOOTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "β Boötis",
    constellation: "Boötes",
    right_ascension: RightAscension::new(15, 1, 57.),
    declination: Declination::new(Sgn::Pos, 40, 23, 26.),
    apparent_magnitude: 3.488,
    distance: Length {
        m: 225. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.7,
    mass: Mass {
        kg: 3.4 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 21.5 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4932. },
    age: Some(Time {
        s: 0.240 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.297402042 * BILLION_YEARS.s,
    },
};

const MUPHRID: RealData = RealData {
    common_name: "Muphrid",
    astronomical_name: "η Boötis",
    constellation: "Boötes",
    right_ascension: RightAscension::new(13, 54, 41.),
    declination: Declination::new(Sgn::Pos, 18, 23, 52.),
    apparent_magnitude: 2.680,
    distance: Length {
        m: 37.2 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.41,
    mass: Mass {
        kg: 1.71 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 2.672 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 6100. },
    age: Some(Time {
        s: 1.6 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.73766023 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 6] = [
    ARCTURUS,
    IZAR,
    GAMMA_BOOTIS,
    DELTA_BOOTIS,
    BETA_BOOTIS,
    MUPHRID,
];
