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

fn BETA_CAMELOPARDALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Camelopardalis",
        constellation: "Camelopardalis",
        radius: Some(Length::new::<solar_radii>(58.)),
        mass: Mass::new::<solar_mass>(6.5),
        absolute_magnitude: -3.1,
        apparent_magnitude: 4.02,
        temperature: ThermodynamicTemperature::new::<kelvin>(5300.),
        right_ascension: RightAscension::new(5, 3, 25.),
        declination: Declination::new(Sgn::Pos, 60, 26, 32.),
        distance: Length::new::<light_year>(870.),
        age: Some(Time::new::<gigayear>(0.053)),
        lifetime: Time::new::<gigayear>(0.063411557),
    }
}

fn CS_CAMELOPARDALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "CS Camelopardalis",
        constellation: "Camelopardalis",
        radius: Some(Length::new::<solar_radii>(85.7)),
        mass: Mass::new::<solar_mass>(19.),
        absolute_magnitude: -6.39,
        apparent_magnitude: 4.21,
        temperature: ThermodynamicTemperature::new::<kelvin>(10_800.),
        right_ascension: RightAscension::new(3, 29, 4.),
        declination: Declination::new(Sgn::Pos, 59, 56, 25.),
        distance: Length::new::<light_year>(4289.),
        age: Some(Time::new::<gigayear>(0.011)),
        lifetime: Time::new::<gigayear>(0.011037517),
    }
}

fn ALPHA_CAMELOPARDALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Camelopardalis",
        constellation: "Camelopardalis",
        radius: Some(Length::new::<solar_radii>(32.5)),
        mass: Mass::new::<solar_mass>(37.6),
        absolute_magnitude: -7.1,
        apparent_magnitude: 4.29,
        temperature: ThermodynamicTemperature::new::<kelvin>(29_000.),
        right_ascension: RightAscension::new(4, 54, 3.),
        declination: Declination::new(Sgn::Pos, 66, 20, 34.),
        distance: Length {
            m: 6_000. * .m,
        },
        age: Some(Time::new::<gigayear>(0.002)),
        lifetime: Time::new::<gigayear>(0.005279908),
    }
}

pub(crate) const STARS: [RealData; 3] =
    [BETA_CAMELOPARDALIS, CS_CAMELOPARDALIS, ALPHA_CAMELOPARDALIS];
