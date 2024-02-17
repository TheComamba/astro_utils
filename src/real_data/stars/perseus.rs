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

const MIRPHAK: RealData = RealData {
    common_name: "Mirphak",
    astronomical_name: "Alpha Persei",
    constellation: "Perseus",
    radius: Some(Distance {
        m: 68. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 8.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -4.50,
    apparent_magnitude: 1.79,
    temperature: Some(Temperature { K: 6350. }),
    age: Some(Time {
        s: 0.041 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(3, 24, 19),
    declination: Declination::new(Sgn::Pos, 49, 51, 40),
    distance: Distance {
        m: 592. * LIGHT_YEAR.m,
    },
};

const ALGOL: RealData = RealData {
    common_name: "Algol",
    astronomical_name: "Beta Persei",
    constellation: "Perseus",
    radius: Some(Distance {
        m: 2.73 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 3.17 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.18,
    apparent_magnitude: 2.09,
    temperature: Some(Temperature { K: 13_000. }),
    age: Some(Time {
        s: 0.57 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(3, 8, 10),
    declination: Declination::new(Sgn::Pos, 40, 57, 20),
    distance: Distance {
        m: 93. * LIGHT_YEAR.m,
    },
};

const GORGONEA_TERTIA: RealData = RealData {
    common_name: "Gorgonea Tertia",
    astronomical_name: "Rho Persei",
    constellation: "Perseus",
    radius: Some(Distance {
        m: 143. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.9 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.67,
    apparent_magnitude: 3.32,
    temperature: Some(Temperature { K: 3479. }),
    age: Some(Time {
        s: 0.440 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(3, 5, 11),
    declination: Declination::new(Sgn::Pos, 38, 50, 25),
    distance: Distance {
        m: 325. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 3] = [MIRPHAK, ALGOL, GORGONEA_TERTIA];
