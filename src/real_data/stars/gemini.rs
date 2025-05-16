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

fn POLLUX() -> RealData {
    RealData {
        common_name: "Pollux",
        astronomical_name: "β Geminorum",
        constellation: "Gemini",
        radius: Some(Length::new::<solar_radii>(9.06)),
        mass: Mass::new::<solar_mass>(1.91),
        absolute_magnitude: 1.09,
        apparent_magnitude: 1.16,
        temperature: ThermodynamicTemperature::new::<kelvin>(4586.),
        age: Some(Time::new::<gigayear>(0.724)),
        lifetime: Time::new::<gigayear>(1.54706939),
        right_ascension: RightAscension::new(7, 45, 19.),
        declination: Declination::new(Sgn::Pos, 28, 1, 34.),
        distance: Length::new::<light_year>(34.),
    }
}

fn CASTOR() -> RealData {
    RealData {
        common_name: "Castor",
        astronomical_name: "α Geminorum",
        constellation: "Gemini",
        radius: Some(Length::new::<solar_radii>(2.089)),
        mass: Mass::new::<solar_mass>(2.37),
        absolute_magnitude: 0.59,
        apparent_magnitude: 1.58,
        temperature: ThermodynamicTemperature::new::<kelvin>(10_286.),
        age: Some(Time::new::<gigayear>(0.290)),
        lifetime: Time::new::<gigayear>(0.800458342),
        right_ascension: RightAscension::new(7, 34, 36.),
        declination: Declination::new(Sgn::Pos, 31, 53, 18.),
        distance: Length::new::<light_year>(52.),
    }
}

fn ALHENA() -> RealData {
    RealData {
        common_name: "Alhena",
        astronomical_name: "γ Geminorum",
        constellation: "Gemini",
        radius: Some(Length::new::<solar_radii>(3.3)),
        mass: Mass::new::<solar_mass>(2.81),
        absolute_magnitude: -0.60,
        apparent_magnitude: 1.93,
        temperature: ThermodynamicTemperature::new::<kelvin>(9260.),
        age: None,
        lifetime: Time::new::<gigayear>(0.513076303),
        right_ascension: RightAscension::new(6, 37, 43.),
        declination: Declination::new(Sgn::Pos, 16, 23, 57.),
        distance: Length::new::<light_year>(105.),
    }
}

fn TEJAT() -> RealData {
    RealData {
        common_name: "Tejat",
        astronomical_name: "μ Geminorum",
        constellation: "Gemini",
        radius: Some(Length::new::<solar_radii>(90.)),
        mass: Mass::new::<solar_mass>(2.1),
        absolute_magnitude: -1.42,
        apparent_magnitude: 2.75,
        temperature: ThermodynamicTemperature::new::<kelvin>(3460.),
        age: None,
        lifetime: Time::new::<gigayear>(1.17901142),
        right_ascension: RightAscension::new(6, 22, 58.),
        declination: Declination::new(Sgn::Pos, 22, 30, 49.),
        distance: Length::new::<light_year>(230.),
    }
}

fn PROPUS() -> RealData {
    RealData {
        common_name: "Propus",
        astronomical_name: "η Geminorum",
        constellation: "Gemini",
        radius: Some(Length::new::<solar_radii>(275.)),
        mass: Mass::new::<solar_mass>(2.5),
        absolute_magnitude: -1.84,
        apparent_magnitude: 3.31,
        temperature: ThermodynamicTemperature::new::<kelvin>(3502.),
        age: Some(Time::new::<gigayear>(0.8)),
        lifetime: Time::new::<gigayear>(0.800458342),
        right_ascension: RightAscension::new(6, 14, 53.),
        declination: Declination::new(Sgn::Pos, 22, 30, 24.),
        distance: Length::new::<light_year>(349.),
    }
}

pub(crate) fn STARS() -> [RealData; 5] { [POLLUX(), CASTOR(), ALHENA(), TEJAT(), PROPUS()] }
