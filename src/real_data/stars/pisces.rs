use astro_coords::ra_and_dec::*;
use simple_si_units::base::{Length, Mass, Temperature, Time};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

const ALPHA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "α Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(2, 2, 3.),
    declination: Declination::new(Sgn::Pos, 2, 45, 50.),
    apparent_magnitude: 3.82,
    distance: Length {
        m: 151. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.5,
    mass: Mass {
        kg: 2.55 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 2.45 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 10_233. },
    age: None,
    lifetime: Time {
        s: 0.63513384 * BILLION_YEARS.s,
    },
};

const DELTA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "δ Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(0, 48, 41.),
    declination: Declination::new(Sgn::Pos, 7, 35, 6.),
    apparent_magnitude: 4.416,
    distance: Length {
        m: 311. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.46,
    mass: Mass {
        kg: 1.65 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 44. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 3963. },
    age: Some(Time {
        s: 0.00298 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.89665739 * BILLION_YEARS.s,
    },
};

const NU_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "ν Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(1, 41, 26.),
    declination: Declination::new(Sgn::Pos, 5, 29, 15.),
    apparent_magnitude: 4.44,
    distance: Length {
        m: 363. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.78,
    mass: Mass {
        kg: 1.66 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 34. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4154. },
    age: Some(Time {
        s: 1.8 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.89665739 * BILLION_YEARS.s,
    },
};

const IOTA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "ι Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(23, 39, 57.),
    declination: Declination::new(Sgn::Pos, 5, 37, 35.),
    apparent_magnitude: 4.13,
    distance: Length {
        m: 44.73 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 3.43,
    mass: Mass {
        kg: 1.3 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 1.595 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 6288. },
    age: Some(Time {
        s: 3.8 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 3.9126515 * BILLION_YEARS.s,
    },
};

const OMICRON_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "ο Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(1, 45, 24.),
    declination: Declination::new(Sgn::Pos, 9, 9, 28.),
    apparent_magnitude: 4.27,
    distance: Length {
        m: 280. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.22,
    mass: Mass {
        kg: 3.03 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 14.57 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5004. },
    age: Some(Time {
        s: 0.390 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.420724107 * BILLION_YEARS.s,
    },
};

const EPSILON_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "ε Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(1, 2, 57.),
    declination: Declination::new(Sgn::Pos, 7, 53, 24.),
    apparent_magnitude: 4.27,
    distance: Length {
        m: 182. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.44,
    mass: Mass {
        kg: 2.27 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 10.9 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4814. },
    age: Some(Time {
        s: 0.9 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.964406929 * BILLION_YEARS.s,
    },
};

const THETA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "θ Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(23, 27, 58.),
    declination: Declination::new(Sgn::Pos, 6, 22, 44.),
    apparent_magnitude: 4.27,
    distance: Length {
        m: 149. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.83,
    mass: Mass {
        kg: 1.58 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 11. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4684. },
    age: Some(Time {
        s: 0.00245 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 2.08398753 * BILLION_YEARS.s,
    },
};

const ETA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "η Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(1, 31, 29.),
    declination: Declination::new(Sgn::Pos, 15, 20, 45.),
    apparent_magnitude: 3.611,
    distance: Length {
        m: 350. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.52,
    mass: Mass {
        kg: 3.78 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 26.48 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4937. },
    age: Some(Time {
        s: 0.220 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.220601963 * BILLION_YEARS.s,
    },
};

const GAMMA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(23, 17, 10.),
    declination: Declination::new(Sgn::Pos, 3, 16, 56.),
    apparent_magnitude: 3.699,
    distance: Length {
        m: 135. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.68,
    mass: Mass {
        kg: 0.97 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 11.28 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4833. },
    age: Some(Time {
        s: 4.58 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 11.7800188 * BILLION_YEARS.s,
    },
};

const OMEGA_PISCIUM: RealData = RealData {
    common_name: "",
    astronomical_name: "ω Piscium",
    constellation: "Pisces",
    right_ascension: RightAscension::new(23, 59, 19.),
    declination: Declination::new(Sgn::Pos, 6, 51, 48.),
    apparent_magnitude: 4.01,
    distance: Length {
        m: 104.3 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.51,
    mass: Mass {
        kg: 1.22 * SOLAR_MASS.kg,
    },
    radius: None,
    temperature: Temperature { K: 6641. },
    age: Some(Time {
        s: 1.337 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 5.06543331 * BILLION_YEARS.s,
    },
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
