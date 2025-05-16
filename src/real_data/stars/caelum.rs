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

fn ALPHA_CAELI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Caeli",
        constellation: "Caelum",
        radius: Some(Length {
            m: 1.3 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.48 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 2.92,
        apparent_magnitude: 4.44,
        temperature: Temperature { K: 6991. },
        right_ascension: RightAscension::new(4, 40, 34.),
        declination: Declination::new(Sgn::Neg, 41, 51, 50.),
        distance: Length {
            m: 65.63 * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.9 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 2.54186931 * BILLION_YEARS.s,
        },
    }
}

fn GAMMA1_CAELI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ¹ Caeli",
        constellation: "Caelum",
        radius: Some(Length {
            m: 14.31 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.4 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 0.781,
        apparent_magnitude: 4.57,
        temperature: Temperature { K: 4411. },
        right_ascension: RightAscension::new(5, 4, 24.),
        declination: Declination::new(Sgn::Neg, 35, 28, 59.),
        distance: Length {
            m: 185. * LIGHT_YEAR.m,
        },
        age: None,
        lifetime: Time {
            s: 3.10253119 * BILLION_YEARS.s,
        },
    }
}

fn BETA_CAELI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Caeli",
        constellation: "Caelum",
        radius: Some(Length {
            m: 1.3 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.32 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 2.64,
        apparent_magnitude: 5.04,
        temperature: Temperature { K: 6763. },
        right_ascension: RightAscension::new(4, 42, 3.),
        declination: Declination::new(Sgn::Neg, 37, 8, 39.),
        distance: Length {
            m: 94. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 1.753 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 3.9126515 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [ALPHA_CAELI, GAMMA1_CAELI, BETA_CAELI];
