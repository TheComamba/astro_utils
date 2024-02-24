use crate::{
    coordinates::{
        declination::{Declination, Sgn},
        right_ascension::RightAscension,
    },
    stars::{fate::StarFate, real_data::RealData},
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};
use simple_si_units::base::{Distance, Mass, Temperature, Time};

const RIGEL_KENTAURUS: RealData = RealData {
    common_name: "Rigel Kentaurus",
    astronomical_name: "Alpha Centauri",
    constellation: "Centaurus",
    radius: Some(Distance {
        m: 1.2175 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.0788 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 4.34,
    apparent_magnitude: -0.27,
    temperature: Temperature { K: 5790. },
    age: Some(Time {
        s: 4.85 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 10. * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
    right_ascension: RightAscension::new(14, 39, 36),
    declination: Declination::new(Sgn::Neg, 60, 50, 2),
    distance: Distance {
        m: 4. * LIGHT_YEAR.m,
    },
};

const HADAR: RealData = RealData {
    common_name: "Hadar",
    astronomical_name: "Beta Centauri",
    constellation: "Centaurus",
    radius: Some(Distance {
        m: 9. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 12.02 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -5.42,
    apparent_magnitude: 0.61,
    temperature: Temperature { K: 25_000. },
    age: Some(Time {
        s: 0.0141 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 7.9 * BILLION_YEARS.s,
    },
    fate: StarFate::TypeIISupernova,
    right_ascension: RightAscension::new(14, 3, 49),
    declination: Declination::new(Sgn::Neg, 60, 22, 23),
    distance: Distance {
        m: 526. * LIGHT_YEAR.m,
    },
};

const MENKENT: RealData = RealData {
    common_name: "Menkent",
    astronomical_name: "Theta Centauri",
    constellation: "Centaurus",
    radius: Some(Distance {
        m: 10.6 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.27 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.70,
    apparent_magnitude: 2.06,
    temperature: Temperature { K: 4980. },
    age: None,
    lifetime: Time {
        s: 7.9 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
    right_ascension: RightAscension::new(14, 6, 41),
    declination: Declination::new(Sgn::Neg, 36, 22, 11),
    distance: Distance {
        m: 61. * LIGHT_YEAR.m,
    },
};

const MUHLIFAIN: RealData = RealData {
    common_name: "Muhlifain",
    astronomical_name: "Gamma Centauri",
    constellation: "Centaurus",
    radius: None,
    mass: Some(Mass {
        kg: 2.91 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.81,
    apparent_magnitude: 2.20,
    temperature: Temperature { K: 9082. },
    age: None,
    lifetime: Time {
        s: 7.9 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
    right_ascension: RightAscension::new(12, 41, 31),
    declination: Declination::new(Sgn::Neg, 48, 57, 35),
    distance: Distance {
        m: 130. * LIGHT_YEAR.m,
    },
};

const EPSILON_CENTAURI: RealData = RealData {
    common_name: "",
    astronomical_name: "Epsilon Centauri",
    constellation: "Centaurus",
    radius: None,
    mass: Some(Mass {
        kg: 11.6 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.02,
    apparent_magnitude: 2.29,
    temperature: Temperature { K: 24_000. },
    age: Some(Time {
        s: 0.0158 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 7.9 * BILLION_YEARS.s,
    },
    fate: StarFate::TypeIISupernova,
    right_ascension: RightAscension::new(13, 39, 53),
    declination: Declination::new(Sgn::Neg, 53, 27, 59),
    distance: Distance {
        m: 376. * LIGHT_YEAR.m,
    },
};

const ETA_CENTAURI: RealData = RealData {
    common_name: "",
    astronomical_name: "Eta Centauri",
    constellation: "Centaurus",
    radius: Some(Distance {
        m: 6.1 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 12.0 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.55,
    apparent_magnitude: 2.29,
    temperature: Temperature { K: 25_700. },
    age: Some(Time {
        s: 0.0056 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.013 * BILLION_YEARS.s,
    },
    fate: StarFate::TypeIISupernova,
    right_ascension: RightAscension::new(14, 35, 30),
    declination: Declination::new(Sgn::Neg, 42, 9, 28),
    distance: Distance {
        m: 308. * LIGHT_YEAR.m,
    },
};

const ZETA_CENTAURI: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeta Centauri",
    constellation: "Centaurus",
    radius: Some(Distance {
        m: 5.8 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 7.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.81,
    apparent_magnitude: 2.55,
    temperature: Temperature { K: 23_561. },
    age: Some(Time {
        s: 0.04 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.041 * BILLION_YEARS.s, //guessed
    },
    fate: StarFate::WhiteDwarf,
    right_ascension: RightAscension::new(13, 55, 33),
    declination: Declination::new(Sgn::Neg, 47, 17, 18),
    distance: Distance {
        m: 384. * LIGHT_YEAR.m,
    },
};

const MA_WEI: RealData = RealData {
    common_name: "Ma Wei",
    astronomical_name: "Delta Centauri",
    constellation: "Centaurus",
    radius: Some(Distance {
        m: 6.5 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 8.7 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -2.84,
    apparent_magnitude: 2.58,
    temperature: Temperature { K: 22_360. },
    age: Some(Time {
        s: 0.02 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.021 * BILLION_YEARS.s, //guessed
    },
    fate: StarFate::TypeIISupernova,
    right_ascension: RightAscension::new(12, 8, 21),
    declination: Declination::new(Sgn::Neg, 50, 43, 21),
    distance: Distance {
        m: 395. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 8] = [
    RIGEL_KENTAURUS,
    HADAR,
    MENKENT,
    MUHLIFAIN,
    EPSILON_CENTAURI,
    ETA_CENTAURI,
    ZETA_CENTAURI,
    MA_WEI,
];
