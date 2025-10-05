use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn acrux() -> RealData {
    RealData {
        common_name: "Acrux",
        astronomical_name: "α Crucis",
        constellation: "Crux",
        radius: Some(Length::new::<solar_radius>(7.8)),
        mass: Mass::new::<solar_mass>(17.8),
        absolute_magnitude: -4.19,
        apparent_magnitude: 0.77,
        temperature: ThermodynamicTemperature::new::<kelvin>(24_000.),
        age: Some(Time::new::<gigayear>(0.0108)),
        lifetime: Time::new::<gigayear>(0.011037517),
        right_ascension: RightAscension::new(12, 26, 36.),
        declination: Declination::new(Sgn::Neg, 63, 5, 57.),
        distance: Length::new::<light_year>(321.),
    }
}

fn mimosa() -> RealData {
    RealData {
        common_name: "Mimosa",
        astronomical_name: "β Crucis",
        constellation: "Crux",
        radius: Some(Length::new::<solar_radius>(8.4)),
        mass: Mass::new::<solar_mass>(16.),
        absolute_magnitude: -3.92,
        apparent_magnitude: 1.25,
        temperature: ThermodynamicTemperature::new::<kelvin>(27_000.),
        age: Some(Time::new::<gigayear>(0.010)),
        right_ascension: RightAscension::new(12, 47, 43.),
        declination: Declination::new(Sgn::Neg, 59, 41, 20.),
        distance: Length::new::<light_year>(352.),
        lifetime: Time::new::<gigayear>(0.012799766),
    }
}

fn gacrux() -> RealData {
    RealData {
        common_name: "Gacrux",
        astronomical_name: "γ Crucis",
        constellation: "Crux",
        radius: Some(Length::new::<solar_radius>(120.)),
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

pub(crate) fn stars() -> [RealData; 3] {
    [acrux(), mimosa(), gacrux()]
}
