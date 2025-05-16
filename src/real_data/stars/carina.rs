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

fn CANOPUS() -> RealData {
    RealData {
        common_name: "Canopus",
        astronomical_name: "α Carinae",
        constellation: "Carina",
        radius: Some(Length {
            m: 72. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 9. * SOLAR_MASS.kg,
        },
        absolute_magnitude: -5.53,
        apparent_magnitude: -0.62,
        temperature: Temperature { K: 7400. },
        right_ascension: RightAscension::new(6, 23, 57.),
        declination: Declination::new(Sgn::Neg, 52, 41, 44.),
        distance: Length {
            m: 313. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.0251 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.03224554 * BILLION_YEARS.s,
        },
    }
}

fn MIAPLACIDUS() -> RealData {
    RealData {
        common_name: "Miaplacidus",
        astronomical_name: "β Carinae",
        constellation: "Carina",
        radius: Some(Length {
            m: 6.8 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 3.5 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -0.99,
        apparent_magnitude: 1.67,
        temperature: Temperature { K: 8866. },
        right_ascension: RightAscension::new(9, 13, 12.),
        declination: Declination::new(Sgn::Neg, 69, 43, 2.),
        distance: Length {
            m: 111. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.260 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.297402042 * BILLION_YEARS.s,
        },
    }
}

fn AVIOR() -> RealData {
    RealData {
        common_name: "Avior",
        astronomical_name: "ε Carinae",
        constellation: "Carina",
        radius: None,
        mass: Mass {
            kg: 10.5 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -4.58,
        apparent_magnitude: 1.86,
        temperature: Temperature { K: 3523. },
        right_ascension: RightAscension::new(8, 22, 31.),
        declination: Declination::new(Sgn::Neg, 59, 30, 34.),
        distance: Length {
            m: 632. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.026 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.026540021 * BILLION_YEARS.s,
        },
    }
}

fn ASPIDISKE() -> RealData {
    RealData {
        common_name: "Aspidiske",
        astronomical_name: "ι Carinae",
        constellation: "Carina",
        radius: Some(Length {
            m: 43. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 7.4 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -4.42,
        apparent_magnitude: 2.21,
        temperature: Temperature { K: 7500. },
        right_ascension: RightAscension::new(9, 17, 5.),
        declination: Declination::new(Sgn::Neg, 59, 16, 30.),
        distance: Length {
            m: 694. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.0374 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.052267043 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 4] = [CANOPUS, MIAPLACIDUS, AVIOR, ASPIDISKE];
