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

fn CAPELLA() -> RealData {
    RealData {
        common_name: "Capella",
        astronomical_name: "α Aurigae",
        constellation: "Auriga",
        radius: Some(Length::new::<solar_radii>(11.98)),
        mass: Mass::new::<solar_mass>(2.5687),
        absolute_magnitude: -0.48,
        apparent_magnitude: 0.08,
        temperature: ThermodynamicTemperature::new::<kelvin>(4970.),
        age: Some(Time::new::<gigayear>(0.620)),
        right_ascension: RightAscension::new(5, 16, 41.),
        declination: Declination::new(Sgn::Pos, 45, 59, 53.),
        distance: Length::new::<light_year>(42.),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn MENKALINAN() -> RealData {
    RealData {
        common_name: "Menkalinan",
        astronomical_name: "β Aurigae",
        constellation: "Auriga",
        radius: Some(Length::new::<solar_radii>(2.77)),
        mass: Mass::new::<solar_mass>(2.389),
        absolute_magnitude: -0.10,
        apparent_magnitude: 1.9,
        temperature: ThermodynamicTemperature::new::<kelvin>(9350.),
        right_ascension: RightAscension::new(5, 59, 32.),
        declination: Declination::new(Sgn::Pos, 44, 56, 51.),
        distance: Length::new::<light_year>(82.),
        age: Some(Time::new::<gigayear>(0.570)),
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

fn HASSALEH() -> RealData {
    RealData {
        common_name: "Hassaleh",
        astronomical_name: "ι Aurigae",
        constellation: "Auriga",
        radius: Some(Length::new::<solar_radii>(127.)),
        mass: Mass::new::<solar_mass>(7.1),
        absolute_magnitude: -3.20,
        apparent_magnitude: 2.69,
        temperature: ThermodynamicTemperature::new::<kelvin>(4160.),
        right_ascension: RightAscension::new(4, 56, 60.),
        declination: Declination::new(Sgn::Pos, 33, 9, 58.),
        distance: Length::new::<light_year>(490.),
        age: Some(Time::new::<gigayear>(0.04)),
        lifetime: Time::new::<gigayear>(0.052267043),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [CAPELLA, MENKALINAN, HASSALEH] }
