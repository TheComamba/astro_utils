use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn alpha_reticuli() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Reticuli",
        constellation: "Reticulum",
        right_ascension: RightAscension::new(4, 14, 25.),
        declination: Declination::new(Sgn::Neg, 62, 28, 26.),
        apparent_magnitude: 3.315,
        distance: Length::new::<light_year>(161.6),
        absolute_magnitude: -0.17,
        mass: Mass::new::<solar_mass>(3.11),
        radius: Some(Length::new::<solar_radius>(12.8)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5196.),
        age: Some(Time::new::<gigayear>(0.33)),
        lifetime: Time::new::<gigayear>(0.351318702),
    }
}

fn beta_reticuli() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Reticuli",
        constellation: "Reticulum",
        right_ascension: RightAscension::new(3, 44, 12.),
        declination: Declination::new(Sgn::Neg, 64, 48, 25.),
        apparent_magnitude: 3.84,
        distance: Length::new::<light_year>(97.),
        absolute_magnitude: 1.46,
        mass: Mass::new::<solar_mass>(1.2),
        radius: Some(Length::new::<solar_radius>(9.3)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4580.),
        age: Some(Time::new::<gigayear>(5.)),
        lifetime: Time::new::<gigayear>(5.06543331),
    }
}

fn epsilon_reticuli() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Reticuli",
        constellation: "Reticulum",
        right_ascension: RightAscension::new(4, 16, 29.),
        declination: Declination::new(Sgn::Neg, 59, 18, 8.),
        apparent_magnitude: 4.44,
        distance: Length::new::<light_year>(60.1),
        absolute_magnitude: 3.1,
        mass: Mass::new::<solar_mass>(1.46),
        radius: Some(Length::new::<solar_radius>(3.18)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4961.),
        age: Some(Time::new::<gigayear>(2.8)),
        lifetime: Time::new::<gigayear>(2.82957282),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [alpha_reticuli(), beta_reticuli(), epsilon_reticuli()]
}
