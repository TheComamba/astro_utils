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

fn alpha_antliae() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Antliae",
        constellation: "Antlia",
        radius: Some(Length::new::<solar_radii>(41.)),
        mass: Mass::new::<solar_mass>(2.2),
        absolute_magnitude: -0.973,
        apparent_magnitude: 4.28,
        temperature: ThermodynamicTemperature::new::<kelvin>(4070.),
        age: None,
        lifetime: Time::new::<gigayear>(1.03650581),
        right_ascension: RightAscension::new(10, 27, 9.),
        declination: Declination::new(Sgn::Neg, 31, 4, 4.),
        distance: Length::new::<light_year>(366.3),
    }
}

fn epsilon_antliae() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Antliae",
        constellation: "Antlia",
        radius: Some(Length::new::<solar_radii>(56.3)),
        mass: Mass::new::<solar_mass>(2.),
        absolute_magnitude: -2.15,
        apparent_magnitude: 4.51,
        temperature: ThermodynamicTemperature::new::<kelvin>(4237.),
        age: None,
        lifetime: Time::new::<gigayear>(1.36020165),
        right_ascension: RightAscension::new(9, 29, 15.),
        declination: Declination::new(Sgn::Neg, 35, 57, 5.),
        distance: Length::new::<light_year>(699.6),
    }
}

fn iota_antliae() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ι Antliae",
        constellation: "Antlia",
        radius: Some(Length::new::<solar_radii>(12.1)),
        mass: Mass::new::<solar_mass>(1.55),
        absolute_magnitude: 0.674,
        apparent_magnitude: 4.60,
        temperature: ThermodynamicTemperature::new::<kelvin>(4892.),
        age: Some(Time::new::<gigayear>(2.2)),
        lifetime: Time::new::<gigayear>(2.29668629),
        right_ascension: RightAscension::new(10, 56, 43.),
        declination: Declination::new(Sgn::Neg, 37, 8, 16.),
        distance: Length::new::<light_year>(198.8),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [alpha_antliae(), epsilon_antliae(), iota_antliae()]
}
