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

const FOURTYSIX_LEONIS_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "46 Leonis Minoris",
    constellation: "Leo Minor",
    right_ascension: RightAscension::new(10, 53, 19),
    declination: Declination::new(Sgn::Pos, 34, 12, 54),
    apparent_magnitude: 3.83,
    distance: Distance {
        m: 94.9 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.45,
    mass: Some(Mass {
        kg: 1.69 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 8.22 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4670. },
    age: Some(Time {
        s: 6.76 * BILLION_YEARS.s,
    }),
};

const BETA_LEONIS_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Leonis Minoris",
    constellation: "Leo Minor",
    right_ascension: RightAscension::new(10, 27, 53),
    declination: Declination::new(Sgn::Pos, 36, 42, 26),
    apparent_magnitude: 4.21,
    distance: Distance {
        m: 154. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.85,
    mass: Some(Mass {
        kg: 2.98 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 9.4 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4097. },
    age: Some(Time {
        s: 1.2 * BILLION_YEARS.s,
    }),
};

const TWENTYFOUR_LEONIS_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "21 Leonis Minoris",
    constellation: "Leo Minor",
    right_ascension: RightAscension::new(10, 7, 26),
    declination: Declination::new(Sgn::Pos, 35, 14, 41),
    apparent_magnitude: 4.5,
    distance: Distance {
        m: 92.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.43,
    mass: Some(Mass {
        kg: 1.75 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.75 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 7839. },
    age: Some(Time {
        s: 0.390 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 3] = [
    BETA_LEONIS_MINORIS,
    FOURTYSIX_LEONIS_MINORIS,
    TWENTYFOUR_LEONIS_MINORIS,
];
