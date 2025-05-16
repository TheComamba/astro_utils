use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::{
    stars::real_data::RealData,
    units::{mass::solar_mass, time::gigayear},
};

fn GAMMA2_NORMAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ² Normae",
        constellation: "Norma",
        right_ascension: RightAscension::new(16, 19, 50.),
        declination: Declination::new(Sgn::Neg, 50, 9, 20.),
        apparent_magnitude: 4.02,
        distance: Length::new::<light_year>(129.),
        absolute_magnitude: 1.057,
        mass: Mass::new::<solar_mass>(2.16),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(4699.),
        age: None,
        lifetime: Time::new::<gigayear>(1.09929685),
    }
}

fn EPSILON_NORMAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Normae",
        constellation: "Norma",
        right_ascension: RightAscension::new(16, 27, 11.),
        declination: Declination::new(Sgn::Neg, 47, 33, 17.),
        apparent_magnitude: 4.46,
        distance: Length::new::<light_year>(399.5),
        absolute_magnitude: -0.982,
        mass: Mass::new::<solar_mass>(6.4),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(10_888.),
        age: Some(Time::new::<gigayear>(0.0501)),
        lifetime: Time::new::<gigayear>(0.063411557),
    }
}

fn IOTA1_NORMAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ι¹ Normae",
        constellation: "Norma",
        right_ascension: RightAscension::new(16, 3, 32.),
        declination: Declination::new(Sgn::Neg, 57, 46, 30.),
        apparent_magnitude: 4.69,
        distance: Length::new::<light_year>(128.),
        absolute_magnitude: 1.46,
        mass: Mass::new::<solar_mass>(1.94),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(7842.),
        age: Some(Time::new::<gigayear>(0.731)),
        lifetime: Time::new::<gigayear>(1.46316038),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [GAMMA2_NORMAE(), EPSILON_NORMAE(), IOTA1_NORMAE()]
}
