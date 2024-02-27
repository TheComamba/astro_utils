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

const PHACT: RealData = RealData {
    common_name: "Phact",
    astronomical_name: "α Columbae",
    constellation: "Columba",
    right_ascension: RightAscension::new(5, 39, 39),
    declination: Declination::new(Sgn::Neg, 34, 4, 27),
    apparent_magnitude: 2.645,
    distance: Distance {
        m: 261. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.87,
    mass: Mass {
        kg: 4.5 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 5.8 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 12_963. },
    age: Some(Time {
        s: 0.093 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.151849866 * BILLION_YEARS.s,
    },
};

const WAZN: RealData = RealData {
    common_name: "Wazn",
    astronomical_name: "β Columbae",
    constellation: "Columba",
    right_ascension: RightAscension::new(5, 50, 58),
    declination: Declination::new(Sgn::Neg, 35, 46, 6),
    apparent_magnitude: 3.105,
    distance: Distance {
        m: 87.41 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.01,
    mass: Mass {
        kg: 1.1 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 11.5 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4545. },
    age: Some(Time {
        s: 2. * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 6.97272616 * BILLION_YEARS.s,
    },
};

const DELTA_COLUMBAE: RealData = RealData {
    common_name: "",
    astronomical_name: "δ Columbae",
    constellation: "Columba",
    right_ascension: RightAscension::new(6, 22, 7),
    declination: Declination::new(Sgn::Neg, 33, 26, 11),
    apparent_magnitude: 3.85,
    distance: Distance {
        m: 234. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.32,
    mass: Mass {
        kg: 3.7 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 5136. },
    age: None,
    lifetime: Time {
        s: 0.254814649 * BILLION_YEARS.s,
    },
};

const EPSILON_COLUMBAE: RealData = RealData {
    common_name: "",
    astronomical_name: "ε Columbae",
    constellation: "Columba",
    right_ascension: RightAscension::new(5, 31, 13),
    declination: Declination::new(Sgn::Neg, 35, 28, 14),
    apparent_magnitude: 3.87,
    distance: Distance {
        m: 262. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.67,
    mass: Mass {
        kg: 2.47 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 25.2 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4575. },
    age: Some(Time {
        s: 0.8 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.800458342 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 4] = [PHACT, WAZN, DELTA_COLUMBAE, EPSILON_COLUMBAE];
