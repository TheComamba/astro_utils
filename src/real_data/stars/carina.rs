use astro_coordinates::{
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

const CANOPUS: RealData = RealData {
    common_name: "Canopus",
    astronomical_name: "α Carinae",
    constellation: "Carina",
    radius: Some(Distance {
        m: 72. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 9. * SOLAR_MASS.kg,
    },
    absolute_magnitude: -5.53,
    apparent_magnitude: -0.62,
    temperature: Temperature { K: 7400. },
    right_ascension: RightAscension::new(6, 23, 57),
    declination: Declination::new(Sgn::Neg, 52, 41, 44),
    distance: Distance {
        m: 313. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.0251 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.03224554 * BILLION_YEARS.s,
    },
};

const MIAPLACIDUS: RealData = RealData {
    common_name: "Miaplacidus",
    astronomical_name: "β Carinae",
    constellation: "Carina",
    radius: Some(Distance {
        m: 6.8 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 3.5 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.99,
    apparent_magnitude: 1.67,
    temperature: Temperature { K: 8866. },
    right_ascension: RightAscension::new(9, 13, 12),
    declination: Declination::new(Sgn::Neg, 69, 43, 2),
    distance: Distance {
        m: 111. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.260 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.297402042 * BILLION_YEARS.s,
    },
};

const AVIOR: RealData = RealData {
    common_name: "Avior",
    astronomical_name: "ε Carinae",
    constellation: "Carina",
    radius: None,
    mass: Mass {
        kg: 10.5 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -4.58,
    apparent_magnitude: 1.86,
    temperature: Temperature { K: 3523. },
    right_ascension: RightAscension::new(8, 22, 31),
    declination: Declination::new(Sgn::Neg, 59, 30, 34),
    distance: Distance {
        m: 632. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.026 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.026540021 * BILLION_YEARS.s,
    },
};

const ASPIDISKE: RealData = RealData {
    common_name: "Aspidiske",
    astronomical_name: "ι Carinae",
    constellation: "Carina",
    radius: Some(Distance {
        m: 43. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 7.4 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -4.42,
    apparent_magnitude: 2.21,
    temperature: Temperature { K: 7500. },
    right_ascension: RightAscension::new(9, 17, 5),
    declination: Declination::new(Sgn::Neg, 59, 16, 30),
    distance: Distance {
        m: 694. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.0374 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.052267043 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 4] = [CANOPUS, MIAPLACIDUS, AVIOR, ASPIDISKE];
