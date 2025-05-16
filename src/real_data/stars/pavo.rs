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

fn PEACOCK() -> RealData {
    RealData {
        common_name: "Peacock",
        astronomical_name: "α Pavonis",
        constellation: "Pavo",
        radius: Some(Length {
            m: 4.83 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 5.91 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -1.81,
        apparent_magnitude: 1.94,
        temperature: Temperature { K: 17_711. },
        age: Some(Time {
            s: 0.048 * BILLION_YEARS.s,
        }),
        right_ascension: RightAscension::new(20, 25, 39.),
        declination: Declination::new(Sgn::Neg, 56, 44, 6.),
        distance: Length {
            m: 183. * LIGHT_YEAR.m,
        },
        lifetime: Time {
            s: 0.073299383 * BILLION_YEARS.s,
        },
    }
}

fn BETA_PAVONIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Pavonis",
        constellation: "Pavo",
        right_ascension: RightAscension::new(20, 44, 57.),
        declination: Declination::new(Sgn::Neg, 66, 12, 12.),
        apparent_magnitude: 3.42,
        distance: Length {
            m: 135.1 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.33,
        mass: Mass {
            kg: 2.51 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 2.3 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 8184. },
        age: Some(Time {
            s: 0.305 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.63513384 * BILLION_YEARS.s,
        },
    }
}

fn DELTA_PAVONIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Pavonis",
        constellation: "Pavo",
        right_ascension: RightAscension::new(20, 8, 44.),
        declination: Declination::new(Sgn::Neg, 66, 10, 55.),
        apparent_magnitude: 3.56,
        distance: Length {
            m: 19.89 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 4.62,
        mass: Mass {
            kg: 1.051 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 1.197 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 5571. },
        age: Some(Time {
            s: 6.7 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 8.24015833 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [PEACOCK, BETA_PAVONIS, DELTA_PAVONIS];
