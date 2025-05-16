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

fn ALPHA_ARAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Arae",
        constellation: "Ara",
        right_ascension: RightAscension::new(17, 31, 50.),
        declination: Declination::new(Sgn::Neg, 49, 52, 34.),
        apparent_magnitude: 2.93,
        distance: Length::new::<light_year>(270.),
        absolute_magnitude: -1.72,
        mass: Mass::new::<solar_mass>(9.6),
        radius: Some(Length::new::<solar_radii>(4.5)),
        temperature: ThermodynamicTemperature::new::<kelvin>(18_044.),
        age: Some(Time::new::<gigayear>(0.026)),
        lifetime: Time::new::<gigayear>(0.026540021),
    }
}

fn BETA_ARAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Arae",
        constellation: "Ara",
        radius: Some(Length::new::<solar_radii>(142.)),
        mass: Mass::new::<solar_mass>(8.21),
        absolute_magnitude: -3.49,
        apparent_magnitude: 2.84,
        temperature: ThermodynamicTemperature::new::<kelvin>(4197.),
        right_ascension: RightAscension::new(17, 25, 18.),
        declination: Declination::new(Sgn::Neg, 55, 31, 48.),
        distance: Length::new::<light_year>(602.6),
        age: Some(Time::new::<gigayear>(0.035)),
        lifetime: Time::new::<gigayear>(0.040555762),
    }
}

fn GAMMA_ARAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Arae",
        constellation: "Ara",
        right_ascension: RightAscension::new(17, 25, 24.),
        declination: Declination::new(Sgn::Neg, 56, 22, 40.),
        apparent_magnitude: 3.31,
        distance: Length::new::<light_year>(1136.),
        absolute_magnitude: -4.4,
        mass: Mass::new::<solar_mass>(20.),
        radius: Some(Length::new::<solar_radii>(23.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(21_500.),
        age: Some(Time::new::<gigayear>(0.009)),
        lifetime: Time::new::<gigayear>(0.009767659),
    }
}

fn DELTA_ARAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Arae",
        constellation: "Ara",
        right_ascension: RightAscension::new(17, 31, 6.),
        declination: Declination::new(Sgn::Neg, 60, 41, 2.),
        apparent_magnitude: 3.62,
        distance: Length::new::<light_year>(198.),
        absolute_magnitude: -0.31,
        mass: Mass::new::<solar_mass>(3.56),
        radius: Some(Length::new::<solar_radii>(3.12)),
        temperature: ThermodynamicTemperature::new::<kelvin>(11_962.),
        age: Some(Time::new::<gigayear>(0.125)),
        lifetime: Time::new::<gigayear>(0.254814649),
    }
}

fn ZETA_ARAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Arae",
        constellation: "Ara",
        right_ascension: RightAscension::new(16, 58, 37.),
        declination: Declination::new(Sgn::Neg, 55, 59, 25.),
        apparent_magnitude: 3.12,
        distance: Length::new::<light_year>(573.9),
        absolute_magnitude: -3.11,
        mass: Mass::new::<solar_mass>(7.5),
        radius: Some(Length::new::<solar_radii>(114.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4246.),
        age: Some(Time::new::<gigayear>(0.045)),
        lifetime: Time::new::<gigayear>(0.052267043),
    }
}

pub(crate) fn STARS() -> [RealData; 5] { [ALPHA_ARAE, BETA_ARAE, GAMMA_ARAE, DELTA_ARAE, ZETA_ARAE] }
