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

const ALPHA_CAELI: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Caeli",
    constellation: "Caelum",
    radius: Some(Distance {
        m: 1.3 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.48 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 2.92,
    apparent_magnitude: 4.44,
    temperature: Temperature { K: 6991. },
    age: Some(Time {
        s: 0.9 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 40, 34),
    declination: Declination::new(Sgn::Neg, 41, 51, 50),
    distance: Distance {
        m: 65.63 * LIGHT_YEAR.m,
    },
};

const GAMMA1_CAELI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma1 Caeli",
    constellation: "Caelum",
    radius: Some(Distance {
        m: 14.31 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.781,
    apparent_magnitude: 4.57,
    temperature: Temperature { K: 4411. },
    age: None,
    right_ascension: RightAscension::new(5, 4, 24),
    declination: Declination::new(Sgn::Neg, 35, 28, 59),
    distance: Distance {
        m: 185. * LIGHT_YEAR.m,
    },
};

const BETA_CAELI: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Caeli",
    constellation: "Caelum",
    radius: Some(Distance {
        m: 1.3 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.32 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 2.64,
    apparent_magnitude: 5.04,
    temperature: Temperature { K: 6763. },
    age: Some(Time {
        s: 1.753 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(4, 42, 3),
    declination: Declination::new(Sgn::Neg, 37, 8, 39),
    distance: Distance {
        m: 94. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_CAELI, GAMMA1_CAELI, BETA_CAELI];
