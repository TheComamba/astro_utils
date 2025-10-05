use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn canopus() -> RealData {
    RealData {
        common_name: "Canopus",
        astronomical_name: "α Carinae",
        constellation: "Carina",
        radius: Some(Length::new::<solar_radius>(72.)),
        mass: Mass::new::<solar_mass>(9.),
        absolute_magnitude: -5.53,
        apparent_magnitude: -0.62,
        temperature: ThermodynamicTemperature::new::<kelvin>(7400.),
        right_ascension: RightAscension::new(6, 23, 57.),
        declination: Declination::new(Sgn::Neg, 52, 41, 44.),
        distance: Length::new::<light_year>(313.),
        age: Some(Time::new::<gigayear>(0.0251)),
        lifetime: Time::new::<gigayear>(0.03224554),
    }
}

fn miaplacidus() -> RealData {
    RealData {
        common_name: "Miaplacidus",
        astronomical_name: "β Carinae",
        constellation: "Carina",
        radius: Some(Length::new::<solar_radius>(6.8)),
        mass: Mass::new::<solar_mass>(3.5),
        absolute_magnitude: -0.99,
        apparent_magnitude: 1.67,
        temperature: ThermodynamicTemperature::new::<kelvin>(8866.),
        right_ascension: RightAscension::new(9, 13, 12.),
        declination: Declination::new(Sgn::Neg, 69, 43, 2.),
        distance: Length::new::<light_year>(111.),
        age: Some(Time::new::<gigayear>(0.260)),
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

fn avior() -> RealData {
    RealData {
        common_name: "Avior",
        astronomical_name: "ε Carinae",
        constellation: "Carina",
        radius: None,
        mass: Mass::new::<solar_mass>(10.5),
        absolute_magnitude: -4.58,
        apparent_magnitude: 1.86,
        temperature: ThermodynamicTemperature::new::<kelvin>(3523.),
        right_ascension: RightAscension::new(8, 22, 31.),
        declination: Declination::new(Sgn::Neg, 59, 30, 34.),
        distance: Length::new::<light_year>(632.),
        age: Some(Time::new::<gigayear>(0.026)),
        lifetime: Time::new::<gigayear>(0.026540021),
    }
}

fn aspidiske() -> RealData {
    RealData {
        common_name: "Aspidiske",
        astronomical_name: "ι Carinae",
        constellation: "Carina",
        radius: Some(Length::new::<solar_radius>(43.)),
        mass: Mass::new::<solar_mass>(7.4),
        absolute_magnitude: -4.42,
        apparent_magnitude: 2.21,
        temperature: ThermodynamicTemperature::new::<kelvin>(7500.),
        right_ascension: RightAscension::new(9, 17, 5.),
        declination: Declination::new(Sgn::Neg, 59, 16, 30.),
        distance: Length::new::<light_year>(694.),
        age: Some(Time::new::<gigayear>(0.0374)),
        lifetime: Time::new::<gigayear>(0.052267043),
    }
}

pub(crate) fn stars() -> [RealData; 4] {
    [canopus(), miaplacidus(), avior(), aspidiske()]
}
