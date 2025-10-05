use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn gamma_sagittae() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Sagittae",
        constellation: "Sagitta",
        right_ascension: RightAscension::new(19, 58, 45.),
        declination: Declination::new(Sgn::Pos, 19, 29, 32.),
        apparent_magnitude: 3.47,
        distance: Length::new::<light_year>(288.),
        absolute_magnitude: -1.11,
        mass: Mass::new::<solar_mass>(0.88),
        radius: Some(Length::new::<solar_radii>(55.13)),
        temperature: ThermodynamicTemperature::new::<kelvin>(3862.),
        age: Some(Time::new::<gigayear>(2.35)),
        lifetime: Time::new::<gigayear>(14.2493142),
    }
}

fn delta_sagittae() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Sagittae",
        constellation: "Sagitta",
        right_ascension: RightAscension::new(19, 47, 23.),
        declination: Declination::new(Sgn::Pos, 18, 32, 4.),
        apparent_magnitude: 3.82,
        distance: Length::new::<light_year>(550.),
        absolute_magnitude: -2.58,
        mass: Mass::new::<solar_mass>(3.35),
        radius: Some(Length::new::<solar_radii>(108.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(3660.),
        age: None,
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

fn alpha_sagittae() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Sagittae",
        constellation: "Sagitta",
        right_ascension: RightAscension::new(19, 40, 6.),
        declination: Declination::new(Sgn::Pos, 18, 0, 50.),
        apparent_magnitude: 4.38,
        distance: Length::new::<light_year>(382.),
        absolute_magnitude: -0.96,
        mass: Mass::new::<solar_mass>(4.11),
        radius: Some(Length::new::<solar_radii>(21.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5333.),
        age: Some(Time::new::<gigayear>(0.151)),
        lifetime: Time::new::<gigayear>(0.170765802),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [gamma_sagittae(), delta_sagittae(), alpha_sagittae()]
}
