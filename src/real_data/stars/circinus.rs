use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn alpha_circini() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Circini",
        constellation: "Circinus",
        right_ascension: RightAscension::new(14, 42, 30.),
        declination: Declination::new(Sgn::Neg, 64, 58, 30.),
        apparent_magnitude: 3.18,
        distance: Length::new::<light_year>(54.),
        absolute_magnitude: 2.18,
        mass: Mass::new::<solar_mass>(1.6),
        radius: Some(Length::new::<solar_radius>(1.967)),
        temperature: ThermodynamicTemperature::new::<kelvin>(7500.),
        age: Some(Time::new::<gigayear>(0.012)),
        lifetime: Time::new::<gigayear>(2.08398753),
    }
}

fn beta_circini() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Circini",
        constellation: "Circinus",
        right_ascension: RightAscension::new(15, 17, 31.),
        declination: Declination::new(Sgn::Neg, 58, 48, 4.),
        apparent_magnitude: 4.069,
        distance: Length::new::<light_year>(93.),
        absolute_magnitude: 1.64,
        mass: Mass::new::<solar_mass>(2.2),
        radius: Some(Length::new::<solar_radius>(1.3)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8676.),
        age: Some(Time::new::<gigayear>(0.4)),
        lifetime: Time::new::<gigayear>(1.03650581),
    }
}

fn gamma_circini() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Circini",
        constellation: "Circinus",
        right_ascension: RightAscension::new(15, 23, 23.),
        declination: Declination::new(Sgn::Neg, 59, 19, 15.),
        apparent_magnitude: 4.51,
        distance: Length::new::<light_year>(450.),
        absolute_magnitude: -1.18,
        mass: Mass::new::<solar_mass>(6.),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(15_135.),
        age: Some(Time::new::<gigayear>(0.0631)),
        lifetime: Time::new::<gigayear>(0.073299383),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [alpha_circini(), beta_circini(), gamma_circini()]
}
