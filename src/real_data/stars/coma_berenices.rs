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

const DIADEM: RealData = RealData {
    common_name: "Diadem",
    astronomical_name: "Alpha Comae Berenices",
    constellation: "Coma Berenices",
    right_ascension: RightAscension::new(13, 9, 59),
    declination: Declination::new(Sgn::Pos, 17, 31, 46),
    apparent_magnitude: 4.32,
    distance: Distance {
        m: 46.7 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.54,
    mass: Some(Mass {
        kg: 1.237 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 6365. }),
    age: None,
};

const BETA_COMA_BERENICES: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Comae Berenices",
    constellation: "Coma Berenices",
    right_ascension: RightAscension::new(13, 11, 53),
    declination: Declination::new(Sgn::Pos, 27, 52, 41),
    apparent_magnitude: 4.26,
    distance: Distance {
        m: 29.95 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 4.46,
    mass: Some(Mass {
        kg: 1.15 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.106 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5936. }),
    age: Some(Time {
        s: 2. * BILLION_YEARS.s,
    }),
};

const GAMMA_COMA_BERENICES: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Comae Berenices",
    constellation: "Coma Berenices",
    right_ascension: RightAscension::new(12, 26, 56),
    declination: Declination::new(Sgn::Pos, 28, 16, 6),
    apparent_magnitude: 4.36,
    distance: Distance {
        m: 169. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.76,
    mass: Some(Mass {
        kg: 1.65 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 11.76 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4652. }),
    age: Some(Time {
        s: 2.72 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 3] = [DIADEM, BETA_COMA_BERENICES, GAMMA_COMA_BERENICES];
