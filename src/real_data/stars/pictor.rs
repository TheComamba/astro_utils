use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn alpha_pictoris() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Pictoris",
        constellation: "Pictor",
        right_ascension: RightAscension::new(6, 48, 11.),
        declination: Declination::new(Sgn::Neg, 61, 56, 29.),
        apparent_magnitude: 3.27,
        distance: Length::new::<light_year>(97.),
        absolute_magnitude: 0.86,
        mass: Mass::new::<solar_mass>(2.04),
        radius: Some(Length::new::<solar_radius>(1.6)),
        temperature: ThermodynamicTemperature::new::<kelvin>(7530.),
        age: Some(Time::new::<gigayear>(0.660)),
        lifetime: Time::new::<gigayear>(1.25731981),
    }
}

fn beta_pictoris() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Pictoris",
        constellation: "Pictor",
        right_ascension: RightAscension::new(5, 47, 17.),
        declination: Declination::new(Sgn::Neg, 51, 3, 59.),
        apparent_magnitude: 3.861,
        distance: Length::new::<light_year>(63.4),
        absolute_magnitude: 2.402,
        mass: Mass::new::<solar_mass>(1.75),
        radius: Some(Length::new::<solar_radius>(1.8)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8052.),
        age: Some(Time::new::<gigayear>(0.023)),
        lifetime: Time::new::<gigayear>(1.59501327),
    }
}

fn gamma_pictoris() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Pictoris",
        constellation: "Pictor",
        right_ascension: RightAscension::new(5, 49, 50.),
        declination: Declination::new(Sgn::Neg, 56, 9, 60.),
        apparent_magnitude: 4.50,
        distance: Length::new::<light_year>(177.),
        absolute_magnitude: 0.83,
        mass: Mass::new::<solar_mass>(1.59),
        radius: Some(Length::new::<solar_radius>(11.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4600.),
        age: None,
        lifetime: Time::new::<gigayear>(2.08398753),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [alpha_pictoris(), beta_pictoris(), gamma_pictoris()]
}
