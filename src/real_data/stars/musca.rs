use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{solar_radii },
        mass::{solar_mass, },
        time::{gigayear, },
    },
};

fn ALPHA_MUSCAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Muscae",
        constellation: "Musca",
        right_ascension: RightAscension::new(12, 37, 11.),
        declination: Declination::new(Sgn::Neg, 69, 8, 8.),
        apparent_magnitude: 2.69,
        distance: Length::new::<light_year>(315.),
        absolute_magnitude: -2.2,
        mass: Mass::new::<solar_mass>(8.8),
        radius: Some(Length::new::<solar_radii>(4.8)),
        temperature: ThermodynamicTemperature::new::<kelvin>(21_400.),
        age: Some(Time::new::<gigayear>(0.0183)),
        lifetime: Time::new::<gigayear>(0.03224554),
    }
}

fn BETA_MUSCAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Muscae",
        constellation: "Musca",
        right_ascension: RightAscension::new(12, 46, 17.),
        declination: Declination::new(Sgn::Neg, 68, 6, 29.),
        apparent_magnitude: 3.05,
        distance: Length::new::<light_year>(340.),
        absolute_magnitude: -2.06,
        mass: Mass::new::<solar_mass>(7.35),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(21_000.),
        age: Some(Time::new::<gigayear>(0.0151)),
        lifetime: Time::new::<gigayear>(0.052267043),
    }
}

fn DELTA_MUSCAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Muscae",
        constellation: "Musca",
        right_ascension: RightAscension::new(13, 2, 16.),
        declination: Declination::new(Sgn::Neg, 71, 32, 56.),
        apparent_magnitude: 3.61,
        distance: Length::new::<light_year>(91.),
        absolute_magnitude: 1.38,
        mass: Mass::new::<solar_mass>(1.1),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(4_400.),
        age: None,
        lifetime: Time::new::<gigayear>(6.97272616),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [ALPHA_MUSCAE(), BETA_MUSCAE(), DELTA_MUSCAE()] }
