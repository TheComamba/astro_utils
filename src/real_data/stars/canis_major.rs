use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{solar_radii },
        mass::{solar_mass, },
        time::{gigayear, },
    },
};

fn SIRIUS() -> RealData {
    RealData {
        common_name: "Sirius",
        astronomical_name: "α Canis Majoris",
        constellation: "Canis Major",
        radius: Some(Length::new::<solar_radii>(1.711)),
        mass: Mass::new::<solar_mass>(2.063),
        absolute_magnitude: 1.45,
        apparent_magnitude: -1.44,
        temperature: ThermodynamicTemperature::new::<kelvin>(9940.),
        right_ascension: RightAscension::new(6, 45, 9.),
        declination: Declination::new(Sgn::Neg, 16, 42, 58.),
        distance: Length::new::<light_year>(9.),
        age: Some(Time::new::<gigayear>(0.242)),
        lifetime: Time::new::<gigayear>(1.25731981),
    }
}

fn ADHARA() -> RealData {
    RealData {
        common_name: "Adhara",
        astronomical_name: "ε Canis Majoris",
        constellation: "Canis Major",
        radius: Some(Length::new::<solar_radii>(13.9)),
        mass: Mass::new::<solar_mass>(12.6),
        absolute_magnitude: -4.10,
        apparent_magnitude: 1.5,
        temperature: ThermodynamicTemperature::new::<kelvin>(22_900.),
        right_ascension: RightAscension::new(6, 58, 38.),
        declination: Declination::new(Sgn::Neg, 28, 58, 19.),
        distance: Length::new::<light_year>(431.),
        age: Some(Time::new::<gigayear>(0.019)),
        lifetime: Time::new::<gigayear>(0.019450199),
    }
}

fn WEZEN() -> RealData {
    RealData {
        common_name: "Wezen",
        astronomical_name: "δ Canis Majoris",
        constellation: "Canis Major",
        radius: Some(Length::new::<solar_radii>(215.)),
        mass: Mass::new::<solar_mass>(16.9),
        absolute_magnitude: -6.87,
        apparent_magnitude: 1.83,
        temperature: ThermodynamicTemperature::new::<kelvin>(6390.),
        right_ascension: RightAscension::new(7, 8, 23.),
        declination: Declination::new(Sgn::Neg, 26, 23, 36.),
        distance: Length::new::<light_year>(1791.),
        age: Some(Time::new::<gigayear>(0.012)),
        lifetime: Time::new::<gigayear>(0.012799766),
    }
}

fn MIRZAM() -> RealData {
    RealData {
        common_name: "Mirzam",
        astronomical_name: "β Canis Majoris",
        constellation: "Canis Major",
        radius: Some(Length::new::<solar_radii>(9.7)),
        mass: Mass::new::<solar_mass>(13.5),
        absolute_magnitude: -3.95,
        apparent_magnitude: 1.98,
        temperature: ThermodynamicTemperature::new::<kelvin>(25_000.),
        right_ascension: RightAscension::new(6, 22, 42.),
        declination: Declination::new(Sgn::Neg, 17, 57, 21.),
        distance: Length::new::<light_year>(499.),
        age: Some(Time::new::<gigayear>(0.0124)),
        lifetime: Time::new::<gigayear>(0.015362858),
    }
}

fn ALUDRA() -> RealData {
    RealData {
        common_name: "Aludra",
        astronomical_name: "η Canis Majoris",
        constellation: "Canis Major",
        radius: Some(Length::new::<solar_radii>(54.)),
        mass: Mass::new::<solar_mass>(18.19),
        absolute_magnitude: -7.51,
        apparent_magnitude: 2.45,
        temperature: ThermodynamicTemperature::new::<kelvin>(15_500.),
        right_ascension: RightAscension::new(7, 24, 6.),
        declination: Declination::new(Sgn::Neg, 29, 18, 11.),
        distance: Length::new::<light_year>(3196.),
        age: Some(Time::new::<gigayear>(0.0083)),
        lifetime: Time::new::<gigayear>(0.011037517),
    }
}

pub(crate) fn STARS() -> [RealData; 5] { [SIRIUS(), ADHARA(), WEZEN(), MIRZAM(), ALUDRA()] }
