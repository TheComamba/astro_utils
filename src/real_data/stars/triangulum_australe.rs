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

const ATRIA: RealData = RealData {
    common_name: "Atria",
    astronomical_name: "Alpha Trianguli Australis",
    constellation: "Triangulum Australe",
    radius: Some(Distance {
        m: 143. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.62,
    apparent_magnitude: 1.91,
    temperature: Some(Temperature { K: 4150. }),
    age: Some(Time {
        s: 0.048 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(16, 48, 40),
    declination: Declination::new(Sgn::Neg, 69, 1, 40),
    distance: Distance {
        m: 415. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
