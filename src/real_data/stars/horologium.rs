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

fn ALPHA_HOROLOGII() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Horologii",
        constellation: "Horologium",
        right_ascension: RightAscension::new(4, 14, 0.),
        declination: Declination::new(Sgn::Neg, 42, 17, 40.),
        apparent_magnitude: 3.853,
        distance: Length::new::<light_year>(115.),
        absolute_magnitude: 1.08,
        mass: Mass::new::<solar_mass>(1.55),
        radius: Some(Length::new::<solar_radii>(8.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5028.),
        age: None,
        lifetime: Time::new::<gigayear>(2.29668629),
    }
}

fn R_HOROLOGII() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "R Horologii",
        constellation: "Horologium",
        right_ascension: RightAscension::new(2, 53, 53.),
        declination: Declination::new(Sgn::Neg, 49, 53, 23.),
        apparent_magnitude: 7.22,
        distance: Length::new::<light_year>(1003.),
        absolute_magnitude: -0.221,
        mass: Mass::new::<solar_mass>(3.0),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(2200.),
        age: None,
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

fn BETA_HOROLOGII() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Horologii",
        constellation: "Horologium",
        right_ascension: RightAscension::new(2, 58, 48.),
        declination: Declination::new(Sgn::Neg, 64, 4, 17.),
        apparent_magnitude: 4.979,
        distance: Length::new::<light_year>(312.),
        absolute_magnitude: 0.2,
        mass: Mass::new::<solar_mass>(3.3),
        radius: Some(Length::new::<solar_radii>(1.4)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8303.),
        age: None,
        lifetime: Time::new::<gigayear>(0.351318702),
    }
}

pub(crate) fn STARS() -> [RealData; 3] {
    [ALPHA_HOROLOGII(), R_HOROLOGII(), BETA_HOROLOGII()]
}
