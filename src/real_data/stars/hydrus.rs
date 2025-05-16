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

fn ALPHA_HYDRI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Hydri",
        constellation: "Hydrus",
        right_ascension: RightAscension::new(1, 58, 46.),
        declination: Declination::new(Sgn::Neg, 61, 34, 11.),
        apparent_magnitude: 2.9,
        distance: Length::new::<light_year>(71.8),
        absolute_magnitude: 1.153,
        mass: Mass::new::<solar_mass>(2.),
        radius: Some(Length::new::<solar_radii>(3.040)),
        temperature: ThermodynamicTemperature::new::<kelvin>(7087.),
        age: Some(Time::new::<gigayear>(0.810)),
        lifetime: Time::new::<gigayear>(1.36020165),
    }
}

fn BETA_HYDRI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Hydri",
        constellation: "Hydrus",
        right_ascension: RightAscension::new(0, 25, 45.),
        declination: Declination::new(Sgn::Neg, 77, 15, 15.),
        apparent_magnitude: 2.8,
        distance: Length::new::<light_year>(24.33),
        absolute_magnitude: 3.45,
        mass: Mass::new::<solar_mass>(1.08),
        radius: Some(Length::new::<solar_radii>(1.809)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5872.),
        age: Some(Time::new::<gigayear>(6.4)),
        lifetime: Time::new::<gigayear>(6.97272616),
    }
}

fn GAMMA_HYDRI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Hydri",
        constellation: "Hydrus",
        radius: Some(Length::new::<solar_radii>(62.)),
        mass: Mass::new::<solar_mass>(1.),
        absolute_magnitude: -0.83,
        apparent_magnitude: 3.26,
        temperature: ThermodynamicTemperature::new::<kelvin>(3499.),
        right_ascension: RightAscension::new(3, 47, 14.),
        declination: Declination::new(Sgn::Neg, 74, 14, 20.),
        distance: Length::new::<light_year>(214.),
        age: None,
        lifetime: Time::new::<gigayear>(9.81519157),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [ALPHA_HYDRI(), BETA_HYDRI(), GAMMA_HYDRI()]
}
