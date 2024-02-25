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

const ALIOTH: RealData = RealData {
    common_name: "Alioth",
    astronomical_name: "Epsilon Ursae Majoris",
    constellation: "Ursa Major",
    radius: Some(Distance {
        m: 4.14 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.91 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.21,
    apparent_magnitude: 1.76,
    temperature: Temperature { K: 9_020. },
    age: Some(Time {
        s: 0.3 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 54, 2),
    declination: Declination::new(Sgn::Pos, 55, 57, 36),
    distance: Distance {
        m: 81. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0. * BILLION_YEARS.s,
    },
};

const DUBHE: RealData = RealData {
    common_name: "Dubhe",
    astronomical_name: "Alpha Ursae Majoris",
    constellation: "Ursa Major",
    radius: Some(Distance {
        m: 17.03 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 3.44 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -1.08,
    apparent_magnitude: 1.81,
    temperature: Temperature { K: 5012. },
    age: Some(Time {
        s: 0.28 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(11, 3, 44),
    declination: Declination::new(Sgn::Pos, 61, 45, 4),
    distance: Distance {
        m: 124. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0. * BILLION_YEARS.s,
    },
};

const ALKAID: RealData = RealData {
    common_name: "Alkaid",
    astronomical_name: "Eta Ursae Majoris",
    constellation: "Ursa Major",
    radius: Some(Distance {
        m: 3.4 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 6.1 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.60,
    apparent_magnitude: 1.85,
    temperature: Temperature { K: 15_540. },
    age: Some(Time {
        s: 0.01 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(13, 47, 32),
    declination: Declination::new(Sgn::Pos, 49, 18, 48),
    distance: Distance {
        m: 101. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0. * BILLION_YEARS.s,
    },
};

const MIZAR: RealData = RealData {
    common_name: "Mizar",
    astronomical_name: "Zeta Ursae Majoris",
    constellation: "Ursa Major",
    radius: Some(Distance {
        m: 2.4 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.2 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 0.33,
    apparent_magnitude: 2.23,
    temperature: Temperature { K: 9000. },
    age: Some(Time {
        s: 0.37 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(13, 23, 56),
    declination: Declination::new(Sgn::Pos, 54, 55, 31),
    distance: Distance {
        m: 78. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0. * BILLION_YEARS.s,
    },
};

const MERAK: RealData = RealData {
    common_name: "Merak",
    astronomical_name: "Beta Ursae Majoris",
    constellation: "Ursa Major",
    radius: Some(Distance {
        m: 3.021 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.7 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 0.41,
    apparent_magnitude: 2.34,
    temperature: Temperature { K: 9377. },
    age: Some(Time {
        s: 0.5 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(11, 1, 50),
    declination: Declination::new(Sgn::Pos, 56, 22, 57),
    distance: Distance {
        m: 79. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0. * BILLION_YEARS.s,
    },
};

const PHECDA: RealData = RealData {
    common_name: "Phecda",
    astronomical_name: "Gamma Ursae Majoris",
    constellation: "Ursa Major",
    radius: Some(Distance {
        m: 3.04 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.94 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 0.36,
    apparent_magnitude: 2.41,
    temperature: Temperature { K: 9355. },
    age: Some(Time {
        s: 0.3 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(11, 53, 50),
    declination: Declination::new(Sgn::Pos, 53, 41, 41),
    distance: Distance {
        m: 84. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0. * BILLION_YEARS.s,
    },
};

const TANIA_AUSTRALIS: RealData = RealData {
    common_name: "Tania Australis",
    astronomical_name: "Mu Ursae Majoris",
    constellation: "Ursa Major",
    radius: Some(Distance {
        m: 75. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 6.3 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -1.2,
    apparent_magnitude: 3.06,
    temperature: Temperature { K: 3899. },
    age: None,
    right_ascension: RightAscension::new(10, 22, 20),
    declination: Declination::new(Sgn::Pos, 41, 29, 58),
    distance: Distance {
        m: 230.0 * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0. * BILLION_YEARS.s,
    },
};

const MEGREZ: RealData = RealData {
    common_name: "Megrez",
    astronomical_name: "Delta Ursae Majoris",
    constellation: "Ursa Major",
    radius: Some(Distance {
        m: 1.4 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.63 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 1.39,
    apparent_magnitude: 3.312,
    temperature: Temperature { K: 9480. },
    age: Some(Time {
        s: 0.3 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 15, 26),
    declination: Declination::new(Sgn::Pos, 57, 1, 57),
    distance: Distance {
        m: 80.5 * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0. * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 8] = [
    ALIOTH,
    DUBHE,
    ALKAID,
    MIZAR,
    MERAK,
    PHECDA,
    TANIA_AUSTRALIS,
    MEGREZ,
];
