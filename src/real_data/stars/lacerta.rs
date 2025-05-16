use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::{
    stars::real_data::RealData,
    units::{length::solar_radii, mass::solar_mass, time::gigayear},
};

fn ALPHA_LACERTAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Lacertae",
        constellation: "Lacerta",
        right_ascension: RightAscension::new(22, 31, 18.),
        declination: Declination::new(Sgn::Pos, 50, 16, 57.),
        apparent_magnitude: 3.76,
        distance: Length::new::<light_year>(102.6),
        absolute_magnitude: 1.27,
        mass: Mass::new::<solar_mass>(2.194),
        radius: Some(Length::new::<solar_radii>(2.1432)),
        temperature: ThermodynamicTemperature::new::<kelvin>(9050.),
        age: Some(Time::new::<gigayear>(0.4)),
        lifetime: Time::new::<gigayear>(1.03650581),
    }
}

fn BETA_LACERTAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Lacertae",
        constellation: "Lacerta",
        right_ascension: RightAscension::new(22, 23, 34.),
        declination: Declination::new(Sgn::Pos, 52, 13, 45.),
        apparent_magnitude: 4.43,
        distance: Length::new::<light_year>(170.),
        absolute_magnitude: 0.67,
        mass: Mass::new::<solar_mass>(0.97),
        radius: Some(Length::new::<solar_radii>(10.96)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4803.),
        age: Some(Time::new::<gigayear>(6.76)),
        lifetime: Time::new::<gigayear>(11.7800188),
    }
}

fn FIVE_LACERTAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "5 Lacertae",
        constellation: "Lacerta",
        right_ascension: RightAscension::new(22, 29, 32.),
        declination: Declination::new(Sgn::Pos, 47, 42, 25.),
        apparent_magnitude: 4.34,
        distance: Length::new::<light_year>(1164.),
        absolute_magnitude: -3.42,
        mass: Mass::new::<solar_mass>(5.11),
        radius: Some(Length::new::<solar_radii>(319.2)),
        temperature: ThermodynamicTemperature::new::<kelvin>(3713.),
        age: Some(Time::new::<gigayear>(0.1)),
        lifetime: Time::new::<gigayear>(0.10143918),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [ALPHA_LACERTAE(), BETA_LACERTAE(), FIVE_LACERTAE()]
}
