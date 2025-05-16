use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

fn ALTAIR() -> RealData {
    RealData {
        common_name: "Altair",
        astronomical_name: "α Aquilae",
        constellation: "Aquila",
        radius: Some(Length::new::<solar_radii>(1.63)),
        mass: Mass::new::<solar_mass>(1.86),
        absolute_magnitude: 2.20,
        apparent_magnitude: 0.76,
        temperature: ThermodynamicTemperature::new::<kelvin>(7670.),
        right_ascension: RightAscension::new(19, 50, 47.),
        declination: Declination::new(Sgn::Pos, 8, 52, 6.),
        distance: Length::new::<light_year>(17.),
        age: Some(Time::new::<gigayear>(0.100)),
        lifetime: Time::new::<gigayear>(1.65092742),
    }
}

fn TARAZED() -> RealData {
    RealData {
        common_name: "Tarazed",
        astronomical_name: "γ Aquilae",
        constellation: "Aquila",
        radius: Some(Length::new::<solar_radii>(91.82)),
        mass: Mass::new::<solar_mass>(3.51),
        absolute_magnitude: -3.03,
        apparent_magnitude: 2.72,
        temperature: ThermodynamicTemperature::new::<kelvin>(4098.),
        right_ascension: RightAscension::new(19, 46, 16.),
        declination: Declination::new(Sgn::Pos, 10, 36, 48.),
        distance: Length::new::<light_year>(460.5),
        age: Some(Time::new::<gigayear>(0.250)),
        lifetime: Time::new::<gigayear>(0.254814649),
    }
}

fn OKAB() -> RealData {
    RealData {
        common_name: "Okab",
        astronomical_name: "ζ Aquilae",
        constellation: "Aquila",
        right_ascension: RightAscension::new(19, 5, 25.),
        declination: Declination::new(Sgn::Pos, 13, 51, 49.),
        apparent_magnitude: 2.983,
        distance: Length::new::<light_year>(83.0),
        absolute_magnitude: 0.96,
        mass: Mass::new::<solar_mass>(2.37),
        radius: Some(Length::new::<solar_radii>(2.27)),
        temperature: ThermodynamicTemperature::new::<kelvin>(9620.),
        age: Some(Time::new::<gigayear>(0.1)),
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

fn THETA_AQUILAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "θ Aquilae",
        constellation: "Aquila",
        right_ascension: RightAscension::new(20, 11, 18.),
        declination: Declination::new(Sgn::Neg, 0, 49, 17.),
        apparent_magnitude: 3.26,
        distance: Length::new::<light_year>(286.),
        absolute_magnitude: -1.39,
        mass: Mass::new::<solar_mass>(3.564),
        radius: Some(Length::new::<solar_radii>(4.76)),
        temperature: Temperature { K: 10_300. },
        age: Some(Time::new::<gigayear>(0.0209)),
        lifetime: Time::new::<gigayear>(0.254814649),
    }
}

fn DELTA_AQUILAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Aquilae",
        constellation: "Aquila",
        right_ascension: RightAscension::new(19, 25, 30.),
        declination: Declination::new(Sgn::Pos, 3, 6, 53.),
        apparent_magnitude: 3.365,
        distance: Length::new::<light_year>(50.6),
        absolute_magnitude: 2.46,
        mass: Mass::new::<solar_mass>(1.65),
        radius: Some(Length::new::<solar_radii>(2.04)),
        temperature: ThermodynamicTemperature::new::<kelvin>(7016.),
        age: None,
        lifetime: Time::new::<gigayear>(1.89665739),
    }
}

fn LAMBDA_AQUILAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "λ Aquilae",
        constellation: "Aquila",
        right_ascension: RightAscension::new(19, 6, 15.),
        declination: Declination::new(Sgn::Neg, 4, 52, 57.),
        apparent_magnitude: 3.43,
        distance: Length::new::<light_year>(125.),
        absolute_magnitude: 0.54,
        mass: Mass::new::<solar_mass>(3.1),
        radius: Some(Length::new::<solar_radii>(1.9)),
        temperature: Temperature { K: 11_780. },
        age: Some(Time::new::<gigayear>(0.160)),
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

pub(crate) const STARS: [RealData; 6] = [
    ALTAIR,
    TARAZED,
    OKAB,
    THETA_AQUILAE,
    DELTA_AQUILAE,
    LAMBDA_AQUILAE,
];
