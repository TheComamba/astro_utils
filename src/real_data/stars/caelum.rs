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

fn ALPHA_CAELI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Caeli",
        constellation: "Caelum",
        radius: Some(Length::new::<solar_radii>(1.3)),
        mass: Mass::new::<solar_mass>(1.48),
        absolute_magnitude: 2.92,
        apparent_magnitude: 4.44,
        temperature: ThermodynamicTemperature::new::<kelvin>(6991.),
        right_ascension: RightAscension::new(4, 40, 34.),
        declination: Declination::new(Sgn::Neg, 41, 51, 50.),
        distance: Length::new::<light_year>(65.63),
        age: Some(Time::new::<gigayear>(0.9)),
        lifetime: Time::new::<gigayear>(2.54186931),
    }
}

fn GAMMA1_CAELI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ¹ Caeli",
        constellation: "Caelum",
        radius: Some(Length::new::<solar_radii>(14.31)),
        mass: Mass::new::<solar_mass>(1.4),
        absolute_magnitude: 0.781,
        apparent_magnitude: 4.57,
        temperature: ThermodynamicTemperature::new::<kelvin>(4411.),
        right_ascension: RightAscension::new(5, 4, 24.),
        declination: Declination::new(Sgn::Neg, 35, 28, 59.),
        distance: Length::new::<light_year>(185.),
        age: None,
        lifetime: Time::new::<gigayear>(3.10253119),
    }
}

fn BETA_CAELI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Caeli",
        constellation: "Caelum",
        radius: Some(Length::new::<solar_radii>(1.3)),
        mass: Mass::new::<solar_mass>(1.32),
        absolute_magnitude: 2.64,
        apparent_magnitude: 5.04,
        temperature: ThermodynamicTemperature::new::<kelvin>(6763.),
        right_ascension: RightAscension::new(4, 42, 3.),
        declination: Declination::new(Sgn::Neg, 37, 8, 39.),
        distance: Length::new::<light_year>(94.),
        age: Some(Time::new::<gigayear>(1.753)),
        lifetime: Time::new::<gigayear>(3.9126515),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [ALPHA_CAELI(), GAMMA1_CAELI(), BETA_CAELI()]
}
