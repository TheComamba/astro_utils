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

fn HAMAL() -> RealData {
    RealData {
        common_name: "Hamal",
        astronomical_name: "α Arietis",
        constellation: "Aries",
        radius: Some(Length {
            m: 14.9 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.5 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 0.48,
        apparent_magnitude: 2.01,
        temperature: Temperature { K: 4480. },
        right_ascension: RightAscension::new(2, 7, 10.),
        declination: Declination::new(Sgn::Pos, 23, 27, 45.),
        distance: Length {
            m: 66. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 2.5 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 2.54186931 * BILLION_YEARS.s,
        },
    }
}

fn BETA_ARIETIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Arietis",
        constellation: "Aries",
        right_ascension: RightAscension::new(1, 54, 38.),
        declination: Declination::new(Sgn::Pos, 20, 48, 29.),
        apparent_magnitude: 2.655,
        distance: Length {
            m: 59.6 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 1.55,
        mass: Mass {
            kg: 2.34 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 23. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 9000. },
        age: Some(Time {
            s: 0.3 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.916355612 * BILLION_YEARS.s,
        },
    }
}

fn BHARANI() -> RealData {
    RealData {
        common_name: "Bharani",
        astronomical_name: "41 Arietis",
        constellation: "Aries",
        right_ascension: RightAscension::new(2, 49, 59.),
        declination: Declination::new(Sgn::Pos, 27, 15, 38.),
        apparent_magnitude: 3.63,
        distance: Length {
            m: 166. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.163,
        mass: Mass {
            kg: 3.1 * SOLAR_MASS.kg,
        },
        radius: None,
        temperature: Temperature { K: 11_900. },
        age: Some(Time {
            s: 0.130 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.420724107 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [HAMAL, BETA_ARIETIS, BHARANI];
