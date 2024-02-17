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

const SPICA: RealData = RealData {
    common_name: "Spica",
    astronomical_name: "Alpha Virginis",
    constellation: "Virgo",
    radius: Some(Distance {
        m: 7.47 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 11.43 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.55,
    apparent_magnitude: 0.98,
    temperature: Some(Temperature { K: 22_300. }),
    age: Some(Time {
        s: 0.0125 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(13, 25, 12),
    declination: Declination::new(Sgn::Neg, 11, 9, 41),
    distance: Distance {
        m: 262. * LIGHT_YEAR.m,
    },
};

const MINELAUVA: RealData = RealData {
    common_name: "Minelauva",
    astronomical_name: "Delta Virginis",
    constellation: "Virgo",
    radius: Some(Distance {
        m: 48. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.575,
    apparent_magnitude: 3.39,
    temperature: Some(Temperature { K: 3999. }),
    age: None,
    right_ascension: RightAscension::new(12, 55, 36),
    declination: Declination::new(Sgn::Pos, 3, 23, 51),
    distance: Distance {
        m: 202.4 * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
