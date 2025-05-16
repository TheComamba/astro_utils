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

fn ENIF() -> RealData {
    RealData {
        common_name: "Enif",
        astronomical_name: "ε Pegasi",
        constellation: "Pegasus",
        radius: Some(Length::new::<solar_radii>(211.)),
        mass: Mass::new::<solar_mass>(7.07),
        absolute_magnitude: -4.19,
        apparent_magnitude: 2.38,
        temperature: ThermodynamicTemperature::new::<kelvin>(3963.),
        age: Some(Time::new::<gigayear>(0.020)),
        right_ascension: RightAscension::new(21, 44, 11.),
        declination: Declination::new(Sgn::Pos, 9, 52, 30.),
        distance: Length::new::<light_year>(672.),
        lifetime: Time::new::<gigayear>(0.052267043),
    }
}

fn SCHEAT() -> RealData {
    RealData {
        common_name: "Scheat",
        astronomical_name: "β Pegasi",
        constellation: "Pegasus",
        radius: Some(Length::new::<solar_radii>(95.)),
        mass: Mass::new::<solar_mass>(2.1),
        absolute_magnitude: -1.49,
        apparent_magnitude: 2.44,
        temperature: ThermodynamicTemperature::new::<kelvin>(3689.),
        age: None,
        right_ascension: RightAscension::new(23, 3, 46.),
        declination: Declination::new(Sgn::Pos, 28, 4, 58.),
        distance: Length::new::<light_year>(199.),
        lifetime: Time::new::<gigayear>(1.17901142),
    }
}

fn MARKAB() -> RealData {
    RealData {
        common_name: "Markab",
        astronomical_name: "α Pegasi",
        constellation: "Pegasus",
        radius: Some(Length::new::<solar_radii>(4.62)),
        mass: Mass::new::<solar_mass>(3.5),
        absolute_magnitude: -0.67,
        apparent_magnitude: 2.49,
        temperature: ThermodynamicTemperature::new::<kelvin>(10_100.),
        age: Some(Time::new::<gigayear>(0.2)),
        right_ascension: RightAscension::new(23, 4, 46.),
        declination: Declination::new(Sgn::Pos, 15, 12, 19.),
        distance: Length::new::<light_year>(140.),
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [ENIF(), SCHEAT(), MARKAB()] }
