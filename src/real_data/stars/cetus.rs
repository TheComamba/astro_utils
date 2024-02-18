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

const DIPHDA: RealData = RealData {
    common_name: "Diphda",
    astronomical_name: "Beta Ceti",
    constellation: "Cetus",
    radius: Some(Distance {
        m: 16.78 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.8 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.30,
    apparent_magnitude: 2.04,
    temperature: Temperature { K: 4797. },
    age: Some(Time {
        s: 1. * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(0, 43, 35),
    declination: Declination::new(Sgn::Neg, 17, 59, 12),
    distance: Distance {
        m: 96. * LIGHT_YEAR.m,
    },
};

const MENKAR: RealData = RealData {
    common_name: "Menkar",
    astronomical_name: "Alpha Ceti",
    constellation: "Cetus",
    radius: Some(Distance {
        m: 89. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.3 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -1.61,
    apparent_magnitude: 2.54,
    temperature: Temperature { K: 3795. },
    age: None,
    right_ascension: RightAscension::new(3, 2, 17),
    declination: Declination::new(Sgn::Pos, 4, 5, 23),
    distance: Distance {
        m: 220. * LIGHT_YEAR.m,
    },
};

const MIRA: RealData = RealData {
    common_name: "Mira",
    astronomical_name: "Omicron Ceti",
    constellation: "Cetus",
    right_ascension: RightAscension::new(2, 19, 21),
    declination: Declination::new(Sgn::Neg, 2, 58, 39),
    apparent_magnitude: 6.47,
    distance: Distance {
        m: 418.5 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.928,
    mass: Some(Mass {
        kg: 1.18 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 350. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 3000. },
    age: Some(Time {
        s: 6. * BILLION_YEARS.s,
    }),
};

const BATEN_KAITOS: RealData = RealData {
    common_name: "Baten Kaitos",
    astronomical_name: "Zeta Ceti",
    constellation: "Cetus",
    right_ascension: RightAscension::new(1, 51, 28),
    declination: Declination::new(Sgn::Neg, 10, 20, 6),
    apparent_magnitude: 3.742,
    distance: Distance {
        m: 235. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.54,
    mass: Some(Mass {
        kg: 2.34 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 25. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4581. },
    age: Some(Time {
        s: 1.24 * BILLION_YEARS.s,
    }),
};

const GAMMA_CETI: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Ceti",
    constellation: "Cetus",
    right_ascension: RightAscension::new(2, 43, 18),
    declination: Declination::new(Sgn::Pos, 3, 14, 9),
    apparent_magnitude: 3.47,
    distance: Distance {
        m: 80. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.53,
    mass: Some(Mass {
        kg: 1.88 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.9 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8551. },
    age: Some(Time {
        s: 0.647 * BILLION_YEARS.s,
    }),
};

const IOTA_CETI: RealData = RealData {
    common_name: "",
    astronomical_name: "Iota Ceti",
    constellation: "Cetus",
    right_ascension: RightAscension::new(0, 19, 26),
    declination: Declination::new(Sgn::Neg, 8, 49, 26),
    apparent_magnitude: 3.562,
    distance: Distance {
        m: 275. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.2,
    mass: Some(Mass {
        kg: 2.78 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 34. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4446. },
    age: Some(Time {
        s: 2.23 * BILLION_YEARS.s,
    }),
};

const DENEB_ALGENUBI: RealData = RealData {
    common_name: "Deneb Algenubi",
    astronomical_name: "Eta Ceti",
    constellation: "Cetus",
    right_ascension: RightAscension::new(1, 8, 35),
    declination: Declination::new(Sgn::Neg, 10, 10, 56),
    apparent_magnitude: 3.446,
    distance: Distance {
        m: 123.9 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.68,
    mass: Some(Mass {
        kg: 1.84 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 15.10 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4543. },
    age: Some(Time {
        s: 1.8 * BILLION_YEARS.s,
    }),
};

const TAU_CETI: RealData = RealData {
    common_name: "",
    astronomical_name: "Tau Ceti",
    constellation: "Cetus",
    right_ascension: RightAscension::new(1, 44, 4),
    declination: Declination::new(Sgn::Neg, 15, 56, 15),
    apparent_magnitude: 3.50,
    distance: Distance {
        m: 11.912 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 5.69,
    mass: Some(Mass {
        kg: 0.783 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 0.793 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5320. },
    age: Some(Time {
        s: 9. * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 8] = [
    DIPHDA,
    MENKAR,
    MIRA,
    BATEN_KAITOS,
    GAMMA_CETI,
    IOTA_CETI,
    DENEB_ALGENUBI,
    TAU_CETI,
];
