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

const VEGA: RealData = RealData {
    common_name: "Vega",
    astronomical_name: "Alpha Lyrae",
    constellation: "Lyra",
    radius: Some(Distance {
        m: 2.362 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.135 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.58,
    apparent_magnitude: 0.03,
    temperature: Some(Temperature { K: 9602. }),
    age: Some(Time {
        s: 0.455 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(18, 36, 56),
    declination: Declination::new(Sgn::Pos, 38, 47, 1),
    distance: Distance {
        m: 25. * LIGHT_YEAR.m,
    },
};

const R_LYRAE: RealData = RealData {
    common_name: "",
    astronomical_name: "R Lyrae",
    constellation: "Lyra",
    radius: None,
    mass: Some(Mass {
        kg: 1.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.07,
    apparent_magnitude: 4.08,
    temperature: Some(Temperature { K: 3313. }),
    age: None,
    right_ascension: RightAscension::new(18, 55, 20),
    declination: Declination::new(Sgn::Pos, 43, 56, 46),
    distance: Distance {
        m: 349.4 * LIGHT_YEAR.m,
    },
};

const GAMMA_LYRAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Lyrae",
    constellation: "Lyra",
    right_ascension: RightAscension::new(18, 58, 57),
    declination: Declination::new(Sgn::Pos, 32, 41, 22),
    apparent_magnitude: 3.261,
    distance: Distance {
        m: 620. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -3.140,
    mass: Some(Mass {
        kg: 5.76 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 15.40 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 10_000. }),
    age: None,
};

const BETA_LYRAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Lyrae",
    constellation: "Lyra",
    right_ascension: RightAscension::new(18, 50, 5),
    declination: Declination::new(Sgn::Pos, 33, 21, 46),
    apparent_magnitude: 3.52,
    distance: Distance {
        m: 960. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -3.82,
    mass: Some(Mass {
        kg: 2.97 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 15.2 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 13_300. }),
    age: Some(Time {
        s: 0.023 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 4] = [VEGA, R_LYRAE, GAMMA_LYRAE, BETA_LYRAE];
