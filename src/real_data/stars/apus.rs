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

fn alpha_apodis() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Apodis",
        constellation: "Apus",
        radius: Some(Length::new::<solar_radii>(48.)),
        mass: Mass::new::<solar_mass>(1.2),
        absolute_magnitude: -1.67,
        apparent_magnitude: 3.825,
        temperature: ThermodynamicTemperature::new::<kelvin>(4312.),
        age: None,
        lifetime: Time::new::<gigayear>(5.06543331),
        right_ascension: RightAscension::new(14, 47, 52.),
        declination: Declination::new(Sgn::Neg, 79, 2, 41.),
        distance: Length::new::<light_year>(411.1),
    }
}

fn gamma_apodis() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Apodis",
        constellation: "Apus",
        radius: None,
        mass: Mass::new::<solar_mass>(0.95),
        absolute_magnitude: 0.41,
        apparent_magnitude: 3.86,
        temperature: ThermodynamicTemperature::new::<kelvin>(5040.),
        age: None,
        lifetime: Time::new::<gigayear>(11.7800188),
        right_ascension: RightAscension::new(16, 33, 27.),
        declination: Declination::new(Sgn::Neg, 78, 53, 50.),
        distance: Length::new::<light_year>(150.),
    }
}

fn beta_apodis() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Apodis",
        constellation: "Apus",
        radius: Some(Length::new::<solar_radii>(11.)),
        mass: Mass::new::<solar_mass>(1.84),
        absolute_magnitude: 0.819,
        apparent_magnitude: 4.24,
        temperature: ThermodynamicTemperature::new::<kelvin>(4900.),
        age: None,
        lifetime: Time::new::<gigayear>(1.65092742),
        right_ascension: RightAscension::new(16, 43, 5.),
        declination: Declination::new(Sgn::Neg, 77, 31, 3.),
        distance: Length::new::<light_year>(149.),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [alpha_apodis(), gamma_apodis(), beta_apodis()]
}
