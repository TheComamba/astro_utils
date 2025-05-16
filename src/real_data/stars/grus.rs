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

fn ALNAIR() -> RealData {
    RealData {
        common_name: "Alnair",
        astronomical_name: "α Gruis",
        constellation: "Grus",
        radius: Some(Length::new::<solar_radii>(3.4)),
        mass: Mass::new::<solar_mass>(4.),
        absolute_magnitude: -0.73,
        apparent_magnitude: 1.73,
        temperature: ThermodynamicTemperature::new::<kelvin>(13_920.),
        age: Some(Time::new::<gigayear>(0.1)),
        lifetime: Time::new::<gigayear>(0.193156929),
        right_ascension: RightAscension::new(22, 8, 14.),
        declination: Declination::new(Sgn::Neg, 46, 57, 40.),
        distance: Length::new::<light_year>(101.),
    }
}

fn TIAKI() -> RealData {
    RealData {
        common_name: "Tiaki",
        astronomical_name: "β Gruis",
        constellation: "Grus",
        radius: Some(Length::new::<solar_radii>(180.)),
        mass: Mass::new::<solar_mass>(2.4),
        absolute_magnitude: -1.52,
        apparent_magnitude: 2.07,
        temperature: ThermodynamicTemperature::new::<kelvin>(3480.),
        age: None,
        lifetime: Time::new::<gigayear>(0.800458342),
        right_ascension: RightAscension::new(22, 42, 40.),
        declination: Declination::new(Sgn::Neg, 46, 53, 4.),
        distance: Length::new::<light_year>(170.),
    }
}

fn ALDHANAB() -> RealData {
    RealData {
        common_name: "Aldhanab",
        astronomical_name: "γ Gruis",
        constellation: "Grus",
        right_ascension: RightAscension::new(21, 53, 56.),
        declination: Declination::new(Sgn::Neg, 37, 21, 53.),
        apparent_magnitude: 3.003,
        distance: Length::new::<light_year>(211.),
        absolute_magnitude: -1.05,
        mass: Mass::new::<solar_mass>(3.06),
        radius: Some(Length::new::<solar_radii>(4.5)),
        temperature: ThermodynamicTemperature::new::<kelvin>(12_520.),
        age: Some(Time::new::<gigayear>(0.075)),
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [ALNAIR(), TIAKI(), ALDHANAB()] }
