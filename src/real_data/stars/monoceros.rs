use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

fn ALPHA_MONOCEROTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Monocerotis",
        constellation: "Monoceros",
        right_ascension: RightAscension::new(7, 41, 15.),
        declination: Declination::new(Sgn::Neg, 9, 33, 4.),
        apparent_magnitude: 3.94,
        distance: Length::new::<light_year>(148.),
        absolute_magnitude: 0.71,
        mass: Mass::new::<solar_mass>(2.02),
        radius: Some(Length::new::<solar_radii>(10.1)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4879.),
        age: Some(Time::new::<gigayear>(1.18)),
        lifetime: Time::new::<gigayear>(1.36020165),
    }
}

fn GAMMA_MONOCEROTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Monocerotis",
        constellation: "Monoceros",
        right_ascension: RightAscension::new(6, 14, 51.),
        declination: Declination::new(Sgn::Neg, 6, 16, 29.),
        apparent_magnitude: 3.96,
        distance: Length::new::<light_year>(500.),
        absolute_magnitude: -1.93,
        mass: Mass::new::<solar_mass>(1.2),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(4375.),
        age: None,
        lifetime: Time::new::<gigayear>(5.06543331),
    }
}

fn DELTA_MONOCEROTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Monocerotis",
        constellation: "Monoceros",
        right_ascension: RightAscension::new(7, 11, 52.),
        declination: Declination::new(Sgn::Neg, 0, 29, 34.),
        apparent_magnitude: 4.15,
        distance: Length::new::<light_year>(384.),
        absolute_magnitude: -1.20,
        mass: Mass::new::<solar_mass>(2.4),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(9462.),
        age: Some(Time::new::<gigayear>(0.405)),
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

pub(crate) const STARS: [RealData; 3] = [ALPHA_MONOCEROTIS, GAMMA_MONOCEROTIS, DELTA_MONOCEROTIS];
