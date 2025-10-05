use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn alpheratz() -> RealData {
    RealData {
        common_name: "Alpheratz",
        astronomical_name: "α Andromedae",
        constellation: "Andromeda",
        radius: Some(Length::new::<solar_radii>(2.7)),
        mass: Mass::new::<solar_mass>(3.8),
        absolute_magnitude: -0.30,
        apparent_magnitude: 2.07,
        temperature: ThermodynamicTemperature::new::<kelvin>(13_800.),
        age: Some(Time::new::<gigayear>(0.06)),
        lifetime: Time::new::<gigayear>(0.220601963),
        right_ascension: RightAscension::new(0, 8, 23.),
        declination: Declination::new(Sgn::Pos, 29, 5, 26.),
        distance: Length::new::<light_year>(97.),
    }
}

fn mirach() -> RealData {
    RealData {
        common_name: "Mirach",
        astronomical_name: "β Andromedae",
        constellation: "Andromeda",
        radius: Some(Length::new::<solar_radii>(100.)),
        mass: Mass::new::<solar_mass>(2.49),
        absolute_magnitude: -1.86,
        apparent_magnitude: 2.07,
        temperature: ThermodynamicTemperature::new::<kelvin>(3842.),
        age: None,
        lifetime: Time::new::<gigayear>(0.800458342),
        right_ascension: RightAscension::new(1, 9, 44.),
        declination: Declination::new(Sgn::Pos, 35, 37, 14.),
        distance: Length::new::<light_year>(199.),
    }
}

fn almach() -> RealData {
    RealData {
        common_name: "Almach",
        astronomical_name: "γ Andromedae",
        constellation: "Andromeda",
        radius: Some(Length::new::<solar_radii>(80.)),
        mass: Mass::new::<solar_mass>(23.7),
        absolute_magnitude: -3.08,
        apparent_magnitude: 2.1,
        temperature: ThermodynamicTemperature::new::<kelvin>(4250.),
        age: Some(Time::new::<gigayear>(0.0065)),
        lifetime: Time::new::<gigayear>(0.008063854),
        right_ascension: RightAscension::new(2, 3, 54.),
        declination: Declination::new(Sgn::Pos, 42, 19, 47.),
        distance: Length::new::<light_year>(355.),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [alpheratz(), mirach(), almach()]
}
