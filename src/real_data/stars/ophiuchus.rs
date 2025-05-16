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

fn RASALHAGUE() -> RealData {
    RealData {
        common_name: "Rasalhague",
        astronomical_name: "α Ophiuchi",
        constellation: "Ophiuchus",
        radius: Some(Length::new::<solar_radii>(2.6)),
        mass: Mass::new::<solar_mass>(2.4),
        absolute_magnitude: 1.30,
        apparent_magnitude: 2.08,
        temperature: ThermodynamicTemperature::new::<kelvin>(8000.),
        age: Some(Time::new::<gigayear>(0.77)),
        right_ascension: RightAscension::new(17, 34, 56.),
        declination: Declination::new(Sgn::Pos, 12, 33, 37.),
        distance: Length::new::<light_year>(47.),
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

fn SABIK() -> RealData {
    RealData {
        common_name: "Sabik",
        astronomical_name: "η Ophiuchi",
        constellation: "Ophiuchus",
        radius: None,
        mass: Mass::new::<solar_mass>(2.966),
        absolute_magnitude: 0.37,
        apparent_magnitude: 2.43,
        temperature: ThermodynamicTemperature::new::<kelvin>(8900.),
        age: None,
        right_ascension: RightAscension::new(17, 10, 23.),
        declination: Declination::new(Sgn::Neg, 15, 43, 30.),
        distance: Length::new::<light_year>(84.),
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

fn HAN() -> RealData {
    RealData {
        common_name: "Han",
        astronomical_name: "ζ Ophiuchi",
        constellation: "Ophiuchus",
        radius: Some(Length::new::<solar_radii>(8.5)),
        mass: Mass::new::<solar_mass>(20.2),
        absolute_magnitude: -3.20,
        apparent_magnitude: 2.54,
        temperature: ThermodynamicTemperature::new::<kelvin>(34_300.),
        right_ascension: RightAscension::new(16, 37, 10.),
        declination: Declination::new(Sgn::Neg, 10, 34, 2.),
        distance: Length::new::<light_year>(458.),
        age: Some(Time::new::<gigayear>(0.003)),
        lifetime: Time::new::<gigayear>(0.009767659),
    }
}

fn YED_PRIOR() -> RealData {
    RealData {
        common_name: "Yed Prior",
        astronomical_name: "δ Ophiuchi",
        constellation: "Ophiuchus",
        radius: Some(Length::new::<solar_radii>(59.)),
        mass: Mass::new::<solar_mass>(1.5),
        absolute_magnitude: -0.90,
        apparent_magnitude: 2.73,
        temperature: ThermodynamicTemperature::new::<kelvin>(3679.),
        age: None,
        right_ascension: RightAscension::new(16, 14, 21.),
        declination: Declination::new(Sgn::Neg, 3, 41, 40.),
        distance: Length::new::<light_year>(171.),
        lifetime: Time::new::<gigayear>(2.54186931),
    }
}

fn CEBALRAI() -> RealData {
    RealData {
        common_name: "Cebalrai",
        astronomical_name: "β Ophiuchi",
        constellation: "Ophiuchus",
        radius: Some(Length::new::<solar_radii>(12.42)),
        mass: Mass::new::<solar_mass>(1.13),
        absolute_magnitude: 0.77,
        apparent_magnitude: 2.76,
        temperature: ThermodynamicTemperature::new::<kelvin>(4467.),
        right_ascension: RightAscension::new(17, 43, 28.),
        declination: Declination::new(Sgn::Pos, 4, 34, 2.),
        distance: Length::new::<light_year>(81.8),
        age: Some(Time::new::<gigayear>(3.82)),
        lifetime: Time::new::<gigayear>(5.9461393),
    }
}

pub(crate) fn STARS() -> [RealData; 5] { [RASALHAGUE(), SABIK(), HAN(), YED_PRIOR(), CEBALRAI()] }
