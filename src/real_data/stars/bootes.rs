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

fn ARCTURUS() -> RealData {
    RealData {
        common_name: "Arcturus",
        astronomical_name: "α Boötis",
        constellation: "Boötes",
        radius: Some(Length::new::<solar_radii>(25.4)),
        mass: Mass::new::<solar_mass>(1.08),
        absolute_magnitude: -0.31,
        apparent_magnitude: -0.05,
        temperature: ThermodynamicTemperature::new::<kelvin>(4286.),
        right_ascension: RightAscension::new(14, 15, 40.),
        declination: Declination::new(Sgn::Pos, 19, 10, 56.),
        distance: Length::new::<light_year>(37.),
        age: Some(Time::new::<gigayear>(6.9)),
        lifetime: Time::new::<gigayear>(6.97272616),
    }
}

fn IZAR() -> RealData {
    RealData {
        common_name: "Izar",
        astronomical_name: "ε Boötis",
        constellation: "Boötes",
        radius: Some(Length::new::<solar_radii>(33.)),
        mass: Mass::new::<solar_mass>(4.6),
        absolute_magnitude: -1.69,
        apparent_magnitude: 2.35,
        temperature: ThermodynamicTemperature::new::<kelvin>(4550.),
        right_ascension: RightAscension::new(14, 44, 59.),
        declination: Declination::new(Sgn::Pos, 27, 4, 27.),
        distance: Length::new::<light_year>(210.),
        age: Some(Time::new::<gigayear>(0.0374)),
        lifetime: Time::new::<gigayear>(0.136126994),
    }
}

fn GAMMA_BOOTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Boötis",
        constellation: "Boötes",
        right_ascension: RightAscension::new(14, 32, 5.),
        declination: Declination::new(Sgn::Pos, 38, 18, 30.),
        apparent_magnitude: 3.03,
        distance: Length::new::<light_year>(86.8),
        absolute_magnitude: 0.93,
        mass: Mass::new::<solar_mass>(2.10),
        radius: Some(Length::new::<solar_radii>(5.16)),
        temperature: ThermodynamicTemperature::new::<kelvin>(7800.),
        age: Some(Time::new::<gigayear>(0.9)),
        lifetime: Time::new::<gigayear>(1.17901142),
    }
}

fn DELTA_BOOTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Boötis",
        constellation: "Boötes",
        right_ascension: RightAscension::new(15, 15, 30.),
        declination: Declination::new(Sgn::Pos, 33, 18, 53.),
        apparent_magnitude: 3.482,
        distance: Length::new::<light_year>(121.8),
        absolute_magnitude: 0.7,
        mass: Mass::new::<solar_mass>(1.1),
        radius: Some(Length::new::<solar_radii>(10.5)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4847.),
        age: None,
        lifetime: Time::new::<gigayear>(6.97272616),
    }
}

fn BETA_BOOTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Boötis",
        constellation: "Boötes",
        right_ascension: RightAscension::new(15, 1, 57.),
        declination: Declination::new(Sgn::Pos, 40, 23, 26.),
        apparent_magnitude: 3.488,
        distance: Length::new::<light_year>(225.),
        absolute_magnitude: -0.7,
        mass: Mass::new::<solar_mass>(3.4),
        radius: Some(Length::new::<solar_radii>(21.5)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4932.),
        age: Some(Time::new::<gigayear>(0.240)),
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

fn MUPHRID() -> RealData {
    RealData {
        common_name: "Muphrid",
        astronomical_name: "η Boötis",
        constellation: "Boötes",
        right_ascension: RightAscension::new(13, 54, 41.),
        declination: Declination::new(Sgn::Pos, 18, 23, 52.),
        apparent_magnitude: 2.680,
        distance: Length::new::<light_year>(37.2),
        absolute_magnitude: 2.41,
        mass: Mass::new::<solar_mass>(1.71),
        radius: Some(Length::new::<solar_radii>(2.672)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6100.),
        age: Some(Time::new::<gigayear>(1.6)),
        lifetime: Time::new::<gigayear>(1.73766023),
    }
}

pub(crate) fn STARS() -> [RealData; 6] { [
    ARCTURUS(),
    IZAR(),
    GAMMA_BOOTIS(),
    DELTA_BOOTIS(),
    BETA_BOOTIS(),
    MUPHRID(),
] }
