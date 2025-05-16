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

fn REGULUS() -> RealData {
    RealData {
        common_name: "Regulus",
        astronomical_name: "α Leonis",
        constellation: "Leo",
        radius: Some(Length::new::<solar_radii>(4.35)),
        mass: Mass::new::<solar_mass>(3.8),
        absolute_magnitude: -0.52,
        apparent_magnitude: 1.36,
        temperature: ThermodynamicTemperature::new::<kelvin>(11_668.),
        age: Some(Time::new::<gigayear>(0.100)),
        lifetime: Time::new::<gigayear>(0.220601963),
        right_ascension: RightAscension::new(10, 8, 22.),
        declination: Declination::new(Sgn::Pos, 11, 58, 2.),
        distance: Length::new::<light_year>(77.),
    }
}

fn ALGIEBA() -> RealData {
    RealData {
        common_name: "Algieba",
        astronomical_name: "γ Leonis",
        constellation: "Leo",
        radius: Some(Length::new::<solar_radii>(31.88)),
        mass: Mass::new::<solar_mass>(1.23),
        absolute_magnitude: -0.92,
        apparent_magnitude: 2.01,
        temperature: ThermodynamicTemperature::new::<kelvin>(4470.),
        right_ascension: RightAscension::new(10, 19, 58.),
        declination: Declination::new(Sgn::Pos, 19, 50, 29.),
        distance: Length::new::<light_year>(126.),
        age: None,
        lifetime: Time::new::<gigayear>(4.45521207),
    }
}

fn DENEBOLA() -> RealData {
    RealData {
        common_name: "Denebola",
        astronomical_name: "β Leonis",
        constellation: "Leo",
        radius: Some(Length::new::<solar_radii>(1.728)),
        mass: Mass::new::<solar_mass>(1.78),
        absolute_magnitude: 1.92,
        apparent_magnitude: 2.14,
        temperature: ThermodynamicTemperature::new::<kelvin>(8500.),
        age: Some(Time::new::<gigayear>(0.25)),
        lifetime: Time::new::<gigayear>(1.46605285),
        right_ascension: RightAscension::new(11, 49, 3.),
        declination: Declination::new(Sgn::Pos, 14, 34, 19.),
        distance: Length::new::<light_year>(36.),
    }
}

fn ZOSMA() -> RealData {
    RealData {
        common_name: "Zosma",
        astronomical_name: "δ Leonis",
        constellation: "Leo",
        radius: Some(Length::new::<solar_radii>(2.14)),
        mass: Mass::new::<solar_mass>(2.2),
        absolute_magnitude: 1.32,
        apparent_magnitude: 2.56,
        temperature: ThermodynamicTemperature::new::<kelvin>(8_296.),
        age: Some(Time::new::<gigayear>(0.65)),
        right_ascension: RightAscension::new(11, 14, 7.),
        declination: Declination::new(Sgn::Pos, 20, 31, 25.),
        distance: Length::new::<light_year>(58.),
        lifetime: Time::new::<gigayear>(1.03650581),
    }
}

pub(crate) fn STARS() -> [RealData; 4] { [REGULUS(), ALGIEBA(), DENEBOLA(), ZOSMA()] }
