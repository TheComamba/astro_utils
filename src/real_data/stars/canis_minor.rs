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

const PROCYON: RealData = RealData {
    common_name: "Procyon",
    astronomical_name: "Alpha Canis Minoris",
    constellation: "Canis Minor",
    radius: Some(Distance {
        m: 2.048 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.499 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 2.68,
    apparent_magnitude: 0.40,
    temperature: Some(Temperature { K: 6530. }),
    age: Some(Time {
        s: 1.37 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(7, 39, 18),
    declination: Declination::new(Sgn::Pos, 5, 13, 30),
    distance: Distance {
        m: 11. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 0] = [];
