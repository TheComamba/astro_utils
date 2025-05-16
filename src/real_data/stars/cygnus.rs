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

fn DENEB() -> RealData {
    RealData {
        common_name: "Deneb",
        astronomical_name: "α Cygni",
        constellation: "Cygnus",
        radius: Some(Length::new::<solar_radii>(203.)),
        mass: Mass::new::<solar_mass>(19.),
        absolute_magnitude: -7.13,
        apparent_magnitude: 1.25,
        temperature: ThermodynamicTemperature::new::<kelvin>(8515.),
        right_ascension: RightAscension::new(20, 41, 26.),
        declination: Declination::new(Sgn::Pos, 45, 16, 49.),
        distance: Length::new::<light_year>(1548.),
        age: Some(Time::new::<gigayear>(0.011)),
        lifetime: Time::new::<gigayear>(0.011037517),
    }
}

fn SADIR() -> RealData {
    RealData {
        common_name: "Sadir",
        astronomical_name: "γ Cygni",
        constellation: "Cygnus",
        radius: Some(Length::new::<solar_radii>(150.)),
        mass: Mass::new::<solar_mass>(12.11),
        absolute_magnitude: -6.12,
        apparent_magnitude: 2.23,
        temperature: ThermodynamicTemperature::new::<kelvin>(5790.),
        age: Some(Time::new::<gigayear>(0.012)),
        lifetime: Time::new::<gigayear>(0.019450199),
        right_ascension: RightAscension::new(20, 22, 14.),
        declination: Declination::new(Sgn::Pos, 40, 15, 24.),
        distance: Length::new::<light_year>(1522.),
    }
}

fn ALJANAH() -> RealData {
    RealData {
        common_name: "Aljanah",
        astronomical_name: "ε Cygni",
        constellation: "Cygnus",
        radius: Some(Length::new::<solar_radii>(10.82)),
        mass: Mass::new::<solar_mass>(2.),
        absolute_magnitude: 0.76,
        apparent_magnitude: 2.48,
        temperature: ThermodynamicTemperature::new::<kelvin>(4710.),
        age: Some(Time::new::<gigayear>(1.3)),
        lifetime: Time::new::<gigayear>(1.36020165),
        right_ascension: RightAscension::new(20, 46, 13.),
        declination: Declination::new(Sgn::Pos, 33, 58, 13.),
        distance: Length::new::<light_year>(72.),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [DENEB(), SADIR(), ALJANAH()]
}
