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

const DIADEM: RealData = RealData {
    common_name: "Diadem",
    astronomical_name: "α Comae Berenices",
    constellation: "Coma Berenices",
    right_ascension: RightAscension::new(13, 9, 59.),
    declination: Declination::new(Sgn::Pos, 17, 31, 46.),
    apparent_magnitude: 4.32,
    distance: Length {
        m: 46.7 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.54,
    mass: Mass {
        kg: 1.237 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 6365. },
    age: None,
    lifetime: Time {
        s: 4.45521207 * BILLION_YEARS.s,
    },
};

const BETA_COMA_BERENICES: RealData = RealData {
    common_name: "",
    astronomical_name: "β Comae Berenices",
    constellation: "Coma Berenices",
    right_ascension: RightAscension::new(13, 11, 53.),
    declination: Declination::new(Sgn::Pos, 27, 52, 41.),
    apparent_magnitude: 4.26,
    distance: Length {
        m: 29.95 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 4.46,
    mass: Mass {
        kg: 1.15 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 1.106 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5936. },
    age: Some(Time {
        s: 2. * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 5.9461393 * BILLION_YEARS.s,
    },
};

const GAMMA_COMA_BERENICES: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Comae Berenices",
    constellation: "Coma Berenices",
    right_ascension: RightAscension::new(12, 26, 56.),
    declination: Declination::new(Sgn::Pos, 28, 16, 6.),
    apparent_magnitude: 4.36,
    distance: Length {
        m: 169. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.76,
    mass: Mass {
        kg: 1.65 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 11.76 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4652. },
    age: Some(Time {
        s: 1.8 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.89665739 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [DIADEM, BETA_COMA_BERENICES, GAMMA_COMA_BERENICES];
