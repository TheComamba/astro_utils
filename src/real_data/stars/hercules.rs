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

fn RASALGETHI() -> RealData {
    RealData {
        common_name: "Rasalgethi",
        astronomical_name: "α Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(17, 14, 39.),
        declination: Declination::new(Sgn::Pos, 14, 23, 25.),
        apparent_magnitude: 2.78,
        distance: Length {
            m: 360. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -2.57,
        mass: Mass {
            kg: 2.5 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 284. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 3155. },
        age: None,
        lifetime: Time {
            s: 0.800458342 * BILLION_YEARS.s,
        },
    }
}

fn KORNEPHOROS() -> RealData {
    RealData {
        common_name: "Kornephoros",
        astronomical_name: "β Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(16, 30, 13.),
        declination: Declination::new(Sgn::Pos, 21, 29, 23.),
        apparent_magnitude: 2.81,
        distance: Length {
            m: 139. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -0.49,
        mass: Mass {
            kg: 2.9 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 17. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4887. },
        age: None,
        lifetime: Time {
            s: 0.513076303 * BILLION_YEARS.s,
        },
    }
}

fn SARIN() -> RealData {
    RealData {
        common_name: "Sarin",
        astronomical_name: "δ Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(17, 15, 2.),
        declination: Declination::new(Sgn::Pos, 24, 50, 21.),
        apparent_magnitude: 3.126,
        distance: Length {
            m: 75.1 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 1.31,
        mass: Mass {
            kg: 2.4 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 2.2 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 9620. },
        age: Some(Time {
            s: 0.370 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.800458342 * BILLION_YEARS.s,
        },
    }
}

fn ETA_HERCULIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "η Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(16, 42, 54.),
        declination: Declination::new(Sgn::Pos, 38, 55, 20.),
        apparent_magnitude: 3.487,
        distance: Length {
            m: 112. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.84,
        mass: Mass {
            kg: 2.13 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 8.9 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4900. },
        age: Some(Time {
            s: 1. * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.09929685 * BILLION_YEARS.s,
        },
    }
}

fn MU_HERCULIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "μ Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(17, 46, 28.),
        declination: Declination::new(Sgn::Pos, 27, 43, 14.),
        apparent_magnitude: 3.417,
        distance: Length {
            m: 27.11 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 3.82,
        mass: Mass {
            kg: 1.11 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 1.73 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 5560. },
        age: Some(Time {
            s: 6.8 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 6.97272616 * BILLION_YEARS.s,
        },
    }
}

fn ZETA_HERCULIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(16, 41, 17.),
        declination: Declination::new(Sgn::Pos, 31, 36, 10.),
        apparent_magnitude: 2.81,
        distance: Length {
            m: 35. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 2.65,
        mass: Mass {
            kg: 1.45 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 2.56 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 5820. },
        age: Some(Time {
            s: 2.7 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 2.82957282 * BILLION_YEARS.s,
        },
    }
}

fn PI_HERCULIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "π Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(17, 15, 3.),
        declination: Declination::new(Sgn::Pos, 36, 48, 33.),
        apparent_magnitude: 3.15,
        distance: Length {
            m: 377. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -2.1,
        mass: Mass {
            kg: 4. * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 72. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4170. },
        age: None,
        lifetime: Time {
            s: 0.193156929 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 7] = [
    RASALGETHI,
    KORNEPHOROS,
    SARIN,
    ETA_HERCULIS,
    MU_HERCULIS,
    ZETA_HERCULIS,
    PI_HERCULIS,
];
