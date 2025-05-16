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

fn ALPHA_CHAMAELEONTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Chamaeleontis",
        constellation: "Chamaeleon",
        right_ascension: RightAscension::new(8, 18, 32.),
        declination: Declination::new(Sgn::Neg, 76, 55, 11.),
        apparent_magnitude: 4.06,
        distance: Length::new::<light_year>(63.8),
        absolute_magnitude: 2.59,
        mass: Mass::new::<solar_mass>(1.42),
        radius: Some(Length::new::<solar_radii>(2.11)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6580.),
        age: Some(Time::new::<gigayear>(1.8)),
        lifetime: Time::new::<gigayear>(3.10253119),
    }
}

fn GAMMA_CHAMAELEONTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Chamaeleontis",
        constellation: "Chamaeleon",
        right_ascension: RightAscension::new(10, 35, 28.),
        declination: Declination::new(Sgn::Neg, 78, 36, 28.),
        apparent_magnitude: 4.12,
        distance: Length::new::<light_year>(418.),
        absolute_magnitude: -1.43,
        mass: Mass::new::<solar_mass>(2.4),
        radius: Some(Length::new::<solar_radii>(67.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4035.),
        age: None,
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

fn BETA_CHAMAELEONIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Chamaeleontis",
        constellation: "Chamaeleon",
        right_ascension: RightAscension::new(12, 18, 21.),
        declination: Declination::new(Sgn::Neg, 79, 18, 44.),
        apparent_magnitude: 4.24,
        distance: Length::new::<light_year>(298.),
        absolute_magnitude: -0.57,
        mass: Mass::new::<solar_mass>(5.9),
        radius: Some(Length::new::<solar_radii>(2.84)),
        temperature: ThermodynamicTemperature::new::<kelvin>(14_495.),
        age: Some(Time::new::<gigayear>(0.0227)),
        lifetime: Time::new::<gigayear>(0.078916095),
    }
}

pub(crate) const STARS: [RealData; 3] =
    [ALPHA_CHAMAELEONTIS, GAMMA_CHAMAELEONTIS, BETA_CHAMAELEONIS];
