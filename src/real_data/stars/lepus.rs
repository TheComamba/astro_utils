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

fn ARNEB() -> RealData {
    RealData {
        common_name: "Arneb",
        astronomical_name: "α Leporis",
        constellation: "Lepus",
        radius: Some(Length::new::<solar_radii>(75.)),
        mass: Mass::new::<solar_mass>(13.9),
        absolute_magnitude: -5.40,
        apparent_magnitude: 2.58,
        temperature: ThermodynamicTemperature::new::<kelvin>(6_850.),
        age: Some(Time::new::<gigayear>(0.013)),
        lifetime: Time::new::<gigayear>(0.015362858),
        right_ascension: RightAscension::new(5, 32, 44.),
        declination: Declination::new(Sgn::Neg, 17, 49, 20.),
        distance: Length::new::<light_year>(1283.),
    }
}

fn BETA_LEPORIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Leporis",
        constellation: "Lepus",
        right_ascension: RightAscension::new(5, 28, 15.),
        declination: Declination::new(Sgn::Neg, 20, 45, 34.),
        apparent_magnitude: 2.84,
        distance: Length::new::<light_year>(160.),
        absolute_magnitude: -0.65,
        mass: Mass::new::<solar_mass>(3.5),
        radius: Some(Length::new::<solar_radii>(16.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5450.),
        age: Some(Time::new::<gigayear>(0.240)),
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

fn EPSILON_LEPORIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Leporis",
        constellation: "Lepus",
        right_ascension: RightAscension::new(5, 5, 28.),
        declination: Declination::new(Sgn::Neg, 22, 22, 16.),
        apparent_magnitude: 3.166,
        distance: Length::new::<light_year>(209.),
        absolute_magnitude: -1.02,
        mass: Mass::new::<solar_mass>(1.7),
        radius: Some(Length::new::<solar_radii>(40.1)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4131.),
        age: Some(Time::new::<gigayear>(1.72)),
        lifetime: Time::new::<gigayear>(1.73766023),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [ARNEB(), BETA_LEPORIS(), EPSILON_LEPORIS()] }
