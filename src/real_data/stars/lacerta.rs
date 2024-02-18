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

const ALPHA_LACERTAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Lacertae",
    constellation: "Lacerta",
    right_ascension: RightAscension::new(22, 31, 18),
    declination: Declination::new(Sgn::Pos, 50, 16, 57),
    apparent_magnitude: 3.76,
    distance: Distance {
        m: 102.6 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.27,
    mass: Some(Mass {
        kg: 2.194 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.1432 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 9050. }),
    age: Some(Time {
        s: 0.4 * BILLION_YEARS.s,
    }),
};

const BETA_LACERTAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Lacertae",
    constellation: "Lacerta",
    right_ascension: RightAscension::new(22, 23, 34),
    declination: Declination::new(Sgn::Pos, 52, 13, 45),
    apparent_magnitude: 4.43,
    distance: Distance {
        m: 170. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.67,
    mass: Some(Mass {
        kg: 0.97 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 10.96 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4803. }),
    age: Some(Time {
        s: 6.76 * BILLION_YEARS.s,
    }),
};

const FIVE_LACERTAE: RealData = RealData {
    common_name: "",
    astronomical_name: "5 Lacertae",
    constellation: "Lacerta",
    right_ascension: RightAscension::new(22, 29, 32),
    declination: Declination::new(Sgn::Pos, 47, 42, 25),
    apparent_magnitude: 4.34,
    distance: Distance {
        m: 1164. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -3.42,
    mass: Some(Mass {
        kg: 5.11 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 319.2 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 3713. }),
    age: Some(Time {
        s: 110. * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_LACERTAE, BETA_LACERTAE, FIVE_LACERTAE];
