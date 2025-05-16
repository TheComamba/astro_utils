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

fn ALPHA_INDI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Indi",
        constellation: "Indus",
        right_ascension: RightAscension::new(20, 37, 34.),
        declination: Declination::new(Sgn::Neg, 47, 17, 29.),
        apparent_magnitude: 3.11,
        distance: Length::new::<light_year>(98.3),
        absolute_magnitude: 0.65,
        mass: Mass::new::<solar_mass>(2.0),
        radius: Some(Length::new::<solar_radii>(12.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4893.),
        age: Some(Time::new::<gigayear>(1.)),
        lifetime: Time::new::<gigayear>(1.36020165),
    }
}

fn BETA_INDI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Indi",
        constellation: "Indus",
        right_ascension: RightAscension::new(20, 54, 49.),
        declination: Declination::new(Sgn::Neg, 58, 27, 15.),
        apparent_magnitude: 3.67,
        distance: Length::new::<light_year>(600.),
        absolute_magnitude: -2.664,
        mass: Mass::new::<solar_mass>(6.7),
        radius: Some(Length::new::<solar_radii>(55.58)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4541.),
        age: Some(Time::new::<gigayear>(0.0532)),
        lifetime: Time::new::<gigayear>(0.063411557),
    }
}

fn ETA_INDI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "η Indi",
        constellation: "Indus",
        right_ascension: RightAscension::new(20, 44, 2.),
        declination: Declination::new(Sgn::Neg, 51, 55, 15.),
        apparent_magnitude: 4.52,
        distance: Length::new::<light_year>(78.8),
        absolute_magnitude: 2.59,
        mass: Mass::new::<solar_mass>(1.6),
        radius: Some(Length::new::<solar_radii>(2.27)),
        temperature: ThermodynamicTemperature::new::<kelvin>(7694.),
        age: Some(Time::new::<gigayear>(0.1)),
        lifetime: Time::new::<gigayear>(2.08398753),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [ALPHA_INDI(), BETA_INDI(), ETA_INDI()] }
