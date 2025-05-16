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

fn VEGA() -> RealData {
    RealData {
        common_name: "Vega",
        astronomical_name: "α Lyrae",
        constellation: "Lyra",
        radius: Some(Length {
            m: 2.362 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.135 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 0.58,
        apparent_magnitude: 0.03,
        temperature: Temperature { K: 9602. },
        age: Some(Time {
            s: 0.455 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.09929685 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(18, 36, 56.),
        declination: Declination::new(Sgn::Pos, 38, 47, 1.),
        distance: Length {
            m: 25. * LIGHT_YEAR.m,
        },
    }
}

fn R_LYRAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "R Lyrae",
        constellation: "Lyra",
        radius: None,
        mass: Mass {
            kg: 1.8 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -1.07,
        apparent_magnitude: 4.08,
        temperature: Temperature { K: 3313. },
        age: None,
        lifetime: Time {
            s: 1.46605285 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(18, 55, 20.),
        declination: Declination::new(Sgn::Pos, 43, 56, 46.),
        distance: Length {
            m: 349.4 * LIGHT_YEAR.m,
        },
    }
}

fn GAMMA_LYRAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Lyrae",
        constellation: "Lyra",
        right_ascension: RightAscension::new(18, 58, 57.),
        declination: Declination::new(Sgn::Pos, 32, 41, 22.),
        apparent_magnitude: 3.261,
        distance: Length {
            m: 620. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -3.140,
        mass: Mass {
            kg: 5.76 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 15.40 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 10_000. },
        age: None,
        lifetime: Time {
            s: 0.078916095 * BILLION_YEARS.s,
        },
    }
}

fn BETA_LYRAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Lyrae",
        constellation: "Lyra",
        right_ascension: RightAscension::new(18, 50, 5.),
        declination: Declination::new(Sgn::Pos, 33, 21, 46.),
        apparent_magnitude: 3.52,
        distance: Length {
            m: 960. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -3.82,
        mass: Mass {
            kg: 2.97 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 15.2 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 13_300. },
        age: Some(Time {
            s: 0.023 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.420724107 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 4] = [VEGA, R_LYRAE, GAMMA_LYRAE, BETA_LYRAE];
