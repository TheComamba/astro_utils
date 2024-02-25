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

const GAMMA_SAGITTAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Sagittae",
    constellation: "Sagitta",
    right_ascension: RightAscension::new(19, 58, 45),
    declination: Declination::new(Sgn::Pos, 19, 29, 32),
    apparent_magnitude: 3.47,
    distance: Distance {
        m: 288. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.11,
    mass: Mass {
        kg: 0.88 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 55.13 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 3862. },
    age: Some(Time {
        s: 2.35 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 14.2493142 * BILLION_YEARS.s,
    },
};

const DELTA_SAGITTAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Sagittae",
    constellation: "Sagitta",
    right_ascension: RightAscension::new(19, 47, 23),
    declination: Declination::new(Sgn::Pos, 18, 32, 4),
    apparent_magnitude: 3.82,
    distance: Distance {
        m: 550. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.58,
    mass: Mass {
        kg: 3.35 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 108. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 3660. },
    age: None,
    lifetime: Time {
        s: 0.297402042 * BILLION_YEARS.s,
    },
};

const ALPHA_SAGITTAE: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Sagittae",
    constellation: "Sagitta",
    right_ascension: RightAscension::new(19, 40, 6),
    declination: Declination::new(Sgn::Pos, 18, 0, 50),
    apparent_magnitude: 4.38,
    distance: Distance {
        m: 382. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.96,
    mass: Mass {
        kg: 4.11 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 21. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5333. },
    age: Some(Time {
        s: 0.151 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.170765802 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [GAMMA_SAGITTAE, DELTA_SAGITTAE, ALPHA_SAGITTAE];
