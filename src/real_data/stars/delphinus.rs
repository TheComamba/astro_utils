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

fn sualocin() -> RealData {
    RealData {
        common_name: "Sualocin",
        astronomical_name: "α Delphini",
        constellation: "Delphinus",
        right_ascension: RightAscension::new(20, 39, 38.),
        declination: Declination::new(Sgn::Pos, 15, 54, 43.),
        apparent_magnitude: 3.777,
        distance: Length::new::<light_year>(254.),
        absolute_magnitude: -0.4,
        mass: Mass::new::<solar_mass>(3.83),
        radius: Some(Length::new::<solar_radii>(3.92)),
        temperature: ThermodynamicTemperature::new::<kelvin>(11_643.),
        age: Some(Time::new::<gigayear>(0.22)),
        lifetime: Time::new::<gigayear>(0.220601963),
    }
}

fn rotanev() -> RealData {
    RealData {
        common_name: "Rotanev",
        astronomical_name: "β Delphini",
        constellation: "Delphinus",
        right_ascension: RightAscension::new(20, 37, 33.),
        declination: Declination::new(Sgn::Pos, 14, 35, 42.),
        apparent_magnitude: 3.64,
        distance: Length::new::<light_year>(97.34),
        absolute_magnitude: 1.26,
        mass: Mass::new::<solar_mass>(1.75),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(6587.),
        age: Some(Time::new::<gigayear>(1.5)),
        lifetime: Time::new::<gigayear>(1.59501327),
    }
}

fn gamma_delphini() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Delphini",
        constellation: "Delphinus",
        right_ascension: RightAscension::new(20, 46, 39.),
        declination: Declination::new(Sgn::Pos, 16, 7, 27.),
        apparent_magnitude: 5.14,
        distance: Length::new::<light_year>(114.8),
        absolute_magnitude: 2.24,
        mass: Mass::new::<solar_mass>(1.61),
        radius: Some(Length::new::<solar_radii>(2.6)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6295.),
        age: Some(Time::new::<gigayear>(1.85)),
        lifetime: Time::new::<gigayear>(2.08398753),
    }
}

fn delta_delphini() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Delphini",
        constellation: "Delphinus",
        right_ascension: RightAscension::new(20, 43, 28.),
        declination: Declination::new(Sgn::Pos, 15, 4, 28.),
        apparent_magnitude: 4.43,
        distance: Length::new::<light_year>(223.),
        absolute_magnitude: 0.25,
        mass: Mass::new::<solar_mass>(1.78),
        radius: Some(Length::new::<solar_radii>(3.43)),
        temperature: ThermodynamicTemperature::new::<kelvin>(7440.),
        age: Some(Time::new::<gigayear>(0.945)),
        lifetime: Time::new::<gigayear>(1.46605285),
    }
}

fn aldulfin() -> RealData {
    RealData {
        common_name: "Aldulfin",
        astronomical_name: "ε Delphini",
        constellation: "Delphinus",
        right_ascension: RightAscension::new(20, 33, 13.),
        declination: Declination::new(Sgn::Pos, 11, 18, 12.),
        apparent_magnitude: 4.03,
        distance: Length::new::<light_year>(358.6),
        absolute_magnitude: -1.18,
        mass: Mass::new::<solar_mass>(6.4),
        radius: Some(Length::new::<solar_radii>(4.6)),
        temperature: ThermodynamicTemperature::new::<kelvin>(13_614.),
        age: Some(Time::new::<gigayear>(0.06)),
        lifetime: Time::new::<gigayear>(0.063411557),
    }
}

pub(crate) fn stars() -> [RealData; 5] {
    [
        sualocin(),
        rotanev(),
        gamma_delphini(),
        delta_delphini(),
        aldulfin(),
    ]
}
