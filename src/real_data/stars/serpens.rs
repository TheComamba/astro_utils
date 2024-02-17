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

const UNUKALHAI: RealData = RealData {
    common_name: "Unukalhai",
    astronomical_name: "Alpha Serpentis",
    constellation: "Serpens",
    radius: Some(Distance {
        m: 13.48 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.66 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.88,
    apparent_magnitude: 2.63,
    temperature: Some(Temperature { K: 4498. }),
    age: None,
    right_ascension: RightAscension::new(15, 44, 16),
    declination: Declination::new(Sgn::Pos, 6, 25, 32),
    distance: Distance {
        m: 74. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
