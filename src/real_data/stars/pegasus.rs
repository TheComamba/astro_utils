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

const ENIF: RealData = RealData {
    common_name: "Enif",
    astronomical_name: "Epsilon Pegasi",
    constellation: "Pegasus",
    radius: Some(Distance {
        m: 211. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 7.07 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -4.19,
    apparent_magnitude: 2.38,
    temperature: Temperature { K: 3963. },
    age: Some(Time {
        s: 0.020 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(21, 44, 11),
    declination: Declination::new(Sgn::Pos, 9, 52, 30),
    distance: Distance {
        m: 672. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0. * BILLION_YEARS.s,
    },
};

const SCHEAT: RealData = RealData {
    common_name: "Scheat",
    astronomical_name: "Beta Pegasi",
    constellation: "Pegasus",
    radius: Some(Distance {
        m: 95. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.1 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -1.49,
    apparent_magnitude: 2.44,
    temperature: Temperature { K: 3689. },
    age: None,
    right_ascension: RightAscension::new(23, 3, 46),
    declination: Declination::new(Sgn::Pos, 28, 4, 58),
    distance: Distance {
        m: 199. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0. * BILLION_YEARS.s,
    },
};

const MARKAB: RealData = RealData {
    common_name: "Markab",
    astronomical_name: "Alpha Pegasi",
    constellation: "Pegasus",
    radius: Some(Distance {
        m: 4.62 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 3.5 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.67,
    apparent_magnitude: 2.49,
    temperature: Temperature { K: 10_100. },
    age: Some(Time {
        s: 0.2 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(23, 4, 46),
    declination: Declination::new(Sgn::Pos, 15, 12, 19),
    distance: Distance {
        m: 140. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0. * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [ENIF, SCHEAT, MARKAB];
