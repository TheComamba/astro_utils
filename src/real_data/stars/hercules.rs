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

fn RASALGETHI() -> RealData {
    RealData {
        common_name: "Rasalgethi",
        astronomical_name: "α Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(17, 14, 39.),
        declination: Declination::new(Sgn::Pos, 14, 23, 25.),
        apparent_magnitude: 2.78,
        distance: Length::new::<light_year>(360.),
        absolute_magnitude: -2.57,
        mass: Mass::new::<solar_mass>(2.5),
        radius: Some(Length::new::<solar_radii>(284.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(3155.),
        age: None,
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

fn KORNEPHOROS() -> RealData {
    RealData {
        common_name: "Kornephoros",
        astronomical_name: "β Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(16, 30, 13.),
        declination: Declination::new(Sgn::Pos, 21, 29, 23.),
        apparent_magnitude: 2.81,
        distance: Length::new::<light_year>(139.),
        absolute_magnitude: -0.49,
        mass: Mass::new::<solar_mass>(2.9),
        radius: Some(Length::new::<solar_radii>(17.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4887.),
        age: None,
        lifetime: Time::new::<gigayear>(0.513076303),
    }
}

fn SARIN() -> RealData {
    RealData {
        common_name: "Sarin",
        astronomical_name: "δ Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(17, 15, 2.),
        declination: Declination::new(Sgn::Pos, 24, 50, 21.),
        apparent_magnitude: 3.126,
        distance: Length::new::<light_year>(75.1),
        absolute_magnitude: 1.31,
        mass: Mass::new::<solar_mass>(2.4),
        radius: Some(Length::new::<solar_radii>(2.2)),
        temperature: ThermodynamicTemperature::new::<kelvin>(9620.),
        age: Some(Time::new::<gigayear>(0.370)),
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

fn ETA_HERCULIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "η Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(16, 42, 54.),
        declination: Declination::new(Sgn::Pos, 38, 55, 20.),
        apparent_magnitude: 3.487,
        distance: Length::new::<light_year>(112.),
        absolute_magnitude: 0.84,
        mass: Mass::new::<solar_mass>(2.13),
        radius: Some(Length::new::<solar_radii>(8.9)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4900.),
        age: Some(Time::new::<gigayear>(1.)),
        lifetime: Time::new::<gigayear>(1.09929685),
    }
}

fn MU_HERCULIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "μ Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(17, 46, 28.),
        declination: Declination::new(Sgn::Pos, 27, 43, 14.),
        apparent_magnitude: 3.417,
        distance: Length::new::<light_year>(27.11),
        absolute_magnitude: 3.82,
        mass: Mass::new::<solar_mass>(1.11),
        radius: Some(Length::new::<solar_radii>(1.73)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5560.),
        age: Some(Time::new::<gigayear>(6.8)),
        lifetime: Time::new::<gigayear>(6.97272616),
    }
}

fn ZETA_HERCULIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(16, 41, 17.),
        declination: Declination::new(Sgn::Pos, 31, 36, 10.),
        apparent_magnitude: 2.81,
        distance: Length::new::<light_year>(35.),
        absolute_magnitude: 2.65,
        mass: Mass::new::<solar_mass>(1.45),
        radius: Some(Length::new::<solar_radii>(2.56)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5820.),
        age: Some(Time::new::<gigayear>(2.7)),
        lifetime: Time::new::<gigayear>(2.82957282),
    }
}

fn PI_HERCULIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "π Herculis",
        constellation: "Hercules",
        right_ascension: RightAscension::new(17, 15, 3.),
        declination: Declination::new(Sgn::Pos, 36, 48, 33.),
        apparent_magnitude: 3.15,
        distance: Length::new::<light_year>(377.),
        absolute_magnitude: -2.1,
        mass: Mass::new::<solar_mass>(4.),
        radius: Some(Length::new::<solar_radii>(72.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4170.),
        age: None,
        lifetime: Time::new::<gigayear>(0.193156929),
    }
}

pub(crate) fn stars() -> [RealData; 7] {
    [
        RASALGETHI(),
        KORNEPHOROS(),
        SARIN(),
        ETA_HERCULIS(),
        MU_HERCULIS(),
        ZETA_HERCULIS(),
        PI_HERCULIS(),
    ]
}
