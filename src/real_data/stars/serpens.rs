use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn unukalhai() -> RealData {
    RealData {
        common_name: "Unukalhai",
        astronomical_name: "α Serpentis",
        constellation: "Serpens",
        radius: Some(Length::new::<solar_radius>(13.48)),
        mass: Mass::new::<solar_mass>(1.66),
        absolute_magnitude: 0.88,
        apparent_magnitude: 2.63,
        temperature: ThermodynamicTemperature::new::<kelvin>(4498.),
        age: None,
        right_ascension: RightAscension::new(15, 44, 16.),
        declination: Declination::new(Sgn::Pos, 6, 25, 32.),
        distance: Length::new::<light_year>(74.),
        lifetime: Time::new::<gigayear>(1.89665739),
    }
}

fn beta_serpentis() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Serpentis",
        constellation: "Serpens",
        right_ascension: RightAscension::new(15, 46, 11.),
        declination: Declination::new(Sgn::Pos, 15, 25, 19.),
        apparent_magnitude: 3.65,
        distance: Length::new::<light_year>(155.),
        absolute_magnitude: 0.30,
        mass: Mass::new::<solar_mass>(1.94),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(8928.),
        age: Some(Time::new::<gigayear>(0.267)),
        lifetime: Time::new::<gigayear>(1.46316038),
    }
}

fn mu_serpentis() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "μ Serpentis",
        constellation: "Serpens",
        right_ascension: RightAscension::new(15, 49, 37.),
        declination: Declination::new(Sgn::Neg, 3, 25, 49.),
        apparent_magnitude: 3.543,
        distance: Length::new::<light_year>(170.),
        absolute_magnitude: -0.04,
        mass: Mass::new::<solar_mass>(2.4),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(9487.),
        age: None,
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [unukalhai(), beta_serpentis(), mu_serpentis()]
}
