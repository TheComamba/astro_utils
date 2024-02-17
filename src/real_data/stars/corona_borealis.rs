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

const ALPHECCA: RealData = RealData {
    common_name: "Alphecca",
    astronomical_name: "Alpha Coronae Borealis",
    constellation: "Corona Borealis",
    radius: Some(Distance {
        m: 3. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.58 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.42,
    apparent_magnitude: 2.22,
    temperature: Some(Temperature { K: 9700. }),
    age: Some(Time {
        s: 0.314 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(15, 34, 41),
    declination: Declination::new(Sgn::Pos, 26, 42, 53),
    distance: Distance {
        m: 75. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
