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

fn SUALOCIN() -> RealData {
    RealData {
        common_name: "Sualocin",
        astronomical_name: "α Delphini",
        constellation: "Delphinus",
        right_ascension: RightAscension::new(20, 39, 38.),
        declination: Declination::new(Sgn::Pos, 15, 54, 43.),
        apparent_magnitude: 3.777,
        distance: Length {
            m: 254. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -0.4,
        mass: Mass {
            kg: 3.83 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 3.92 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 11_643. },
        age: Some(Time {
            s: 0.22 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.220601963 * BILLION_YEARS.s,
        },
    }
}

fn ROTANEV() -> RealData {
    RealData {
        common_name: "Rotanev",
        astronomical_name: "β Delphini",
        constellation: "Delphinus",
        right_ascension: RightAscension::new(20, 37, 33.),
        declination: Declination::new(Sgn::Pos, 14, 35, 42.),
        apparent_magnitude: 3.64,
        distance: Length {
            m: 97.34 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 1.26,
        mass: Mass {
            kg: 1.75 * SOLAR_MASS.kg,
        },
        radius: None,
        temperature: Temperature { K: 6587. },
        age: Some(Time {
            s: 1.5 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.59501327 * BILLION_YEARS.s,
        },
    }
}

fn GAMMA_DELPHINI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Delphini",
        constellation: "Delphinus",
        right_ascension: RightAscension::new(20, 46, 39.),
        declination: Declination::new(Sgn::Pos, 16, 7, 27.),
        apparent_magnitude: 5.14,
        distance: Length {
            m: 114.8 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 2.24,
        mass: Mass {
            kg: 1.61 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 2.6 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 6295. },
        age: Some(Time {
            s: 1.85 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 2.08398753 * BILLION_YEARS.s,
        },
    }
}

fn DELTA_DELPHINI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Delphini",
        constellation: "Delphinus",
        right_ascension: RightAscension::new(20, 43, 28.),
        declination: Declination::new(Sgn::Pos, 15, 4, 28.),
        apparent_magnitude: 4.43,
        distance: Length {
            m: 223. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.25,
        mass: Mass {
            kg: 1.78 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 3.43 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 7440. },
        age: Some(Time {
            s: 0.945 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.46605285 * BILLION_YEARS.s,
        },
    }
}

fn ALDULFIN() -> RealData {
    RealData {
        common_name: "Aldulfin",
        astronomical_name: "ε Delphini",
        constellation: "Delphinus",
        right_ascension: RightAscension::new(20, 33, 13.),
        declination: Declination::new(Sgn::Pos, 11, 18, 12.),
        apparent_magnitude: 4.03,
        distance: Length {
            m: 358.6 * LIGHT_YEAR.m,
        },
        absolute_magnitude: -1.18,
        mass: Mass {
            kg: 6.4 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 4.6 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 13_614. },
        age: Some(Time {
            s: 0.06 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.063411557 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 5] =
    [SUALOCIN, ROTANEV, GAMMA_DELPHINI, DELTA_DELPHINI, ALDULFIN];
