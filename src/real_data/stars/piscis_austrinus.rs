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

const FORMALHAUT: RealData = RealData {
    common_name: "Formalhaut",
    astronomical_name: "α Piscis Austrini",
    constellation: "Piscis Austrinus",
    radius: Some(Length {
        m: 1.842 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.92 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 1.74,
    apparent_magnitude: 1.17,
    temperature: Temperature { K: 8590. },
    age: Some(Time {
        s: 0.44 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(22, 57, 39.),
    declination: Declination::new(Sgn::Neg, 29, 37, 20.),
    distance: Length {
        m: 25. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 1.54706939 * BILLION_YEARS.s,
    },
};

const DELTA_PISCIS_AUSTRINI: RealData = RealData {
    common_name: "",
    astronomical_name: "δ Piscis Austrini",
    constellation: "Piscis Austrinus",
    right_ascension: RightAscension::new(22, 55, 57.),
    declination: Declination::new(Sgn::Neg, 32, 32, 23.),
    apparent_magnitude: 4.175,
    distance: Length {
        m: 172. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.636,
    mass: Mass {
        kg: 1.42 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 4828. },
    age: Some(Time {
        s: 3.1 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 3.10253119 * BILLION_YEARS.s,
    },
};

const EPSILON_PISCIS_AUSTRINI: RealData = RealData {
    common_name: "",
    astronomical_name: "ε Piscis Austrini",
    constellation: "Piscis Austrinus",
    right_ascension: RightAscension::new(22, 40, 39.),
    declination: Declination::new(Sgn::Neg, 27, 2, 37.),
    apparent_magnitude: 4.18,
    distance: Length {
        m: 744.3 * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.61,
    mass: Mass {
        kg: 4.1 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 3.2 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 11_066. },
    age: None,
    lifetime: Time {
        s: 0.193156929 * BILLION_YEARS.s,
    },
};

const IOTA_PISCIS_AUSTRINI: RealData = RealData {
    common_name: "",
    astronomical_name: "ι Piscis Austrini",
    constellation: "Piscis Austrinus",
    right_ascension: RightAscension::new(21, 44, 57.),
    declination: Declination::new(Sgn::Neg, 33, 1, 33.),
    apparent_magnitude: 4.35,
    distance: Length {
        m: 204. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.37,
    mass: Mass {
        kg: 3.1 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 9330. },
    age: None,
    lifetime: Time {
        s: 0.420724107 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 4] = [
    FORMALHAUT,
    DELTA_PISCIS_AUSTRINI,
    EPSILON_PISCIS_AUSTRINI,
    IOTA_PISCIS_AUSTRINI,
];
