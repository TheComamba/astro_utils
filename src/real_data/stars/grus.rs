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

const ALNAIR: RealData = RealData {
    common_name: "Alnair",
    astronomical_name: "α Gruis",
    constellation: "Grus",
    radius: Some(Distance {
        m: 3.4 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 4. * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.73,
    apparent_magnitude: 1.73,
    temperature: Temperature { K: 13_920. },
    age: Some(Time {
        s: 0.1 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.193156929 * BILLION_YEARS.s,
    },
    right_ascension: RightAscension::new(22, 8, 14),
    declination: Declination::new(Sgn::Neg, 46, 57, 40),
    distance: Distance {
        m: 101. * LIGHT_YEAR.m,
    },
};

const TIAKI: RealData = RealData {
    common_name: "Tiaki",
    astronomical_name: "β Gruis",
    constellation: "Grus",
    radius: Some(Distance {
        m: 180. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.4 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -1.52,
    apparent_magnitude: 2.07,
    temperature: Temperature { K: 3480. },
    age: None,
    lifetime: Time {
        s: 0.800458342 * BILLION_YEARS.s,
    },
    right_ascension: RightAscension::new(22, 42, 40),
    declination: Declination::new(Sgn::Neg, 46, 53, 4),
    distance: Distance {
        m: 170. * LIGHT_YEAR.m,
    },
};

const ALDHANAB: RealData = RealData {
    common_name: "Aldhanab",
    astronomical_name: "γ Gruis",
    constellation: "Grus",
    right_ascension: RightAscension::new(21, 53, 56),
    declination: Declination::new(Sgn::Neg, 37, 21, 53),
    apparent_magnitude: 3.003,
    distance: Distance {
        m: 211. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.05,
    mass: Mass {
        kg: 3.06 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 4.5 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 12_520. },
    age: Some(Time {
        s: 0.075 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.420724107 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALNAIR, TIAKI, ALDHANAB];
