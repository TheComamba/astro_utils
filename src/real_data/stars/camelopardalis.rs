use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn beta_camelopardalis() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Camelopardalis",
        constellation: "Camelopardalis",
        radius: Some(Length::new::<solar_radius>(58.)),
        mass: Mass::new::<solar_mass>(6.5),
        absolute_magnitude: -3.1,
        apparent_magnitude: 4.02,
        temperature: ThermodynamicTemperature::new::<kelvin>(5300.),
        right_ascension: RightAscension::new(5, 3, 25.),
        declination: Declination::new(Sgn::Pos, 60, 26, 32.),
        distance: Length::new::<light_year>(870.),
        age: Some(Time::new::<gigayear>(0.053)),
        lifetime: Time::new::<gigayear>(0.063411557),
    }
}

fn cs_camelopardalis() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "CS Camelopardalis",
        constellation: "Camelopardalis",
        radius: Some(Length::new::<solar_radius>(85.7)),
        mass: Mass::new::<solar_mass>(19.),
        absolute_magnitude: -6.39,
        apparent_magnitude: 4.21,
        temperature: ThermodynamicTemperature::new::<kelvin>(10_800.),
        right_ascension: RightAscension::new(3, 29, 4.),
        declination: Declination::new(Sgn::Pos, 59, 56, 25.),
        distance: Length::new::<light_year>(4289.),
        age: Some(Time::new::<gigayear>(0.011)),
        lifetime: Time::new::<gigayear>(0.011037517),
    }
}

fn alpha_camelopardalis() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Camelopardalis",
        constellation: "Camelopardalis",
        radius: Some(Length::new::<solar_radius>(32.5)),
        mass: Mass::new::<solar_mass>(37.6),
        absolute_magnitude: -7.1,
        apparent_magnitude: 4.29,
        temperature: ThermodynamicTemperature::new::<kelvin>(29_000.),
        right_ascension: RightAscension::new(4, 54, 3.),
        declination: Declination::new(Sgn::Pos, 66, 20, 34.),
        distance: Length::new::<light_year>(6_000.),
        age: Some(Time::new::<gigayear>(0.002)),
        lifetime: Time::new::<gigayear>(0.005279908),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [
        beta_camelopardalis(),
        cs_camelopardalis(),
        alpha_camelopardalis(),
    ]
}
