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

fn BETA_TRIANGULI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Trianguli",
        constellation: "Triangulum",
        right_ascension: RightAscension::new(2, 9, 33.),
        declination: Declination::new(Sgn::Pos, 34, 59, 14.),
        apparent_magnitude: 3.,
        distance: Length::new::<light_year>(127.),
        absolute_magnitude: 0.05,
        mass: Mass::new::<solar_mass>(3.5),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(8186.),
        age: Some(Time::new::<gigayear>(0.29)),
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

fn ALPHA_TRIANGULI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Trianguli",
        constellation: "Triangulum",
        right_ascension: RightAscension::new(1, 53, 5.),
        declination: Declination::new(Sgn::Pos, 29, 34, 44.),
        apparent_magnitude: 3.42,
        distance: Length::new::<light_year>(63.3),
        absolute_magnitude: 1.98,
        mass: Mass::new::<solar_mass>(1.70),
        radius: Some(Length::new::<solar_radii>(3.22)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6288.),
        age: Some(Time::new::<gigayear>(1.6)),
        lifetime: Time::new::<gigayear>(1.73766023),
    }
}

fn GAMMA_TRIANGULI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Trianguli",
        constellation: "Triangulum",
        right_ascension: RightAscension::new(2, 17, 19.),
        declination: Declination::new(Sgn::Pos, 33, 50, 50.),
        apparent_magnitude: 4.01,
        distance: Length::new::<light_year>(112.3),
        absolute_magnitude: 1.35,
        mass: Mass::new::<solar_mass>(2.7),
        radius: Some(Length::new::<solar_radii>(1.96)),
        temperature: ThermodynamicTemperature::new::<kelvin>(9440.),
        age: Some(Time::new::<gigayear>(0.3)),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [BETA_TRIANGULI(), ALPHA_TRIANGULI(), GAMMA_TRIANGULI()]
}
