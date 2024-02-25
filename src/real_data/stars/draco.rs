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

const ELTANIN: RealData = RealData {
    common_name: "Eltanin",
    astronomical_name: "Gamma Draconis",
    constellation: "Draco",
    radius: Some(Distance {
        m: 48.15 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.72 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -1.04,
    apparent_magnitude: 2.24,
    temperature: Temperature { K: 3930. },
    age: None,
    lifetime: Time {
        s: 1.73766023 * BILLION_YEARS.s,
    },
    right_ascension: RightAscension::new(17, 56, 36),
    declination: Declination::new(Sgn::Pos, 51, 29, 20),
    distance: Distance {
        m: 148. * LIGHT_YEAR.m,
    },
};

const ATHEBYNE: RealData = RealData {
    common_name: "Athebyne",
    astronomical_name: "Eta Draconis",
    constellation: "Draco",
    radius: Some(Distance {
        m: 11. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.55 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 0.58,
    apparent_magnitude: 2.73,
    temperature: Temperature { K: 5055. },
    age: Some(Time {
        s: 0.55 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.63513384 * BILLION_YEARS.s,
    },
    right_ascension: RightAscension::new(16, 23, 59),
    declination: Declination::new(Sgn::Pos, 61, 30, 51),
    distance: Distance {
        m: 87.68 * LIGHT_YEAR.m,
    },
};

const THETA_DRACONIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Theta Draconis",
    constellation: "Draco",
    right_ascension: RightAscension::new(16, 1, 53),
    declination: Declination::new(Sgn::Pos, 58, 33, 55),
    apparent_magnitude: 4.119,
    distance: Distance {
        m: 68.6 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.39,
    mass: Mass {
        kg: 1.53 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 2.83 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 6105. },
    age: Some(Time {
        s: 2.03 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 2.29668629 * BILLION_YEARS.s,
    },
};

const KAPPA_DRACONIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Kappa Draconis",
    constellation: "Draco",
    right_ascension: RightAscension::new(12, 33, 29),
    declination: Declination::new(Sgn::Pos, 69, 47, 18),
    apparent_magnitude: 3.82,
    distance: Distance {
        m: 460. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.95,
    mass: Mass {
        kg: 3.65 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 5.85 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 13_982. },
    age: None,
    lifetime: Time {
        s: 0.254814649 * BILLION_YEARS.s,
    },
};

const TYL: RealData = RealData {
    common_name: "Tyl",
    astronomical_name: "Epsilon Draconis",
    constellation: "Draco",
    right_ascension: RightAscension::new(19, 48, 10),
    declination: Declination::new(Sgn::Pos, 70, 16, 5),
    apparent_magnitude: 3.9974,
    distance: Distance {
        m: 153. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.71,
    mass: Mass {
        kg: 2.7 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 11.15 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4993. },
    age: Some(Time {
        s: 0.5 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.63513384 * BILLION_YEARS.s,
    },
};

const GIAUSAR: RealData = RealData {
    common_name: "Giausar",
    astronomical_name: "Lambda Draconis",
    constellation: "Draco",
    right_ascension: RightAscension::new(11, 31, 24),
    declination: Declination::new(Sgn::Pos, 69, 19, 52),
    apparent_magnitude: 3.85,
    distance: Distance {
        m: 333. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.14,
    mass: Mass {
        kg: 1.7 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 71. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 3958. },
    age: None,
    lifetime: Time {
        s: 1.73766023 * BILLION_YEARS.s,
    },
};

const GRUMIUM: RealData = RealData {
    common_name: "Grumium",
    astronomical_name: "Xi Draconis",
    constellation: "Draco",
    right_ascension: RightAscension::new(17, 53, 32),
    declination: Declination::new(Sgn::Pos, 56, 52, 22),
    apparent_magnitude: 3.75,
    distance: Distance {
        m: 112.5 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.06,
    mass: Mass {
        kg: 1.45 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 12. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4445. },
    age: None,
    lifetime: Time {
        s: 2.82957282 * BILLION_YEARS.s,
    },
};

const THUBAN: RealData = RealData {
    common_name: "Thuban",
    astronomical_name: "Alpha Draconis",
    constellation: "Draco",
    right_ascension: RightAscension::new(14, 4, 23),
    declination: Declination::new(Sgn::Pos, 64, 22, 33),
    apparent_magnitude: 3.67,
    distance: Distance {
        m: 303. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.2,
    mass: Mass {
        kg: 3.186 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 4.932 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 10_225. },
    age: Some(Time {
        s: 0.280 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.351318702 * BILLION_YEARS.s,
    },
};

const CHI_DRACONIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Chi Draconis",
    constellation: "Draco",
    right_ascension: RightAscension::new(18, 21, 3),
    declination: Declination::new(Sgn::Pos, 72, 43, 58),
    apparent_magnitude: 3.570,
    distance: Distance {
        m: 27.17 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 4.04,
    mass: Mass {
        kg: 1.029 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 1.2 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 6150. },
    age: Some(Time {
        s: 5.3 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 8.24015833 * BILLION_YEARS.s,
    },
};

const EDASICH: RealData = RealData {
    common_name: "Edasich",
    astronomical_name: "Iota Draconis",
    constellation: "Draco",
    right_ascension: RightAscension::new(15, 24, 56),
    declination: Declination::new(Sgn::Pos, 58, 57, 58),
    apparent_magnitude: 3.290,
    distance: Distance {
        m: 101.2 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.99,
    mass: Mass {
        kg: 1.56 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 11.99 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4504. },
    age: Some(Time {
        s: 2.2 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 2.29668629 * BILLION_YEARS.s,
    },
};

const ALDHIBAH: RealData = RealData {
    common_name: "Aldhibah",
    astronomical_name: "Zeta Draconis",
    constellation: "Draco",
    right_ascension: RightAscension::new(17, 8, 47),
    declination: Declination::new(Sgn::Pos, 65, 42, 53),
    apparent_magnitude: 3.17,
    distance: Distance {
        m: 330. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.88,
    mass: Mass {
        kg: 3.5 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 2.3 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 13_397. },
    age: None,
    lifetime: Time {
        s: 0.297402042 * BILLION_YEARS.s,
    },
};

const ALTAIS: RealData = RealData {
    common_name: "Altais",
    astronomical_name: "Delta Draconis",
    constellation: "Draco",
    right_ascension: RightAscension::new(19, 12, 33),
    declination: Declination::new(Sgn::Pos, 67, 39, 42),
    apparent_magnitude: 3.07,
    distance: Distance {
        m: 97.4 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.62,
    mass: Mass {
        kg: 2.32 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 11. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4820. },
    age: Some(Time {
        s: 0.8 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.916355612 * BILLION_YEARS.s,
    },
};

const RASTABAN: RealData = RealData {
    common_name: "Rastaban",
    astronomical_name: "Beta Draconis",
    constellation: "Draco",
    right_ascension: RightAscension::new(17, 30, 26),
    declination: Declination::new(Sgn::Pos, 52, 18, 5),
    apparent_magnitude: 2.79,
    distance: Distance {
        m: 380. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -2.457,
    mass: Mass {
        kg: 6. * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 40. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5160. },
    age: Some(Time {
        s: 0.062 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.073299383 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 13] = [
    ELTANIN,
    ATHEBYNE,
    THETA_DRACONIS,
    KAPPA_DRACONIS,
    TYL,
    GIAUSAR,
    GRUMIUM,
    THUBAN,
    CHI_DRACONIS,
    EDASICH,
    ALDHIBAH,
    ALTAIS,
    RASTABAN,
];
