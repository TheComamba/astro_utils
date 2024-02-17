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

const HAMAL: RealData = RealData {
    common_name: "Hamal",
    astronomical_name: "Alpha Arietis",
    constellation: "Aries",
    radius: Some(Distance {
        m: 14.9 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.48,
    apparent_magnitude: 2.01,
    temperature: Some(Temperature { K: 4480. }),
    age: Some(Time {
        s: 3.4 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(2, 7, 10),
    declination: Declination::new(Sgn::Pos, 23, 27, 45),
    distance: Distance {
        m: 66. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
