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

const ALPHA_DORADUS: RealData = RealData {
    common_name: "",
    astronomical_name: "α Doradus",
    constellation: "Dorado",
    right_ascension: RightAscension::new(4, 33, 60),
    declination: Declination::new(Sgn::Neg, 55, 2, 42),
    apparent_magnitude: 3.27,
    distance: Distance {
        m: 169. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.357,
    mass: Mass {
        kg: 3.33 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 3.5 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 11_588. },
    age: None,
    lifetime: Time {
        s: 0.297402042 * BILLION_YEARS.s,
    },
};

const BETA_DORADUS: RealData = RealData {
    common_name: "",
    astronomical_name: "β Doradus",
    constellation: "Dorado",
    right_ascension: RightAscension::new(5, 33, 38),
    declination: Declination::new(Sgn::Neg, 62, 29, 23),
    apparent_magnitude: 3.5,
    distance: Distance {
        m: 1050. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -3.91,
    mass: Mass {
        kg: 7.7 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 67.8 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5445. },
    age: Some(Time {
        s: 0.04 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.040555762 * BILLION_YEARS.s,
    },
};

const GAMMA_DORADUS: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Doradus",
    constellation: "Dorado",
    right_ascension: RightAscension::new(4, 16, 2),
    declination: Declination::new(Sgn::Neg, 51, 29, 12),
    apparent_magnitude: 4.25,
    distance: Distance {
        m: 66.7 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.72,
    mass: Mass {
        kg: 1.56 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 1.85 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 6906. },
    age: Some(Time {
        s: 0.535 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 2.29668629 * BILLION_YEARS.s,
    },
};

const R_DORADUS: RealData = RealData {
    common_name: "",
    astronomical_name: "R Doradus",
    constellation: "",
    radius: Some(Distance {
        m: 298. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 0.8 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 1.61,
    apparent_magnitude: 5.59,
    temperature: Temperature { K: 2710. },
    age: Some(Time {
        s: 4. * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 21.4199307 * BILLION_YEARS.s,
    },
    right_ascension: RightAscension::new(4, 36, 46),
    declination: Declination::new(Sgn::Neg, 62, 4, 38),
    distance: Distance {
        m: 203.5 * LIGHT_YEAR.m,
    },
};
pub(crate) const STARS: [RealData; 4] = [ALPHA_DORADUS, BETA_DORADUS, GAMMA_DORADUS, R_DORADUS];
