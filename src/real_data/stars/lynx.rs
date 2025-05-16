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

fn ALPHA_LYNCIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Lyncis",
        constellation: "Lynx",
        radius: Some(Length::new::<solar_radii>(54.5)),
        mass: Mass::new::<solar_mass>(2.),
        absolute_magnitude: -1.02,
        apparent_magnitude: 3.15,
        temperature: ThermodynamicTemperature::new::<kelvin>(3882.),
        right_ascension: RightAscension::new(9, 21, 3.),
        declination: Declination::new(Sgn::Pos, 34, 23, 33.),
        distance: Length::new::<light_year>(221.9),
        age: Some(Time::new::<gigayear>(1.3)),
        lifetime: Time::new::<gigayear>(1.36020165),
    }
}

fn THIRTYEIGHT_LYNCIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "38 Lyncis",
        constellation: "Lynx",
        right_ascension: RightAscension::new(9, 18, 51.),
        declination: Declination::new(Sgn::Pos, 36, 48, 9.),
        apparent_magnitude: 3.82,
        distance: Length::new::<light_year>(117.),
        absolute_magnitude: 0.98,
        mass: Mass::new::<solar_mass>(1.9),
        radius: Some(Length::new::<solar_radii>(3.07)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8862.),
        age: Some(Time::new::<gigayear>(0.213)),
        lifetime: Time::new::<gigayear>(1.54706939),
    }
}

fn THIRTYONE_LYNCIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "31 Lyncis",
        constellation: "Lynx",
        right_ascension: RightAscension::new(8, 22, 50.),
        declination: Declination::new(Sgn::Pos, 43, 11, 17.),
        apparent_magnitude: 4.25,
        distance: Length::new::<light_year>(380.),
        absolute_magnitude: -1.09,
        mass: Mass::new::<solar_mass>(1.95),
        radius: Some(Length::new::<solar_radii>(53.27)),
        temperature: ThermodynamicTemperature::new::<kelvin>(3921.),
        age: Some(Time::new::<gigayear>(1.32)),
        lifetime: Time::new::<gigayear>(1.46316038),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [ALPHA_LYNCIS, THIRTYEIGHT_LYNCIS, THIRTYONE_LYNCIS] }
