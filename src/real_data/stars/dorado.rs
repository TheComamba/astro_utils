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

fn ALPHA_DORADUS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Doradus",
        constellation: "Dorado",
        right_ascension: RightAscension::new(4, 33, 60.),
        declination: Declination::new(Sgn::Neg, 55, 2, 42.),
        apparent_magnitude: 3.27,
        distance: Length::new::<light_year>(169.),
        absolute_magnitude: -0.357,
        mass: Mass::new::<solar_mass>(3.33),
        radius: Some(Length::new::<solar_radii>(3.5)),
        temperature: ThermodynamicTemperature::new::<kelvin>(11_588.),
        age: None,
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

fn BETA_DORADUS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Doradus",
        constellation: "Dorado",
        right_ascension: RightAscension::new(5, 33, 38.),
        declination: Declination::new(Sgn::Neg, 62, 29, 23.),
        apparent_magnitude: 3.5,
        distance: Length::new::<light_year>(1050.),
        absolute_magnitude: -3.91,
        mass: Mass::new::<solar_mass>(7.7),
        radius: Some(Length::new::<solar_radii>(67.8)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5445.),
        age: Some(Time::new::<gigayear>(0.04)),
        lifetime: Time::new::<gigayear>(0.040555762),
    }
}

fn GAMMA_DORADUS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Doradus",
        constellation: "Dorado",
        right_ascension: RightAscension::new(4, 16, 2.),
        declination: Declination::new(Sgn::Neg, 51, 29, 12.),
        apparent_magnitude: 4.25,
        distance: Length::new::<light_year>(66.7),
        absolute_magnitude: 2.72,
        mass: Mass::new::<solar_mass>(1.56),
        radius: Some(Length::new::<solar_radii>(1.85)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6906.),
        age: Some(Time::new::<gigayear>(0.535)),
        lifetime: Time::new::<gigayear>(2.29668629),
    }
}

fn R_DORADUS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "R Doradus",
        constellation: "",
        radius: Some(Length::new::<solar_radii>(298.)),
        mass: Mass::new::<solar_mass>(0.8),
        absolute_magnitude: 1.61,
        apparent_magnitude: 5.59,
        temperature: ThermodynamicTemperature::new::<kelvin>(2710.),
        age: Some(Time::new::<gigayear>(4.)),
        lifetime: Time::new::<gigayear>(21.4199307),
        right_ascension: RightAscension::new(4, 36, 46.),
        declination: Declination::new(Sgn::Neg, 62, 4, 38.),
        distance: Length::new::<light_year>(203.5),
    }
}

pub(crate) fn stars() -> [RealData; 4] {
    [
        ALPHA_DORADUS(),
        BETA_DORADUS(),
        GAMMA_DORADUS(),
        R_DORADUS(),
    ]
}
