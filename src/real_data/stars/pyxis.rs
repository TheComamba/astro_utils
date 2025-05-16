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

fn ALPHA_PYXIDIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Pyxidis",
        constellation: "Pyxis",
        right_ascension: RightAscension::new(8, 43, 36.),
        declination: Declination::new(Sgn::Neg, 33, 11, 11.),
        apparent_magnitude: 3.67,
        distance: Length::new::<light_year>(880.),
        absolute_magnitude: -3.47,
        mass: Mass::new::<solar_mass>(10.7),
        radius: Some(Length::new::<solar_radii>(6.3)),
        temperature: ThermodynamicTemperature::new::<kelvin>(24_300.),
        age: Some(Time::new::<gigayear>(0.026)),
        lifetime: Time::new::<gigayear>(0.026540021),
    }
}

fn BETA_PYXIDIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Pyxidis",
        constellation: "Pyxis",
        right_ascension: RightAscension::new(8, 40, 6.),
        declination: Declination::new(Sgn::Neg, 35, 18, 30.),
        apparent_magnitude: 3.97,
        distance: Length::new::<light_year>(388.1),
        absolute_magnitude: -1.41,
        mass: Mass::new::<solar_mass>(1.2),
        radius: Some(Length::new::<solar_radii>(24.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5124.),
        age: None,
        lifetime: Time::new::<gigayear>(5.06543331),
    }
}

fn GAMMA_PYXIDIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Pyxidis",
        constellation: "Pyxis",
        right_ascension: RightAscension::new(8, 50, 32.),
        declination: Declination::new(Sgn::Neg, 27, 42, 35.),
        apparent_magnitude: 4.010,
        distance: Length::new::<light_year>(207.),
        absolute_magnitude: 0.,
        mass: Mass::new::<solar_mass>(1.64),
        radius: Some(Length::new::<solar_radii>(21.87)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4270.),
        age: Some(Time::new::<gigayear>(1.8)),
        lifetime: Time::new::<gigayear>(1.89665739),
    }
}

pub(crate) fn STARS() -> [RealData; 3] {
    [ALPHA_PYXIDIS(), BETA_PYXIDIS(), GAMMA_PYXIDIS()]
}
