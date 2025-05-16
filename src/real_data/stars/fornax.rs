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

fn DALIM() -> RealData {
    RealData {
        common_name: "Dalim",
        astronomical_name: "α Fornacis",
        constellation: "Fornax",
        right_ascension: RightAscension::new(3, 12, 5.),
        declination: Declination::new(Sgn::Neg, 28, 59, 15.),
        apparent_magnitude: 3.85,
        distance: Length::new::<light_year>(45.66),
        absolute_magnitude: 3.08,
        mass: Mass::new::<solar_mass>(1.33),
        radius: Some(Length::new::<solar_radii>(2.04)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6240.),
        age: Some(Time::new::<gigayear>(2.9)),
        lifetime: Time::new::<gigayear>(3.46068223),
    }
}

fn BETA_FORNACIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Fornacis",
        constellation: "Fornax",
        right_ascension: RightAscension::new(2, 49, 5.),
        declination: Declination::new(Sgn::Neg, 32, 24, 21.),
        apparent_magnitude: 4.46,
        distance: Length::new::<light_year>(178.),
        absolute_magnitude: 0.894,
        mass: Mass::new::<solar_mass>(1.53),
        radius: Some(Length::new::<solar_radii>(11.02)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4820.),
        age: None,
        lifetime: Time::new::<gigayear>(2.29668629),
    }
}

fn NU_FORNACIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ν Fornacis",
        constellation: "Fornax",
        right_ascension: RightAscension::new(2, 4, 29.),
        declination: Declination::new(Sgn::Neg, 29, 17, 49.),
        apparent_magnitude: 4.69,
        distance: Length::new::<light_year>(370.),
        absolute_magnitude: -0.6,
        mass: Mass::new::<solar_mass>(3.65),
        radius: Some(Length::new::<solar_radii>(3.44)),
        temperature: ThermodynamicTemperature::new::<kelvin>(13_400.),
        age: None,
        lifetime: Time::new::<gigayear>(0.254814649),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [DALIM, BETA_FORNACIS, NU_FORNACIS] }
