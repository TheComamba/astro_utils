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

fn alioth() -> RealData {
    RealData {
        common_name: "Alioth",
        astronomical_name: "ε Ursae Majoris",
        constellation: "Ursa Major",
        radius: Some(Length::new::<solar_radii>(4.14)),
        mass: Mass::new::<solar_mass>(2.91),
        absolute_magnitude: -0.21,
        apparent_magnitude: 1.76,
        temperature: ThermodynamicTemperature::new::<kelvin>(9_020.),
        age: Some(Time::new::<gigayear>(0.3)),
        right_ascension: RightAscension::new(12, 54, 2.),
        declination: Declination::new(Sgn::Pos, 55, 57, 36.),
        distance: Length::new::<light_year>(81.),
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

fn dubhe() -> RealData {
    RealData {
        common_name: "Dubhe",
        astronomical_name: "α Ursae Majoris",
        constellation: "Ursa Major",
        radius: Some(Length::new::<solar_radii>(17.03)),
        mass: Mass::new::<solar_mass>(3.44),
        absolute_magnitude: -1.08,
        apparent_magnitude: 1.81,
        temperature: ThermodynamicTemperature::new::<kelvin>(5012.),
        age: Some(Time::new::<gigayear>(0.28)),
        right_ascension: RightAscension::new(11, 3, 44.),
        declination: Declination::new(Sgn::Pos, 61, 45, 4.),
        distance: Length::new::<light_year>(124.),
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

fn alkaid() -> RealData {
    RealData {
        common_name: "Alkaid",
        astronomical_name: "η Ursae Majoris",
        constellation: "Ursa Major",
        radius: Some(Length::new::<solar_radii>(3.4)),
        mass: Mass::new::<solar_mass>(6.1),
        absolute_magnitude: -0.60,
        apparent_magnitude: 1.85,
        temperature: ThermodynamicTemperature::new::<kelvin>(15_540.),
        age: Some(Time::new::<gigayear>(0.01)),
        right_ascension: RightAscension::new(13, 47, 32.),
        declination: Declination::new(Sgn::Pos, 49, 18, 48.),
        distance: Length::new::<light_year>(101.),
        lifetime: Time::new::<gigayear>(0.073299383),
    }
}

fn mizar() -> RealData {
    RealData {
        common_name: "Mizar",
        astronomical_name: "ζ Ursae Majoris",
        constellation: "Ursa Major",
        radius: Some(Length::new::<solar_radii>(2.4)),
        mass: Mass::new::<solar_mass>(2.2),
        absolute_magnitude: 0.33,
        apparent_magnitude: 2.23,
        temperature: ThermodynamicTemperature::new::<kelvin>(9000.),
        age: Some(Time::new::<gigayear>(0.37)),
        right_ascension: RightAscension::new(13, 23, 56.),
        declination: Declination::new(Sgn::Pos, 54, 55, 31.),
        distance: Length::new::<light_year>(78.),
        lifetime: Time::new::<gigayear>(1.03650581),
    }
}

fn merak() -> RealData {
    RealData {
        common_name: "Merak",
        astronomical_name: "β Ursae Majoris",
        constellation: "Ursa Major",
        radius: Some(Length::new::<solar_radii>(3.021)),
        mass: Mass::new::<solar_mass>(2.7),
        absolute_magnitude: 0.41,
        apparent_magnitude: 2.34,
        temperature: ThermodynamicTemperature::new::<kelvin>(9377.),
        age: Some(Time::new::<gigayear>(0.5)),
        right_ascension: RightAscension::new(11, 1, 50.),
        declination: Declination::new(Sgn::Pos, 56, 22, 57.),
        distance: Length::new::<light_year>(79.),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn phecda() -> RealData {
    RealData {
        common_name: "Phecda",
        astronomical_name: "γ Ursae Majoris",
        constellation: "Ursa Major",
        radius: Some(Length::new::<solar_radii>(3.04)),
        mass: Mass::new::<solar_mass>(2.94),
        absolute_magnitude: 0.36,
        apparent_magnitude: 2.41,
        temperature: ThermodynamicTemperature::new::<kelvin>(9355.),
        age: Some(Time::new::<gigayear>(0.3)),
        right_ascension: RightAscension::new(11, 53, 50.),
        declination: Declination::new(Sgn::Pos, 53, 41, 41.),
        distance: Length::new::<light_year>(84.),
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

fn tania_australis() -> RealData {
    RealData {
        common_name: "Tania Australis",
        astronomical_name: "μ Ursae Majoris",
        constellation: "Ursa Major",
        radius: Some(Length::new::<solar_radii>(75.)),
        mass: Mass::new::<solar_mass>(6.3),
        absolute_magnitude: -1.2,
        apparent_magnitude: 3.06,
        temperature: ThermodynamicTemperature::new::<kelvin>(3899.),
        age: None,
        right_ascension: RightAscension::new(10, 22, 20.),
        declination: Declination::new(Sgn::Pos, 41, 29, 58.),
        distance: Length::new::<light_year>(230.0),
        lifetime: Time::new::<gigayear>(0.067960505),
    }
}

fn megrez() -> RealData {
    RealData {
        common_name: "Megrez",
        astronomical_name: "δ Ursae Majoris",
        constellation: "Ursa Major",
        radius: Some(Length::new::<solar_radii>(1.4)),
        mass: Mass::new::<solar_mass>(1.63),
        absolute_magnitude: 1.39,
        apparent_magnitude: 3.312,
        temperature: ThermodynamicTemperature::new::<kelvin>(9480.),
        age: Some(Time::new::<gigayear>(0.3)),
        right_ascension: RightAscension::new(12, 15, 26.),
        declination: Declination::new(Sgn::Pos, 57, 1, 57.),
        distance: Length::new::<light_year>(80.5),
        lifetime: Time::new::<gigayear>(1.89665739),
    }
}

pub(crate) fn stars() -> [RealData; 8] {
    [
        alioth(),
        dubhe(),
        alkaid(),
        mizar(),
        merak(),
        phecda(),
        tania_australis(),
        megrez(),
    ]
}
