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

const TARF: RealData = RealData {
    common_name: "Tarf",
    astronomical_name: "Beta Cancri",
    constellation: "Cancer",
    radius: Some(Distance {
        m: 47.2 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.7 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -1.218,
    apparent_magnitude: 3.50,
    temperature: Temperature { K: 4092. },
    right_ascension: RightAscension::new(8, 16, 31),
    declination: Declination::new(Sgn::Pos, 9, 11, 8),
    distance: Distance {
        m: 290. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 1.85 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.73766023 * BILLION_YEARS.s,
    },
};

const ASELLUS_AUSTRALIS: RealData = RealData {
    common_name: "Asellus Australis",
    astronomical_name: "Delta Cancri",
    constellation: "Cancer",
    radius: Some(Distance {
        m: 11. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.71 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 0.843,
    apparent_magnitude: 3.94,
    temperature: Temperature { K: 4637. },
    right_ascension: RightAscension::new(8, 44, 41),
    declination: Declination::new(Sgn::Pos, 18, 9, 16),
    distance: Distance {
        m: 131. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 2.45 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.73766023 * BILLION_YEARS.s,
    },
};

const IOTA_CANCRI: RealData = RealData {
    common_name: "",
    astronomical_name: "Iota Cancri",
    constellation: "Cancer",
    radius: Some(Distance {
        m: 21. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 3.43 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.79,
    apparent_magnitude: 4.02,
    temperature: Temperature { K: 4954. },
    right_ascension: RightAscension::new(8, 46, 42),
    declination: Declination::new(Sgn::Pos, 28, 45, 36),
    distance: Distance {
        m: 330. * LIGHT_YEAR.m,
    },
    age: None,
    lifetime: Time {
        s: 0.297402042 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [TARF, ASELLUS_AUSTRALIS, IOTA_CANCRI];
