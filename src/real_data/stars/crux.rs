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

fn ACRUX() -> RealData {
    RealData {
        common_name: "Acrux",
        astronomical_name: "α Crucis",
        constellation: "Crux",
        radius: Some(Length::new::<solar_radii>(7.8)),
        mass: Mass::new::<solar_mass>(17.8),
        absolute_magnitude: -4.19,
        apparent_magnitude: 0.77,
        temperature: Temperature { K: 24_000. },
        age: Some(Time::new::<gigayear>(0.0108)),
        lifetime: Time::new::<gigayear>(0.011037517),
        right_ascension: RightAscension::new(12, 26, 36.),
        declination: Declination::new(Sgn::Neg, 63, 5, 57.),
        distance: Length::new::<light_year>(321.),
    }
}

fn MIMOSA() -> RealData {
    RealData {
        common_name: "Mimosa",
        astronomical_name: "β Crucis",
        constellation: "Crux",
        radius: Some(Length::new::<solar_radii>(8.4)),
        mass: Mass::new::<solar_mass>(16.),
        absolute_magnitude: -3.92,
        apparent_magnitude: 1.25,
        temperature: Temperature { K: 27_000. },
        age: Some(Time::new::<gigayear>(0.010)),
        right_ascension: RightAscension::new(12, 47, 43.),
        declination: Declination::new(Sgn::Neg, 59, 41, 20.),
        distance: Length::new::<light_year>(352.),
        lifetime: Time::new::<gigayear>(0.012799766),
    }
}

fn GACRUX() -> RealData {
    RealData {
        common_name: "Gacrux",
        astronomical_name: "γ Crucis",
        constellation: "Crux",
        radius: Some(Length::new::<solar_radii>(120.)),
        mass: Mass::new::<solar_mass>(1.5),
        absolute_magnitude: -0.56,
        apparent_magnitude: 1.59,
        temperature: ThermodynamicTemperature::new::<kelvin>(3689.),
        age: None,
        right_ascension: RightAscension::new(12, 31, 10.),
        declination: Declination::new(Sgn::Neg, 57, 6, 48.),
        distance: Length::new::<light_year>(88.),
        lifetime: Time::new::<gigayear>(2.54186931),
    }
}

pub(crate) const STARS: [RealData; 3] = [ACRUX, MIMOSA, GACRUX];
