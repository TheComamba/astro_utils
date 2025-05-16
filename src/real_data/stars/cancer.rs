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

fn TARF() -> RealData {
    RealData {
        common_name: "Tarf",
        astronomical_name: "β Cancri",
        constellation: "Cancer",
        radius: Some(Length::new::<solar_radii>(47.2)),
        mass: Mass::new::<solar_mass>(1.7),
        absolute_magnitude: -1.218,
        apparent_magnitude: 3.50,
        temperature: ThermodynamicTemperature::new::<kelvin>(4092.),
        right_ascension: RightAscension::new(8, 16, 31.),
        declination: Declination::new(Sgn::Pos, 9, 11, 8.),
        distance: Length::new::<light_year>(290.),
        age: Some(Time::new::<gigayear>(1.7)),
        lifetime: Time::new::<gigayear>(1.73766023),
    }
}

fn ASELLUS_AUSTRALIS() -> RealData {
    RealData {
        common_name: "Asellus Australis",
        astronomical_name: "δ Cancri",
        constellation: "Cancer",
        radius: Some(Length::new::<solar_radii>(11.)),
        mass: Mass::new::<solar_mass>(1.71),
        absolute_magnitude: 0.843,
        apparent_magnitude: 3.94,
        temperature: ThermodynamicTemperature::new::<kelvin>(4637.),
        right_ascension: RightAscension::new(8, 44, 41.),
        declination: Declination::new(Sgn::Pos, 18, 9, 16.),
        distance: Length::new::<light_year>(131.),
        age: Some(Time::new::<gigayear>(1.5)),
        lifetime: Time::new::<gigayear>(1.73766023),
    }
}

fn IOTA_CANCRI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ι Cancri",
        constellation: "Cancer",
        radius: Some(Length::new::<solar_radii>(21.)),
        mass: Mass::new::<solar_mass>(3.43),
        absolute_magnitude: -0.79,
        apparent_magnitude: 4.02,
        temperature: ThermodynamicTemperature::new::<kelvin>(4954.),
        right_ascension: RightAscension::new(8, 46, 42.),
        declination: Declination::new(Sgn::Pos, 28, 45, 36.),
        distance: Length::new::<light_year>(330.),
        age: None,
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [TARF(), ASELLUS_AUSTRALIS(), IOTA_CANCRI()]
}
