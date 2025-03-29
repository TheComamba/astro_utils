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

const CAPELLA: RealData = RealData {
    common_name: "Capella",
    astronomical_name: "α Aurigae",
    constellation: "Auriga",
    radius: Some(Length {
        m: 11.98 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.5687 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.48,
    apparent_magnitude: 0.08,
    temperature: Temperature { K: 4970. },
    age: Some(Time {
        s: 0.620 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 16, 41.),
    declination: Declination::new(Sgn::Pos, 45, 59, 53.),
    distance: Length {
        m: 42. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.63513384 * BILLION_YEARS.s,
    },
};

const MENKALINAN: RealData = RealData {
    common_name: "Menkalinan",
    astronomical_name: "β Aurigae",
    constellation: "Auriga",
    radius: Some(Length {
        m: 2.77 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.389 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.10,
    apparent_magnitude: 1.9,
    temperature: Temperature { K: 9350. },
    right_ascension: RightAscension::new(5, 59, 32.),
    declination: Declination::new(Sgn::Pos, 44, 56, 51.),
    distance: Length {
        m: 82. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.570 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.800458342 * BILLION_YEARS.s,
    },
};

const HASSALEH: RealData = RealData {
    common_name: "Hassaleh",
    astronomical_name: "ι Aurigae",
    constellation: "Auriga",
    radius: Some(Length {
        m: 127. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 7.1 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.20,
    apparent_magnitude: 2.69,
    temperature: Temperature { K: 4160. },
    right_ascension: RightAscension::new(4, 56, 60.),
    declination: Declination::new(Sgn::Pos, 33, 9, 58.),
    distance: Length {
        m: 490. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.04 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.052267043 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [CAPELLA, MENKALINAN, HASSALEH];
