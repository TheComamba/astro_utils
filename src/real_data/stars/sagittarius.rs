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

fn KAUS_AUSTRALIS() -> RealData {
    RealData {
        common_name: "Kaus Australis",
        astronomical_name: "ε Sagittarii",
        constellation: "Sagittarius",
        radius: Some(Length::new::<solar_radii>(6.8)),
        mass: Mass::new::<solar_mass>(3.515),
        absolute_magnitude: -1.44,
        apparent_magnitude: 1.79,
        temperature: ThermodynamicTemperature::new::<kelvin>(9960.),
        age: Some(Time::new::<gigayear>(0.232)),
        right_ascension: RightAscension::new(18, 24, 10.),
        declination: Declination::new(Sgn::Neg, 34, 23, 5.),
        distance: Length::new::<light_year>(145.),
        lifetime: Time::new::<gigayear>(0.254814649),
    }
}

fn NUNKI() -> RealData {
    RealData {
        common_name: "Nunki",
        astronomical_name: "σ Sagittarii",
        constellation: "Sagittarius",
        radius: Some(Length::new::<solar_radii>(4.5)),
        mass: Mass::new::<solar_mass>(7.8),
        absolute_magnitude: -2.14,
        apparent_magnitude: 2.05,
        temperature: ThermodynamicTemperature::new::<kelvin>(18_890.),
        age: Some(Time::new::<gigayear>(0.0314)),
        right_ascension: RightAscension::new(18, 55, 16.),
        declination: Declination::new(Sgn::Neg, 26, 17, 49.),
        distance: Length::new::<light_year>(224.),
        lifetime: Time::new::<gigayear>(0.040555762),
    }
}

fn NAMALWARID() -> RealData {
    RealData {
        common_name: "Namalwarid",
        astronomical_name: "η Sagittarii",
        constellation: "Sagittarius",
        radius: None,
        mass: Mass::new::<solar_mass>(1.2),
        absolute_magnitude: -0.201,
        apparent_magnitude: 3.1,
        temperature: ThermodynamicTemperature::new::<kelvin>(3300.),
        age: None,
        right_ascension: RightAscension::new(18, 17, 38.),
        declination: Declination::new(Sgn::Neg, 36, 45, 42.),
        distance: Length::new::<light_year>(149.1),
        lifetime: Time::new::<gigayear>(5.06543331),
    }
}

fn KAUS_MEDIA() -> RealData {
    RealData {
        common_name: "Kaus Media",
        astronomical_name: "δ Sagittarii",
        constellation: "Sagittarius",
        radius: Some(Length::new::<solar_radii>(16.)),
        mass: Mass::new::<solar_mass>(3.21),
        absolute_magnitude: -2.14,
        apparent_magnitude: 2.72,
        temperature: ThermodynamicTemperature::new::<kelvin>(4203.),
        age: Some(Time::new::<gigayear>(0.26)),
        right_ascension: RightAscension::new(18, 20, 60.),
        declination: Declination::new(Sgn::Neg, 29, 49, 41.),
        distance: Length::new::<light_year>(305.5),
        lifetime: Time::new::<gigayear>(0.351318702),
    }
}

pub(crate) fn STARS() -> [RealData; 4] { [KAUS_AUSTRALIS(), NUNKI(), NAMALWARID(), KAUS_MEDIA()] }
