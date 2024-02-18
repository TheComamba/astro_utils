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

const SPICA: RealData = RealData {
    common_name: "Spica",
    astronomical_name: "Alpha Virginis",
    constellation: "Virgo",
    radius: Some(Distance {
        m: 7.47 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 11.43 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -3.55,
    apparent_magnitude: 0.98,
    temperature: Temperature { K: 22_300. },
    age: Some(Time {
        s: 0.0125 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(13, 25, 12),
    declination: Declination::new(Sgn::Neg, 11, 9, 41),
    distance: Distance {
        m: 262. * LIGHT_YEAR.m,
    },
};

const MINELAUVA: RealData = RealData {
    common_name: "Minelauva",
    astronomical_name: "Delta Virginis",
    constellation: "Virgo",
    radius: Some(Distance {
        m: 48. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.4 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -0.575,
    apparent_magnitude: 3.39,
    temperature: Temperature { K: 3999. },
    age: None,
    right_ascension: RightAscension::new(12, 55, 36),
    declination: Declination::new(Sgn::Pos, 3, 23, 51),
    distance: Distance {
        m: 202.4 * LIGHT_YEAR.m,
    },
};

const ZAVIJAVA: RealData = RealData {
    common_name: "Zavijava",
    astronomical_name: "Beta Virginis",
    constellation: "Virgo",
    right_ascension: RightAscension::new(11, 50, 42),
    declination: Declination::new(Sgn::Pos, 1, 45, 53),
    apparent_magnitude: 3.604,
    distance: Distance {
        m: 35.65 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.41,
    mass: Some(Mass {
        kg: 1.413 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.681 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 6132. },
    age: Some(Time {
        s: 2.9 * BILLION_YEARS.s,
    }),
};

const SYRMA: RealData = RealData {
    common_name: "Syrma",
    astronomical_name: "Iota Virginis",
    constellation: "Virgo",
    right_ascension: RightAscension::new(14, 16, 1),
    declination: Declination::new(Sgn::Neg, 6, 0, 2),
    apparent_magnitude: 4.08,
    distance: Distance {
        m: 72.5 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.4,
    mass: Some(Mass {
        kg: 1.5 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.5 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 6282. },
    age: None,
};

const HEZE: RealData = RealData {
    common_name: "Heze",
    astronomical_name: "Zeta Virginis",
    constellation: "Virgo",
    right_ascension: RightAscension::new(13, 34, 42),
    declination: Declination::new(Sgn::Neg, 0, 35, 45),
    apparent_magnitude: 3.376,
    distance: Distance {
        m: 74.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.64,
    mass: Some(Mass {
        kg: 2.041 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.079 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8247. },
    age: Some(Time {
        s: 0.51 * BILLION_YEARS.s,
    }),
};

const VINDEMIATRIX: RealData = RealData {
    common_name: "Vindemiatrix",
    astronomical_name: "Epsilon Virginis",
    constellation: "Virgo",
    right_ascension: RightAscension::new(13, 2, 11),
    declination: Declination::new(Sgn::Pos, 10, 57, 33),
    apparent_magnitude: 2.826,
    distance: Distance {
        m: 109.6 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.37,
    mass: Some(Mass {
        kg: 2.64 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 10.6 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5086. },
    age: Some(Time {
        s: 0.560 * BILLION_YEARS.s,
    }),
};

const PORRIMA: RealData = RealData {
    common_name: "Porrima",
    astronomical_name: "Gamma Virginis",
    constellation: "Virgo",
    right_ascension: RightAscension::new(12, 41, 40),
    declination: Declination::new(Sgn::Neg, 1, 26, 58),
    apparent_magnitude: 2.74,
    distance: Distance {
        m: 38.1 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.41,
    mass: Some(Mass {
        kg: 1.56 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Temperature { K: 6757. },
    age: Some(Time {
        s: 1.14 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 7] = [
    SPICA,
    MINELAUVA,
    ZAVIJAVA,
    SYRMA,
    HEZE,
    VINDEMIATRIX,
    PORRIMA,
];
