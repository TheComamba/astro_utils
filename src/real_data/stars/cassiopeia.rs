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

fn NAVI() -> RealData {
    RealData {
        common_name: "Navi",
        astronomical_name: "γ Cassiopeiae",
        constellation: "Cassiopeia",
        radius: Some(Length {
            m: 10. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 13. * SOLAR_MASS.kg,
        },
        absolute_magnitude: -4.22,
        apparent_magnitude: 2.20,
        temperature: Temperature { K: 25_000. },
        right_ascension: RightAscension::new(0, 56, 43.),
        declination: Declination::new(Sgn::Pos, 60, 43, 0.),
        distance: Length {
            m: 613. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.008 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.019450199 * BILLION_YEARS.s,
        },
    }
}

fn SCHEDAR() -> RealData {
    RealData {
        common_name: "Schedar",
        astronomical_name: "α Cassiopeiae",
        constellation: "Cassiopeia",
        radius: Some(Length {
            m: 45.39 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 3.98 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -1.99,
        apparent_magnitude: 2.24,
        temperature: Temperature { K: 4552. },
        right_ascension: RightAscension::new(0, 40, 30.),
        declination: Declination::new(Sgn::Pos, 56, 32, 14.),
        distance: Length {
            m: 228. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.19 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.193156929 * BILLION_YEARS.s,
        },
    }
}

fn CAPH() -> RealData {
    RealData {
        common_name: "Caph",
        astronomical_name: "β Cassiopeiae",
        constellation: "Cassiopeia",
        radius: Some(Length {
            m: 3.5 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.91 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 1.17,
        apparent_magnitude: 2.28,
        temperature: Temperature { K: 7079. },
        right_ascension: RightAscension::new(0, 9, 11.),
        declination: Declination::new(Sgn::Pos, 59, 8, 59.),
        distance: Length {
            m: 54. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 1.1 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.54706939 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [NAVI, SCHEDAR, CAPH];
