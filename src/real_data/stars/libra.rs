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

fn ZUBENELGENUBI() -> RealData {
    RealData {
        common_name: "Zubenelgenubi",
        astronomical_name: "α² Librae",
        constellation: "Libra",
        right_ascension: RightAscension::new(14, 50, 53.),
        declination: Declination::new(Sgn::Neg, 16, 2, 30.),
        apparent_magnitude: 2.741,
        distance: Length {
            m: 75.8 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.879,
        mass: Mass {
            kg: 1.95 * SOLAR_MASS.kg,
        },
        radius: None,
        temperature: Temperature { K: 8128. },
        age: None,
        lifetime: Time {
            s: 1.46316038 * BILLION_YEARS.s,
        },
    }
}

fn ZUBENESCHAMALI() -> RealData {
    RealData {
        common_name: "Zubeneschamali",
        astronomical_name: "β Librae",
        constellation: "Libra",
        right_ascension: RightAscension::new(15, 17, 0.),
        declination: Declination::new(Sgn::Neg, 9, 22, 58.),
        apparent_magnitude: 2.61,
        distance: Length {
            m: 185. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -1.16,
        mass: Mass {
            kg: 3.5 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 4.9 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 12_300. },
        age: Some(Time {
            s: 0.08 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.297402042 * BILLION_YEARS.s,
        },
    }
}

fn BRACHIUM() -> RealData {
    RealData {
        common_name: "Brachium",
        astronomical_name: "σ Librae",
        constellation: "Libra",
        radius: Some(Length {
            m: 108. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.2 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -1.5,
        apparent_magnitude: 3.21,
        temperature: Temperature { K: 3596. },
        age: None,
        lifetime: Time {
            s: 1.03650581 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(15, 4, 4.),
        declination: Declination::new(Sgn::Neg, 25, 16, 55.),
        distance: Length {
            m: 288. * LIGHT_YEAR.m,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [ZUBENELGENUBI, ZUBENESCHAMALI, BRACHIUM];
