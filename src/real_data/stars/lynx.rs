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

const ALPHA_LYNCIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Lyncis",
    constellation: "Lynx",
    radius: Some(Distance {
        m: 54.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.02,
    apparent_magnitude: 3.15,
    temperature: Some(Temperature { K: 3882. }),
    age: Some(Time {
        s: 1.4 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(9, 21, 3),
    declination: Declination::new(Sgn::Pos, 34, 23, 33),
    distance: Distance {
        m: 221.9 * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
