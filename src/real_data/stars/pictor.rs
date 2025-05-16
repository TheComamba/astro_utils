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

fn ALPHA_PICTORIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Pictoris",
        constellation: "Pictor",
        right_ascension: RightAscension::new(6, 48, 11.),
        declination: Declination::new(Sgn::Neg, 61, 56, 29.),
        apparent_magnitude: 3.27,
        distance: Length {
            m: 97. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.86,
        mass: Mass {
            kg: 2.04 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 1.6 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 7530. },
        age: Some(Time {
            s: 0.660 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.25731981 * BILLION_YEARS.s,
        },
    }
}

fn BETA_PICTORIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Pictoris",
        constellation: "Pictor",
        right_ascension: RightAscension::new(5, 47, 17.),
        declination: Declination::new(Sgn::Neg, 51, 3, 59.),
        apparent_magnitude: 3.861,
        distance: Length {
            m: 63.4 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 2.402,
        mass: Mass {
            kg: 1.75 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 1.8 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 8052. },
        age: Some(Time {
            s: 0.023 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.59501327 * BILLION_YEARS.s,
        },
    }
}

fn GAMMA_PICTORIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Pictoris",
        constellation: "Pictor",
        right_ascension: RightAscension::new(5, 49, 50.),
        declination: Declination::new(Sgn::Neg, 56, 9, 60.),
        apparent_magnitude: 4.50,
        distance: Length {
            m: 177. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.83,
        mass: Mass {
            kg: 1.59 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 11. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4600. },
        age: None,
        lifetime: Time {
            s: 2.08398753 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [ALPHA_PICTORIS, BETA_PICTORIS, GAMMA_PICTORIS];
