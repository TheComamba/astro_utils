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

fn ALPHARD() -> RealData {
    RealData {
        common_name: "Alphard",
        astronomical_name: "α Hydrae",
        constellation: "Hydra",
        radius: Some(Length::new::<solar_radii>(50.5)),
        mass: Mass::new::<solar_mass>(3.03),
        absolute_magnitude: -1.69,
        apparent_magnitude: 1.99,
        temperature: ThermodynamicTemperature::new::<kelvin>(4120.),
        age: Some(Time::new::<gigayear>(0.42)),
        lifetime: Time::new::<gigayear>(0.420724107),
        right_ascension: RightAscension::new(9, 27, 35.),
        declination: Declination::new(Sgn::Neg, 8, 39, 30.),
        distance: Length::new::<light_year>(177.),
    }
}

fn GAMMA_HYDRAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Hydrae",
        constellation: "Hydra",
        right_ascension: RightAscension::new(13, 18, 55.),
        declination: Declination::new(Sgn::Neg, 23, 10, 17.),
        apparent_magnitude: 2.993,
        distance: Length::new::<light_year>(133.8),
        absolute_magnitude: -0.15,
        mass: Mass::new::<solar_mass>(2.94),
        radius: Some(Length::new::<solar_radii>(16.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5087.),
        age: Some(Time::new::<gigayear>(0.372)),
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

fn ZETA_HYDRAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Hydrae",
        constellation: "Hydra",
        right_ascension: RightAscension::new(8, 55, 24.),
        declination: Declination::new(Sgn::Pos, 5, 56, 44.),
        apparent_magnitude: 3.1,
        distance: Length::new::<light_year>(167.),
        absolute_magnitude: -0.24,
        mass: Mass::new::<solar_mass>(4.2),
        radius: Some(Length::new::<solar_radii>(17.9)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4925.),
        age: Some(Time::new::<gigayear>(0.17)),
        lifetime: Time::new::<gigayear>(0.170765802),
    }
}

fn NU_HYDRAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ν Hydrae",
        constellation: "Hydra",
        right_ascension: RightAscension::new(10, 49, 37.),
        declination: Declination::new(Sgn::Neg, 16, 11, 37.),
        apparent_magnitude: 3.115,
        distance: Length::new::<light_year>(137.1),
        absolute_magnitude: -0.11,
        mass: Mass::new::<solar_mass>(2.0),
        radius: Some(Length::new::<solar_radii>(21.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4335.),
        age: None,
        lifetime: Time::new::<gigayear>(1.36020165),
    }
}

pub(crate) fn STARS() -> [RealData; 4] {
    [ALPHARD(), GAMMA_HYDRAE(), ZETA_HYDRAE(), NU_HYDRAE()]
}
