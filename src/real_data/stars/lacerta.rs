use astro_coords::ra_and_dec::*;
use uom::si::{f64::{Length, Mass, ThermodynamicTemperature, Time}, length::light_year, thermodynamic_temperature::kelvin};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

fn ALPHA_LACERTAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Lacertae",
        constellation: "Lacerta",
        right_ascension: RightAscension::new(22, 31, 18.),
        declination: Declination::new(Sgn::Pos, 50, 16, 57.),
        apparent_magnitude: 3.76,
        distance: Length {
            m: 102.6 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 1.27,
        mass: Mass {
            kg: 2.194 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 2.1432 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 9050. },
        age: Some(Time {
            s: 0.4 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.03650581 * BILLION_YEARS.s,
        },
    }
}

fn BETA_LACERTAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Lacertae",
        constellation: "Lacerta",
        right_ascension: RightAscension::new(22, 23, 34.),
        declination: Declination::new(Sgn::Pos, 52, 13, 45.),
        apparent_magnitude: 4.43,
        distance: Length {
            m: 170. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.67,
        mass: Mass {
            kg: 0.97 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 10.96 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4803. },
        age: Some(Time {
            s: 6.76 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 11.7800188 * BILLION_YEARS.s,
        },
    }
}

fn FIVE_LACERTAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "5 Lacertae",
        constellation: "Lacerta",
        right_ascension: RightAscension::new(22, 29, 32.),
        declination: Declination::new(Sgn::Pos, 47, 42, 25.),
        apparent_magnitude: 4.34,
        distance: Length {
            m: 1164. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -3.42,
        mass: Mass {
            kg: 5.11 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 319.2 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 3713. },
        age: Some(Time {
            s: 0.1 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.10143918 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [ALPHA_LACERTAE, BETA_LACERTAE, FIVE_LACERTAE];
