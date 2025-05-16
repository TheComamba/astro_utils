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

fn ALPHA_SCUTI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Scuti",
        constellation: "Scutum",
        right_ascension: RightAscension::new(18, 35, 12.),
        declination: Declination::new(Sgn::Neg, 8, 14, 39.),
        apparent_magnitude: 3.83,
        distance: Length::new::<light_year>(199.),
        absolute_magnitude: -0.08,
        mass: Mass::new::<solar_mass>(1.33),
        radius: Some(Length::new::<solar_radii>(20.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4315.),
        age: None,
        lifetime: Time::new::<gigayear>(3.46068223),
    }
}

fn BETA_SCUTI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Scuti",
        constellation: "Scutum",
        right_ascension: RightAscension::new(18, 47, 10.),
        declination: Declination::new(Sgn::Neg, 4, 44, 52.),
        apparent_magnitude: 4.22,
        distance: Length::new::<light_year>(900.),
        absolute_magnitude: -2.99,
        mass: Mass::new::<solar_mass>(3.0),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(4622.),
        age: None,
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

fn ZETA_SCUTI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Scuti",
        constellation: "Scutum",
        right_ascension: RightAscension::new(18, 23, 40.),
        declination: Declination::new(Sgn::Neg, 8, 56, 4.),
        apparent_magnitude: 4.66,
        distance: Length::new::<light_year>(210.),
        absolute_magnitude: 0.66,
        mass: Mass::new::<solar_mass>(1.29),
        radius: Some(Length::new::<solar_radii>(9.3)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4750.),
        age: None,
        lifetime: Time::new::<gigayear>(3.9126515),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [ALPHA_SCUTI(), BETA_SCUTI(), ZETA_SCUTI()]
}
