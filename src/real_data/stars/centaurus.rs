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

fn RIGEL_KENTAURUS() -> RealData {
    RealData {
        common_name: "Rigel Kentaurus",
        astronomical_name: "α Centauri",
        constellation: "Centaurus",
        radius: Some(Length {
            m: 1.2175 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.0788 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 4.34,
        apparent_magnitude: -0.27,
        temperature: Temperature { K: 5790. },
        right_ascension: RightAscension::new(14, 39, 36.),
        declination: Declination::new(Sgn::Neg, 60, 50, 2.),
        distance: Length {
            m: 4. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 4.85 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 6.97272616 * BILLION_YEARS.s,
        },
    }
}

fn HADAR() -> RealData {
    RealData {
        common_name: "Hadar",
        astronomical_name: "β Centauri",
        constellation: "Centaurus",
        radius: Some(Length {
            m: 9. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 12.02 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -5.42,
        apparent_magnitude: 0.61,
        temperature: Temperature { K: 25_000. },
        right_ascension: RightAscension::new(14, 3, 49.),
        declination: Declination::new(Sgn::Neg, 60, 22, 23.),
        distance: Length {
            m: 526. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.0141 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.019450199 * BILLION_YEARS.s,
        },
    }
}

fn MENKENT() -> RealData {
    RealData {
        common_name: "Menkent",
        astronomical_name: "θ Centauri",
        constellation: "Centaurus",
        radius: Some(Length {
            m: 10.6 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.27 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 0.70,
        apparent_magnitude: 2.06,
        temperature: Temperature { K: 4980. },
        right_ascension: RightAscension::new(14, 6, 41.),
        declination: Declination::new(Sgn::Neg, 36, 22, 11.),
        distance: Length {
            m: 61. * LIGHT_YEAR.m,
        },
        age: None,
        lifetime: Time {
            s: 4.45521207 * BILLION_YEARS.s,
        },
    }
}

fn MUHLIFAIN() -> RealData {
    RealData {
        common_name: "Muhlifain",
        astronomical_name: "γ Centauri",
        constellation: "Centaurus",
        radius: None,
        mass: Mass {
            kg: 2.91 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -0.81,
        apparent_magnitude: 2.20,
        temperature: Temperature { K: 9082. },
        right_ascension: RightAscension::new(12, 41, 31.),
        declination: Declination::new(Sgn::Neg, 48, 57, 35.),
        distance: Length {
            m: 130. * LIGHT_YEAR.m,
        },
        age: None,
        lifetime: Time {
            s: 0.420724107 * BILLION_YEARS.s,
        },
    }
}

fn EPSILON_CENTAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Centauri",
        constellation: "Centaurus",
        radius: None,
        mass: Mass {
            kg: 11.6 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -3.02,
        apparent_magnitude: 2.29,
        temperature: Temperature { K: 24_000. },
        right_ascension: RightAscension::new(13, 39, 53.),
        declination: Declination::new(Sgn::Neg, 53, 27, 59.),
        distance: Length {
            m: 376. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.0158 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.019450199 * BILLION_YEARS.s,
        },
    }
}

fn ETA_CENTAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "η Centauri",
        constellation: "Centaurus",
        radius: Some(Length {
            m: 6.1 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 12.0 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -2.55,
        apparent_magnitude: 2.29,
        temperature: Temperature { K: 25_700. },
        right_ascension: RightAscension::new(14, 35, 30.),
        declination: Declination::new(Sgn::Neg, 42, 9, 28.),
        distance: Length {
            m: 308. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.0056 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.019450199 * BILLION_YEARS.s,
        },
    }
}

fn ZETA_CENTAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Centauri",
        constellation: "Centaurus",
        radius: Some(Length {
            m: 5.8 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 7.8 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -2.81,
        apparent_magnitude: 2.55,
        temperature: Temperature { K: 23_561. },
        right_ascension: RightAscension::new(13, 55, 33.),
        declination: Declination::new(Sgn::Neg, 47, 17, 18.),
        distance: Length {
            m: 384. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.04 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.040555762 * BILLION_YEARS.s,
        },
    }
}

fn MA_WEI() -> RealData {
    RealData {
        common_name: "Ma Wei",
        astronomical_name: "δ Centauri",
        constellation: "Centaurus",
        radius: Some(Length {
            m: 6.5 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 8.7 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -2.84,
        apparent_magnitude: 2.58,
        temperature: Temperature { K: 22_360. },
        right_ascension: RightAscension::new(12, 8, 21.),
        declination: Declination::new(Sgn::Neg, 50, 43, 21.),
        distance: Length {
            m: 395. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.02 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.03224554 * BILLION_YEARS.s,
        },
    }
}

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
