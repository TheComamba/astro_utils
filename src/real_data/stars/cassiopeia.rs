use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn navi() -> RealData {
    RealData {
        common_name: "Navi",
        astronomical_name: "γ Cassiopeiae",
        constellation: "Cassiopeia",
        radius: Some(Length::new::<solar_radius>(10.)),
        mass: Mass::new::<solar_mass>(13.),
        absolute_magnitude: -4.22,
        apparent_magnitude: 2.20,
        temperature: ThermodynamicTemperature::new::<kelvin>(25_000.),
        right_ascension: RightAscension::new(0, 56, 43.),
        declination: Declination::new(Sgn::Pos, 60, 43, 0.),
        distance: Length::new::<light_year>(613.),
        age: Some(Time::new::<gigayear>(0.008)),
        lifetime: Time::new::<gigayear>(0.019450199),
    }
}

fn schedar() -> RealData {
    RealData {
        common_name: "Schedar",
        astronomical_name: "α Cassiopeiae",
        constellation: "Cassiopeia",
        radius: Some(Length::new::<solar_radius>(45.39)),
        mass: Mass::new::<solar_mass>(3.98),
        absolute_magnitude: -1.99,
        apparent_magnitude: 2.24,
        temperature: ThermodynamicTemperature::new::<kelvin>(4552.),
        right_ascension: RightAscension::new(0, 40, 30.),
        declination: Declination::new(Sgn::Pos, 56, 32, 14.),
        distance: Length::new::<light_year>(228.),
        age: Some(Time::new::<gigayear>(0.19)),
        lifetime: Time::new::<gigayear>(0.193156929),
    }
}

fn caph() -> RealData {
    RealData {
        common_name: "Caph",
        astronomical_name: "β Cassiopeiae",
        constellation: "Cassiopeia",
        radius: Some(Length::new::<solar_radius>(3.5)),
        mass: Mass::new::<solar_mass>(1.91),
        absolute_magnitude: 1.17,
        apparent_magnitude: 2.28,
        temperature: ThermodynamicTemperature::new::<kelvin>(7079.),
        right_ascension: RightAscension::new(0, 9, 11.),
        declination: Declination::new(Sgn::Pos, 59, 8, 59.),
        distance: Length::new::<light_year>(54.),
        age: Some(Time::new::<gigayear>(1.1)),
        lifetime: Time::new::<gigayear>(1.54706939),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [navi(), schedar(), caph()]
}
