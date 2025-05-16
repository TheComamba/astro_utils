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

fn ALPHA_SEXTANTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Sextantis",
        constellation: "Sextans",
        right_ascension: RightAscension::new(10, 7, 56.),
        declination: Declination::new(Sgn::Neg, 0, 22, 18.),
        apparent_magnitude: 4.49,
        distance: Length::new::<light_year>(280.),
        absolute_magnitude: -0.29,
        mass: Mass::new::<solar_mass>(2.57),
        radius: Some(Length::new::<solar_radii>(3.07)),
        temperature: ThermodynamicTemperature::new::<kelvin>(9984.),
        age: Some(Time::new::<gigayear>(0.385)),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn GAMMA_SEXTANTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Sextantis",
        constellation: "Sextans",
        right_ascension: RightAscension::new(9, 52, 30.),
        declination: Declination::new(Sgn::Neg, 8, 6, 18.),
        apparent_magnitude: 5.05,
        distance: Length::new::<light_year>(280.),
        absolute_magnitude: 0.43,
        mass: Mass::new::<solar_mass>(2.60),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(9825.),
        age: Some(Time::new::<gigayear>(0.401)),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn BETA_SEXTANTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Sextantis",
        constellation: "Sextans",
        right_ascension: RightAscension::new(10, 30, 17.),
        declination: Declination::new(Sgn::Neg, 0, 38, 13.),
        apparent_magnitude: 5.07,
        distance: Length::new::<light_year>(364.),
        absolute_magnitude: -0.38,
        mass: Mass::new::<solar_mass>(5.1),
        radius: Some(Length::new::<solar_radii>(3.2)),
        temperature: ThermodynamicTemperature::new::<kelvin>(14_570.),
        age: None,
        lifetime: Time::new::<gigayear>(0.111319448),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [ALPHA_SEXTANTIS(), GAMMA_SEXTANTIS(), BETA_SEXTANTIS()]
}
