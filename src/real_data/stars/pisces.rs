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

const ALPHA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(2, 2, 3),
    declination: Declination::new(Sgn::Pos, 2, 45, 50),
    apparent_magnitude: 3.82,
    distance: Distance {
        m: 151. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.5,
    mass: Some(Mass {
        kg: 2.55 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 2.45 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 10_233. }),
    age: None,
};

const DELTA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Delta Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(0, 48, 41),
    declination: Declination::new(Sgn::Pos, 7, 35, 6),
    apparent_magnitude: 4.416,
    distance: Distance {
        m: 311. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.46,
    mass: Some(Mass {
        kg: 1.65 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 44. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 3963. }),
    age: Some(Time {
        s: 0.00298 * BILLION_YEARS.s,
    }),
};

const NU_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Nu Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(1, 41, 26),
    declination: Declination::new(Sgn::Pos, 5, 29, 15),
    apparent_magnitude: 4.44,
    distance: Distance {
        m: 363. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.78,
    mass: Some(Mass {
        kg: 1.66 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 34. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4154. }),
    age: Some(Time {
        s: 3.41 * BILLION_YEARS.s,
    }),
};

const IOTA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Iota Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(23, 39, 57),
    declination: Declination::new(Sgn::Pos, 5, 37, 35),
    apparent_magnitude: 4.13,
    distance: Distance {
        m: 44.73 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.43,
    mass: None,
    radius: Some(Distance {
        m: 1.595 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 6288. }),
    age: Some(Time {
        s: 5.2 * BILLION_YEARS.s,
    }),
};

const OMICRON_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Omicron Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(1, 45, 24),
    declination: Declination::new(Sgn::Pos, 9, 9, 28),
    apparent_magnitude: 4.27,
    distance: Distance {
        m: 280. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.22,
    mass: Some(Mass {
        kg: 3.03 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 14.57 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 5004. }),
    age: Some(Time {
        s: 0.390 * BILLION_YEARS.s,
    }),
};

const EPSILON_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Epislon Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(1, 2, 57),
    declination: Declination::new(Sgn::Pos, 7, 53, 24),
    apparent_magnitude: 4.27,
    distance: Distance {
        m: 182. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.44,
    mass: Some(Mass {
        kg: 2.27 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 10.9 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4814. }),
    age: Some(Time {
        s: 2.56 * BILLION_YEARS.s,
    }),
};

const THETA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Theta Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(23, 27, 58),
    declination: Declination::new(Sgn::Pos, 6, 22, 44),
    apparent_magnitude: 4.27,
    distance: Distance {
        m: 149. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.83,
    mass: Some(Mass {
        kg: 1.58 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 11. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4684. }),
    age: Some(Time {
        s: 0.00245 * BILLION_YEARS.s,
    }),
};

const ETA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Eta Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(1, 31, 29),
    declination: Declination::new(Sgn::Pos, 15, 20, 45),
    apparent_magnitude: 3.611,
    distance: Distance {
        m: 350. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.52,
    mass: Some(Mass {
        kg: 3.78 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 26.48 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4937. }),
    age: Some(Time {
        s: 0.220 * BILLION_YEARS.s,
    }),
};

const GAMMA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(23, 17, 10),
    declination: Declination::new(Sgn::Pos, 3, 16, 56),
    apparent_magnitude: 3.699,
    distance: Distance {
        m: 135. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.68,
    mass: Some(Mass {
        kg: 0.97 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 11.28 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4833. }),
    age: Some(Time {
        s: 4.58 * BILLION_YEARS.s,
    }),
};

const OMEGA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "Omega Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(23, 59, 19),
    declination: Declination::new(Sgn::Pos, 6, 51, 48),
    apparent_magnitude: 4.01,
    distance: Distance {
        m: 104.3 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.51,
    mass: Some(Mass {
        kg: 1.22 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 6641. }),
    age: Some(Time {
        s: 1.337 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 10] = [
    ALPHA_PISCIUM,
    DELTA_PISCIUM,
    NU_PISCIUM,
    IOTA_PISCIUM,
    OMICRON_PISCIUM,
    EPSILON_PISCIUM,
    THETA_PISCIUM,
    ETA_PISCIUM,
    GAMMA_PISCIUM,
    OMEGA_PISCIUM,
];
