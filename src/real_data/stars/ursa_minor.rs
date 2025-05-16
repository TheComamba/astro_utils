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

fn POLARIS() -> RealData {
    RealData {
        common_name: "Polaris",
        astronomical_name: "α Ursae Minoris",
        constellation: "Ursa Minor",
        radius: Some(Length::new::<solar_radii>(37.5)),
        mass: Mass::new::<solar_mass>(5.4),
        absolute_magnitude: -3.64,
        apparent_magnitude: 1.97,
        temperature: ThermodynamicTemperature::new::<kelvin>(6015.),
        age: Some(Time::new::<gigayear>(0.05)),
        right_ascension: RightAscension::new(2, 31, 49.),
        declination: Declination::new(Sgn::Pos, 89, 15, 51.),
        distance: Length::new::<light_year>(431.),
        lifetime: Time::new::<gigayear>(0.093024309),
    }
}

fn KOCHAB() -> RealData {
    RealData {
        common_name: "Kochab",
        astronomical_name: "β Ursae Minoris",
        constellation: "Ursa Minor",
        radius: Some(Length::new::<solar_radii>(42.06)),
        mass: Mass::new::<solar_mass>(2.2),
        absolute_magnitude: -0.87,
        apparent_magnitude: 2.07,
        temperature: ThermodynamicTemperature::new::<kelvin>(4030.),
        age: None,
        right_ascension: RightAscension::new(14, 50, 42.),
        declination: Declination::new(Sgn::Pos, 74, 9, 20.),
        distance: Length::new::<light_year>(126.),
        lifetime: Time::new::<gigayear>(1.03650581),
    }
}

fn ZETA_URSAE_MINORIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Ursae Minoris",
        constellation: "Ursa Minor",
        right_ascension: RightAscension::new(15, 44, 4.),
        declination: Declination::new(Sgn::Pos, 77, 47, 40.),
        apparent_magnitude: 4.29,
        distance: Length::new::<light_year>(369.),
        absolute_magnitude: -0.98,
        mass: Mass::new::<solar_mass>(3.4),
        radius: Some(Length::new::<solar_radii>(6.15)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8720.),
        age: Some(Time::new::<gigayear>(0.180)),
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

fn DELTA_URSAE_MINORIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Ursae Minoris",
        constellation: "Ursa Minor",
        right_ascension: RightAscension::new(17, 32, 13.),
        declination: Declination::new(Sgn::Pos, 86, 35, 11.),
        apparent_magnitude: 4.36,
        distance: Length::new::<light_year>(172.),
        absolute_magnitude: 0.62,
        mass: Mass::new::<solar_mass>(2.35),
        radius: Some(Length::new::<solar_radii>(2.8)),
        temperature: ThermodynamicTemperature::new::<kelvin>(9911.),
        age: Some(Time::new::<gigayear>(0.327)),
        lifetime: Time::new::<gigayear>(0.916355612),
    }
}

fn ETA_URSAE_MINORIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "η Ursae Minoris",
        constellation: "Ursa Minor",
        right_ascension: RightAscension::new(16, 17, 30.),
        declination: Declination::new(Sgn::Pos, 75, 45, 19.),
        apparent_magnitude: 4.95,
        distance: Length::new::<light_year>(97.6),
        absolute_magnitude: 2.61,
        mass: Mass::new::<solar_mass>(1.35),
        radius: Some(Length::new::<solar_radii>(2.0)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6858.),
        age: Some(Time::new::<gigayear>(1.061)),
        lifetime: Time::new::<gigayear>(3.46068223),
    }
}

fn PHERKAD() -> RealData {
    RealData {
        common_name: "Pherkad",
        astronomical_name: "γ Ursae Minoris",
        constellation: "Ursa Minor",
        right_ascension: RightAscension::new(15, 20, 44.),
        declination: Declination::new(Sgn::Pos, 71, 50, 2.),
        apparent_magnitude: 3.05,
        distance: Length::new::<light_year>(487.),
        absolute_magnitude: -2.84,
        mass: Mass::new::<solar_mass>(9.),
        radius: Some(Length::new::<solar_radii>(15.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8280.),
        age: Some(Time::new::<gigayear>(0.032)),
        lifetime: Time::new::<gigayear>(0.03224554),
    }
}

fn EPSILON_URSAE_MINORIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Ursae Minoris",
        constellation: "Ursa Minor",
        right_ascension: RightAscension::new(16, 45, 58.),
        declination: Declination::new(Sgn::Pos, 82, 2, 14.),
        apparent_magnitude: 4.19,
        distance: Length::new::<light_year>(300.),
        absolute_magnitude: -0.922,
        mass: Mass::new::<solar_mass>(1.1),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(5215.),
        age: None,
        lifetime: Time::new::<gigayear>(6.97272616),
    }
}

pub(crate) fn STARS() -> [RealData; 7] {
    [
        POLARIS(),
        KOCHAB(),
        ZETA_URSAE_MINORIS(),
        DELTA_URSAE_MINORIS(),
        ETA_URSAE_MINORIS(),
        PHERKAD(),
        EPSILON_URSAE_MINORIS(),
    ]
}
