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

fn ALDEBARAN() -> RealData {
    RealData {
        common_name: "Aldebaran",
        astronomical_name: "α Tauri",
        constellation: "Taurus",
        radius: Some(Length {
            m: 45.1 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.16 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -0.63,
        apparent_magnitude: 0.87,
        temperature: Temperature { K: 3900. },
        age: Some(Time {
            s: 5.5 * BILLION_YEARS.s,
        }),
        right_ascension: RightAscension::new(4, 35, 55.),
        declination: Declination::new(Sgn::Pos, 16, 30, 33.),
        distance: Length {
            m: 65. * LIGHT_YEAR.m,
        },
        lifetime: Time {
            s: 5.9461393 * BILLION_YEARS.s,
        },
    }
}

fn ALNATH() -> RealData {
    RealData {
        common_name: "Alnath",
        astronomical_name: "β Tauri",
        constellation: "Taurus",
        radius: Some(Length {
            m: 4.2 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 5.0 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -1.37,
        apparent_magnitude: 1.65,
        temperature: Temperature { K: 13_824. },
        age: Some(Time {
            s: 0.1 * BILLION_YEARS.s,
        }),
        right_ascension: RightAscension::new(5, 26, 18.),
        declination: Declination::new(Sgn::Pos, 28, 36, 27.),
        distance: Length {
            m: 131. * LIGHT_YEAR.m,
        },
        lifetime: Time {
            s: 0.111319448 * BILLION_YEARS.s,
        },
    }
}

fn GAMMA_TAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Tauri",
        constellation: "Taurus",
        right_ascension: RightAscension::new(4, 19, 48.),
        declination: Declination::new(Sgn::Pos, 15, 37, 40.),
        apparent_magnitude: 3.654,
        distance: Length {
            m: 154. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.22,
        mass: Mass {
            kg: 2.7 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 13.4 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4844. },
        age: Some(Time {
            s: 0.5 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.63513384 * BILLION_YEARS.s,
        },
    }
}

fn EPSILON_TAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Tauri",
        constellation: "Taurus",
        right_ascension: RightAscension::new(4, 28, 37.),
        declination: Declination::new(Sgn::Pos, 19, 10, 50.),
        apparent_magnitude: 3.53,
        distance: Length {
            m: 146. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.145,
        mass: Mass {
            kg: 2.57 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 12.35 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4950. },
        age: Some(Time {
            s: 0.625 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.63513384 * BILLION_YEARS.s,
        },
    }
}

fn LAMBDA_TAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "λ Tauri",
        constellation: "Taurus",
        right_ascension: RightAscension::new(4, 0, 41.),
        declination: Declination::new(Sgn::Pos, 12, 29, 25.),
        apparent_magnitude: 3.37,
        distance: Length {
            m: 480. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -2.45,
        mass: Mass {
            kg: 7.18 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 6.4 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 18_700. },
        age: Some(Time {
            s: 0.0332 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.052267043 * BILLION_YEARS.s,
        },
    }
}

fn ZETA_TAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Tauri",
        constellation: "Taurus",
        right_ascension: RightAscension::new(5, 37, 39.),
        declination: Declination::new(Sgn::Pos, 21, 8, 33.),
        apparent_magnitude: 3.010,
        distance: Length {
            m: 440. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -2.67,
        mass: Mass {
            kg: 11.2 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 5.5 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 15_500. },
        age: Some(Time {
            s: 0.019 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.019450199 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 6] = [
    ALDEBARAN,
    ALNATH,
    GAMMA_TAURI,
    EPSILON_TAURI,
    LAMBDA_TAURI,
    ZETA_TAURI,
];
