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

const ELTANIN: RealData = RealData {
    common_name: "Eltanin",
    astronomical_name: "Gamma Draconis",
    constellation: "Draco",
    radius: Some(Distance {
        m: 48.15 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.72 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.04,
    apparent_magnitude: 2.24,
    temperature: Some(Temperature { K: 3930. }),
    age: None,
    right_ascension: RightAscension::new(17, 56, 36),
    declination: Declination::new(Sgn::Pos, 51, 29, 20),
    distance: Distance {
        m: 148. * LIGHT_YEAR.m,
    },
};

const ATHEBYNE: RealData = RealData {
    common_name: "Athebyne",
    astronomical_name: "Eta Draconis",
    constellation: "Draco",
    radius: Some(Distance {
        m: 11. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.55 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.58,
    apparent_magnitude: 2.73,
    temperature: Some(Temperature { K: 5055. }),
    age: Some(Time {
        s: 0.55 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(16, 23, 59),
    declination: Declination::new(Sgn::Pos, 61, 30, 51),
    distance: Distance {
        m: 87.68 * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
