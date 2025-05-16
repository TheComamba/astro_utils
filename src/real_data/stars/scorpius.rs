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

fn ANTARES() -> RealData {
    RealData {
        common_name: "Antares",
        astronomical_name: "α Scorpii",
        constellation: "Scorpius",
        radius: Some(Length::new::<solar_radii>(680.)),
        mass: Mass::new::<solar_mass>(13.5),
        absolute_magnitude: -5.28,
        apparent_magnitude: 1.06,
        temperature: ThermodynamicTemperature::new::<kelvin>(3660.),
        age: Some(Time::new::<gigayear>(0.015)),
        right_ascension: RightAscension::new(16, 29, 24.),
        declination: Declination::new(Sgn::Neg, 26, 25, 55.),
        distance: Length::new::<light_year>(604.),
        lifetime: Time::new::<gigayear>(0.015362858),
    }
}

fn SHAULA() -> RealData {
    RealData {
        common_name: "Shaula",
        astronomical_name: "λ Scorpii",
        constellation: "Scorpius",
        radius: Some(Length::new::<solar_radii>(8.8)),
        mass: Mass::new::<solar_mass>(10.4),
        absolute_magnitude: -4.8,
        apparent_magnitude: 1.62,
        temperature: Temperature { K: 25_000. },
        right_ascension: RightAscension::new(17, 33, 37.),
        declination: Declination::new(Sgn::Neg, 37, 6, 14.),
        distance: Length::new::<light_year>(600.),
        age: Some(Time::new::<gigayear>(0.026)),
        lifetime: Time::new::<gigayear>(0.026540021),
    }
}

fn SARGAS() -> RealData {
    RealData {
        common_name: "Sargas",
        astronomical_name: "θ Scorpii",
        constellation: "Scorpius",
        radius: Some(Length::new::<solar_radii>(26.3)),
        mass: Mass::new::<solar_mass>(3.1),
        absolute_magnitude: -2.75,
        apparent_magnitude: 1.86,
        temperature: ThermodynamicTemperature::new::<kelvin>(6294.),
        age: None,
        right_ascension: RightAscension::new(17, 37, 19.),
        declination: Declination::new(Sgn::Neg, 42, 59, 52.),
        distance: Length::new::<light_year>(272.),
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

fn DSCHUBBA() -> RealData {
    RealData {
        common_name: "Dschubba",
        astronomical_name: "δ Scorpii",
        constellation: "Scorpius",
        radius: Some(Length::new::<solar_radii>(6.7)),
        mass: Mass::new::<solar_mass>(13.),
        absolute_magnitude: -3.16,
        apparent_magnitude: 2.29,
        temperature: Temperature { K: 27_400. },
        age: Some(Time::new::<gigayear>(0.0095)),
        right_ascension: RightAscension::new(16, 0, 20.),
        declination: Declination::new(Sgn::Neg, 22, 37, 18.),
        distance: Length::new::<light_year>(401.5),
        lifetime: Time::new::<gigayear>(0.019450199),
    }
}

fn LARAWAG() -> RealData {
    RealData {
        common_name: "Larawag",
        astronomical_name: "ε Scorpii",
        constellation: "Scorpius",
        radius: Some(Length::new::<solar_radii>(12.6)),
        mass: Mass::new::<solar_mass>(1.24),
        absolute_magnitude: 0.78,
        apparent_magnitude: 2.29,
        temperature: ThermodynamicTemperature::new::<kelvin>(4560.),
        age: None,
        right_ascension: RightAscension::new(16, 50, 10.),
        declination: Declination::new(Sgn::Neg, 34, 17, 36.),
        distance: Length::new::<light_year>(65.),
        lifetime: Time::new::<gigayear>(4.45521207),
    }
}

fn GIRTAB() -> RealData {
    RealData {
        common_name: "Girtab",
        astronomical_name: "κ Scorpii",
        constellation: "Scorpius",
        radius: Some(Length::new::<solar_radii>(6.8)),
        mass: Mass::new::<solar_mass>(17.),
        absolute_magnitude: -3.38,
        apparent_magnitude: 2.39,
        temperature: Temperature { K: 23_400. },
        right_ascension: RightAscension::new(17, 42, 29.),
        declination: Declination::new(Sgn::Neg, 39, 1, 48.),
        distance: Length::new::<light_year>(464.),
        age: Some(Time::new::<gigayear>(0.012)),
        lifetime: Time::new::<gigayear>(0.012799766),
    }
}

fn ACRAB() -> RealData {
    RealData {
        common_name: "Acrab",
        astronomical_name: "β Scorpii",
        constellation: "Scorpius",
        radius: Some(Length::new::<solar_radii>(6.3)),
        mass: Mass::new::<solar_mass>(15.0),
        absolute_magnitude: -3.50,
        apparent_magnitude: 2.56,
        temperature: Temperature { K: 28_000. },
        right_ascension: RightAscension::new(16, 5, 26.),
        declination: Declination::new(Sgn::Neg, 19, 48, 20.),
        distance: Length::new::<light_year>(530.),
        age: Some(Time::new::<gigayear>(0.015)),
        lifetime: Time::new::<gigayear>(0.015362858),
    }
}

pub(crate) const STARS: [RealData; 7] = [ANTARES, SHAULA, SARGAS, DSCHUBBA, LARAWAG, GIRTAB, ACRAB];
