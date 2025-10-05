use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn alpha_mensae() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Mensae",
        constellation: "Mensa",
        right_ascension: RightAscension::new(6, 10, 14.),
        declination: Declination::new(Sgn::Neg, 74, 45, 11.),
        apparent_magnitude: 5.09,
        distance: Length::new::<light_year>(33.31),
        absolute_magnitude: 5.03,
        mass: Mass::new::<solar_mass>(0.964),
        radius: Some(Length::new::<solar_radii>(0.960)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5569.),
        age: Some(Time::new::<gigayear>(6.2)),
        lifetime: Time::new::<gigayear>(11.7800188),
    }
}

fn beta_mensae() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Mensae",
        constellation: "Mensa",
        right_ascension: RightAscension::new(5, 2, 43.),
        declination: Declination::new(Sgn::Neg, 71, 18, 51.),
        apparent_magnitude: 5.31,
        distance: Length::new::<light_year>(641.7),
        absolute_magnitude: -1.17,
        mass: Mass::new::<solar_mass>(3.58),
        radius: Some(Length::new::<solar_radii>(25.85)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5088.),
        age: Some(Time::new::<gigayear>(0.250)),
        lifetime: Time::new::<gigayear>(0.254814649),
    }
}

fn gamma_mensae() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Mensae",
        constellation: "Mensa",
        right_ascension: RightAscension::new(5, 31, 53.),
        declination: Declination::new(Sgn::Neg, 76, 20, 27.),
        apparent_magnitude: 5.19,
        distance: Length::new::<light_year>(104.9),
        absolute_magnitude: 2.70,
        mass: Mass::new::<solar_mass>(1.04),
        radius: Some(Length::new::<solar_radii>(4.99)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4491.),
        age: Some(Time::new::<gigayear>(8.)),
        lifetime: Time::new::<gigayear>(8.24015833),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [alpha_mensae(), beta_mensae(), gamma_mensae()]
}
