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

const ALDERAMIN: RealData = RealData {
    common_name: "Alderamin",
    astronomical_name: "Alpha Cephei",
    constellation: "Cepheus",
    radius: Some(Distance {
        m: 2.4 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2. * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 1.58,
    apparent_magnitude: 2.45,
    temperature: Some(Temperature { K: 7700. }),
    age: Some(Time {
        s: 0.82 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(21, 18, 35),
    declination: Declination::new(Sgn::Pos, 62, 35, 8),
    distance: Distance {
        m: 49. * LIGHT_YEAR.m,
    },
};

const ALFIRK: RealData = RealData {
    common_name: "Alfirk",
    astronomical_name: "Beta Cephei",
    constellation: "Cepheus",
    right_ascension: RightAscension::new(21, 28, 40),
    declination: Declination::new(Sgn::Pos, 70, 33, 39),
    apparent_magnitude: 3.2,
    distance: Distance {
        m: 690. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -3.03,
    mass: Some(Mass {
        kg: 7.4 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 5.6 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 27_000. }),
    age: Some(Time {
        s: 0.0087 * BILLION_YEARS.s,
    }),
};

const ERRAI: RealData = RealData {
    common_name: "Errai",
    astronomical_name: "Gamma Cephei",
    constellation: "Cepheus",
    right_ascension: RightAscension::new(23, 39, 21),
    declination: Declination::new(Sgn::Pos, 77, 37, 57),
    apparent_magnitude: 3.21,
    distance: Distance {
        m: 44.98 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.62,
    mass: Some(Mass {
        kg: 1.294 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 4.93 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4792. }),
    age: Some(Time {
        s: 3.25 * BILLION_YEARS.s,
    }),
};

const DELTA_CEPHERI: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Cephei",
    constellation: "Cepheus",
    right_ascension: RightAscension::new(22, 29, 10),
    declination: Declination::new(Sgn::Pos, 58, 24, 55),
    apparent_magnitude: 4.07,
    distance: Distance {
        m: 887. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -3.47,
    mass: Some(Mass {
        kg: 4.5 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 44.5 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 6000. }),
    age: Some(Time {
        s: 0.1 * BILLION_YEARS.s,
    }),
};

const ETA_CEPHEI: RealData = RealData {
    common_name: "",
    astronomical_name: "Eta Cephei",
    constellation: "Cepheus",
    right_ascension: RightAscension::new(20, 45, 17),
    declination: Declination::new(Sgn::Pos, 61, 50, 20),
    apparent_magnitude: 3.426,
    distance: Distance {
        m: 46.53 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.631,
    mass: Some(Mass {
        kg: 1.6 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 4.12 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4950. }),
    age: Some(Time {
        s: 2.5 * BILLION_YEARS.s,
    }),
};

const IOTA_CEPHEI: RealData = RealData {
    common_name: "",
    astronomical_name: "Ioata Cephei",
    constellation: "Cepheus",
    right_ascension: RightAscension::new(22, 49, 41),
    declination: Declination::new(Sgn::Pos, 66, 12, 1),
    apparent_magnitude: 3.507,
    distance: Distance {
        m: 115.3 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.76,
    mass: Some(Mass {
        kg: 2.15 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 11.08 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4768. }),
    age: Some(Time {
        s: 1.2 * BILLION_YEARS.s,
    }),
};

const ZETA_CEPHEI: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeata Cephei",
    constellation: "Cepheus",
    right_ascension: RightAscension::new(22, 10, 51),
    declination: Declination::new(Sgn::Pos, 58, 12, 5),
    apparent_magnitude: 3.35,
    distance: Distance {
        m: 990. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -4.7,
    mass: None,
    radius: Some(Distance {
        m: 94. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4072. }),
    age: None,
};

const ERAKIS: RealData = RealData {
    common_name: "Erakis",
    astronomical_name: "Mu Cephei",
    constellation: "Cepheus",
    radius: Some(Distance {
        m: 972. * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 19.2 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: -6.5,
    apparent_magnitude: 3.43,
    temperature: Some(Temperature { K: 3551. }),
    age: Some(Time {
        s: 0.01 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(21, 43, 30),
    declination: Declination::new(Sgn::Pos, 58, 46, 48),
    distance: Distance {
        m: 3066. * LIGHT_YEAR.m,
    },
};

pub(crate) const STARS: [RealData; 8] = [
    ALDERAMIN,
    ALFIRK,
    ERRAI,
    DELTA_CEPHERI,
    ETA_CEPHEI,
    IOTA_CEPHEI,
    ZETA_CEPHEI,
    ERAKIS,
];
