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

fn ALDEBARAN() -> RealData {
    RealData {
        common_name: "Aldebaran",
        astronomical_name: "α Tauri",
        constellation: "Taurus",
        radius: Some(Length::new::<solar_radii>(45.1)),
        mass: Mass::new::<solar_mass>(1.16),
        absolute_magnitude: -0.63,
        apparent_magnitude: 0.87,
        temperature: ThermodynamicTemperature::new::<kelvin>(3900.),
        age: Some(Time::new::<gigayear>(5.5)),
        right_ascension: RightAscension::new(4, 35, 55.),
        declination: Declination::new(Sgn::Pos, 16, 30, 33.),
        distance: Length::new::<light_year>(65.),
        lifetime: Time::new::<gigayear>(5.9461393),
    }
}

fn ALNATH() -> RealData {
    RealData {
        common_name: "Alnath",
        astronomical_name: "β Tauri",
        constellation: "Taurus",
        radius: Some(Length::new::<solar_radii>(4.2)),
        mass: Mass::new::<solar_mass>(5.0),
        absolute_magnitude: -1.37,
        apparent_magnitude: 1.65,
        temperature: ThermodynamicTemperature::new::<kelvin>(13_824.),
        age: Some(Time::new::<gigayear>(0.1)),
        right_ascension: RightAscension::new(5, 26, 18.),
        declination: Declination::new(Sgn::Pos, 28, 36, 27.),
        distance: Length::new::<light_year>(131.),
        lifetime: Time::new::<gigayear>(0.111319448),
    }
}

fn GAMMA_TAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Tauri",
        constellation: "Taurus",
        right_ascension: RightAscension::new(4, 19, 48.),
        declination: Declination::new(Sgn::Pos, 15, 37, 40.),
        apparent_magnitude: 3.654,
        distance: Length::new::<light_year>(154.),
        absolute_magnitude: 0.22,
        mass: Mass::new::<solar_mass>(2.7),
        radius: Some(Length::new::<solar_radii>(13.4)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4844.),
        age: Some(Time::new::<gigayear>(0.5)),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn EPSILON_TAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Tauri",
        constellation: "Taurus",
        right_ascension: RightAscension::new(4, 28, 37.),
        declination: Declination::new(Sgn::Pos, 19, 10, 50.),
        apparent_magnitude: 3.53,
        distance: Length::new::<light_year>(146.),
        absolute_magnitude: 0.145,
        mass: Mass::new::<solar_mass>(2.57),
        radius: Some(Length::new::<solar_radii>(12.35)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4950.),
        age: Some(Time::new::<gigayear>(0.625)),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn LAMBDA_TAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "λ Tauri",
        constellation: "Taurus",
        right_ascension: RightAscension::new(4, 0, 41.),
        declination: Declination::new(Sgn::Pos, 12, 29, 25.),
        apparent_magnitude: 3.37,
        distance: Length::new::<light_year>(480.),
        absolute_magnitude: -2.45,
        mass: Mass::new::<solar_mass>(7.18),
        radius: Some(Length::new::<solar_radii>(6.4)),
        temperature: ThermodynamicTemperature::new::<kelvin>(18_700.),
        age: Some(Time::new::<gigayear>(0.0332)),
        lifetime: Time::new::<gigayear>(0.052267043),
    }
}

fn ZETA_TAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Tauri",
        constellation: "Taurus",
        right_ascension: RightAscension::new(5, 37, 39.),
        declination: Declination::new(Sgn::Pos, 21, 8, 33.),
        apparent_magnitude: 3.010,
        distance: Length::new::<light_year>(440.),
        absolute_magnitude: -2.67,
        mass: Mass::new::<solar_mass>(11.2),
        radius: Some(Length::new::<solar_radii>(5.5)),
        temperature: ThermodynamicTemperature::new::<kelvin>(15_500.),
        age: Some(Time::new::<gigayear>(0.019)),
        lifetime: Time::new::<gigayear>(0.019450199),
    }
}

pub(crate) fn stars() -> [RealData; 6] {
    [
        ALDEBARAN(),
        ALNATH(),
        GAMMA_TAURI(),
        EPSILON_TAURI(),
        LAMBDA_TAURI(),
        ZETA_TAURI(),
    ]
}
