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

fn ALPHA_VULPECULAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Vulpeculae",
        constellation: "Vulpecula",
        right_ascension: RightAscension::new(19, 28, 42.),
        declination: Declination::new(Sgn::Pos, 24, 39, 54.),
        apparent_magnitude: 4.40,
        distance: Length::new::<light_year>(291.),
        absolute_magnitude: -0.36,
        mass: Mass::new::<solar_mass>(0.97),
        radius: Some(Length::new::<solar_radii>(43.14)),
        temperature: ThermodynamicTemperature::new::<kelvin>(3690.),
        age: Some(Time::new::<gigayear>(11.3)),
        lifetime: Time::new::<gigayear>(11.7800188),
    }
}

fn TWENTYTHREE_VULPECULAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "23 Vulpeculae",
        constellation: "Vulpecula",
        right_ascension: RightAscension::new(20, 15, 46.),
        declination: Declination::new(Sgn::Pos, 27, 48, 51.),
        apparent_magnitude: 4.52,
        distance: Length::new::<light_year>(327.),
        absolute_magnitude: -0.58,
        mass: Mass::new::<solar_mass>(2.4),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(4429.),
        age: None,
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

fn THIRTYONE_VULPECULAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "31 Vulpeculae",
        constellation: "Vulpecula",
        right_ascension: RightAscension::new(20, 52, 8.),
        declination: Declination::new(Sgn::Pos, 27, 5, 49.),
        apparent_magnitude: 4.56,
        distance: Length::new::<light_year>(216.5),
        absolute_magnitude: 0.449,
        mass: Mass::new::<solar_mass>(2.4),
        radius: Some(Length::new::<solar_radii>(8.01)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5261.),
        age: Some(Time::new::<gigayear>(0.7)),
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

pub(crate) const STARS: [RealData; 3] = [
    ALPHA_VULPECULAE,
    TWENTYTHREE_VULPECULAE,
    THIRTYONE_VULPECULAE,
];
