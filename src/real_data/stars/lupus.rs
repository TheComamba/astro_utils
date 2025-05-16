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

fn ALPHA_LUPI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Lupi",
        constellation: "Lupus",
        radius: None,
        mass: Mass::new::<solar_mass>(10.1),
        absolute_magnitude: -3.83,
        apparent_magnitude: 2.30,
        temperature: ThermodynamicTemperature::new::<kelvin>(21_820.),
        age: Some(Time::new::<gigayear>(0.018)),
        lifetime: Time::new::<gigayear>(0.026540021),
        right_ascension: RightAscension::new(14, 41, 56.),
        declination: Declination::new(Sgn::Neg, 47, 23, 18.),
        distance: Length::new::<light_year>(548.),
    }
}

fn BETA_LUPI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Lupi",
        constellation: "Lupus",
        right_ascension: RightAscension::new(14, 58, 32.),
        declination: Declination::new(Sgn::Neg, 43, 8, 2.),
        apparent_magnitude: 2.68,
        distance: Length::new::<light_year>(523.3),
        absolute_magnitude: -3.35,
        mass: Mass::new::<solar_mass>(8.8),
        radius: Some(Length::new::<solar_radii>(6.6)),
        temperature: ThermodynamicTemperature::new::<kelvin>(24_090.),
        age: Some(Time::new::<gigayear>(0.0246)),
        lifetime: Time::new::<gigayear>(0.03224554),
    }
}

fn GAMMA_LUPI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Lupi",
        constellation: "Lupus",
        right_ascension: RightAscension::new(15, 35, 8.),
        declination: Declination::new(Sgn::Neg, 41, 10, 0.),
        apparent_magnitude: 2.77,
        distance: Length::new::<light_year>(567.),
        absolute_magnitude: -3.4,
        mass: Mass::new::<solar_mass>(9.5),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(20_900.),
        age: Some(Time::new::<gigayear>(0.0186)),
        lifetime: Time::new::<gigayear>(0.03224554),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [ALPHA_LUPI, BETA_LUPI, GAMMA_LUPI] }
