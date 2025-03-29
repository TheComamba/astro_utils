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

const KITALPHA: RealData = RealData {
    common_name: "Kitalpha",
    astronomical_name: "α Equulei",
    constellation: "Equuleus",
    right_ascension: RightAscension::new(21, 15, 49.),
    declination: Declination::new(Sgn::Pos, 5, 14, 52.),
    apparent_magnitude: 3.919,
    distance: Length {
        m: 190. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.17,
    mass: Mass {
        kg: 2.3 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 9.2 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5100. },
    age: None,
    lifetime: Time {
        s: 0.916355612 * BILLION_YEARS.s,
    },
};

const DELTA_EQUULEI: RealData = RealData {
    common_name: "",
    astronomical_name: "δ Equulei",
    constellation: "Equuleus",
    right_ascension: RightAscension::new(21, 14, 29.),
    declination: Declination::new(Sgn::Pos, 10, 0, 25.),
    apparent_magnitude: 4.47,
    distance: Length {
        m: 60.25 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.140,
    mass: Mass {
        kg: 1.192 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 1.30 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 6200. },
    age: Some(Time {
        s: 3. * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 5.06543331 * BILLION_YEARS.s,
    },
};

const GAMMA_EQUULEI: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Equulei",
    constellation: "Equuleus",
    right_ascension: RightAscension::new(21, 10, 21.),
    declination: Declination::new(Sgn::Pos, 10, 7, 54.),
    apparent_magnitude: 4.6,
    distance: Length {
        m: 118. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.9,
    mass: Mass {
        kg: 1.78 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 2.11 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 7550. },
    age: Some(Time {
        s: 1. * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.46605285 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [KITALPHA, DELTA_EQUULEI, GAMMA_EQUULEI];
