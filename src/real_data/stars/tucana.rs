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

fn ALPHA_TUCANAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Tucanae",
        constellation: "Tucana",
        radius: Some(Length::new::<solar_radii>(37.)),
        mass: Mass::new::<solar_mass>(2.5),
        absolute_magnitude: -1.05,
        apparent_magnitude: 2.87,
        temperature: ThermodynamicTemperature::new::<kelvin>(4300.),
        age: None,
        right_ascension: RightAscension::new(22, 18, 30.),
        declination: Declination::new(Sgn::Neg, 60, 15, 35.),
        distance: Length::new::<light_year>(198.5),
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

fn GAMMA_TUCANAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Tucanae",
        constellation: "Tucana",
        right_ascension: RightAscension::new(23, 17, 26.),
        declination: Declination::new(Sgn::Neg, 58, 14, 9.),
        apparent_magnitude: 3.99,
        distance: Length::new::<light_year>(75.),
        absolute_magnitude: 2.18,
        mass: Mass::new::<solar_mass>(1.55),
        radius: Some(Length::new::<solar_radii>(2.2)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6679.),
        age: Some(Time::new::<gigayear>(1.414)),
        lifetime: Time::new::<gigayear>(2.29668629),
    }
}

fn ZETA_TUCANAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Tucanae",
        constellation: "Tucana",
        right_ascension: RightAscension::new(0, 20, 4.),
        declination: Declination::new(Sgn::Neg, 64, 52, 29.),
        apparent_magnitude: 4.23,
        distance: Length::new::<light_year>(28.01),
        absolute_magnitude: 4.67,
        mass: Mass::new::<solar_mass>(0.99),
        radius: Some(Length::new::<solar_radii>(1.08)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5970.),
        age: Some(Time::new::<gigayear>(2.5)),
        lifetime: Time::new::<gigayear>(9.81519157),
    }
}

pub(crate) fn STARS() -> [RealData; 3] {
    [ALPHA_TUCANAE(), GAMMA_TUCANAE(), ZETA_TUCANAE()]
}
