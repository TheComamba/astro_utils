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

fn REGOR() -> RealData {
    RealData {
        common_name: "Regor",
        astronomical_name: "γ Velorum",
        constellation: "Vela",
        radius: Some(Length::new::<solar_radii>(17.)),
        mass: Mass::new::<solar_mass>(28.5),
        absolute_magnitude: -5.31,
        apparent_magnitude: 1.75,
        temperature: ThermodynamicTemperature::new::<kelvin>(35_000.),
        age: Some(Time::new::<gigayear>(0.0045)),
        right_ascension: RightAscension::new(8, 9, 32.),
        declination: Declination::new(Sgn::Neg, 47, 20, 12.),
        distance: Length::new::<light_year>(840.),
        lifetime: Time::new::<gigayear>(0.006972406),
    }
}

fn ALSEPHINA() -> RealData {
    RealData {
        common_name: "Alsephina",
        astronomical_name: "δ Velorum",
        constellation: "Vela",
        radius: Some(Length::new::<solar_radii>(2.4)),
        mass: Mass::new::<solar_mass>(2.27),
        absolute_magnitude: -0.01,
        apparent_magnitude: 1.93,
        temperature: ThermodynamicTemperature::new::<kelvin>(9440.),
        age: Some(Time::new::<gigayear>(0.4)),
        right_ascension: RightAscension::new(8, 44, 42.),
        declination: Declination::new(Sgn::Neg, 54, 42, 32.),
        distance: Length::new::<light_year>(80.),
        lifetime: Time::new::<gigayear>(0.964406929),
    }
}

fn SUHAIL() -> RealData {
    RealData {
        common_name: "Suhail",
        astronomical_name: "λ Velorum",
        constellation: "Vela",
        radius: Some(Length::new::<solar_radii>(210.)),
        mass: Mass::new::<solar_mass>(7.),
        absolute_magnitude: -3.99,
        apparent_magnitude: 2.23,
        temperature: ThermodynamicTemperature::new::<kelvin>(3900.),
        age: Some(Time::new::<gigayear>(0.0316)),
        right_ascension: RightAscension::new(9, 7, 60.),
        declination: Declination::new(Sgn::Neg, 43, 25, 57.),
        distance: Length::new::<light_year>(573.),
        lifetime: Time::new::<gigayear>(0.052267043),
    }
}

fn MARKEB() -> RealData {
    RealData {
        common_name: "Markeb",
        astronomical_name: "κ Velorum",
        constellation: "Vela",
        radius: Some(Length::new::<solar_radii>(9.1)),
        mass: Mass::new::<solar_mass>(10.5),
        absolute_magnitude: -3.62,
        apparent_magnitude: 2.47,
        temperature: ThermodynamicTemperature::new::<kelvin>(23_000.),
        age: Some(Time::new::<gigayear>(0.018)),
        right_ascension: RightAscension::new(9, 22, 7.),
        declination: Declination::new(Sgn::Neg, 55, 0, 38.),
        distance: Length::new::<light_year>(539.),
        lifetime: Time::new::<gigayear>(0.026540021),
    }
}

pub(crate) fn STARS() -> [RealData; 4] {
    [REGOR(), ALSEPHINA(), SUHAIL(), MARKEB()]
}
