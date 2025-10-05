use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn diadem() -> RealData {
    RealData {
        common_name: "Diadem",
        astronomical_name: "α Comae Berenices",
        constellation: "Coma Berenices",
        right_ascension: RightAscension::new(13, 9, 59.),
        declination: Declination::new(Sgn::Pos, 17, 31, 46.),
        apparent_magnitude: 4.32,
        distance: Length::new::<light_year>(46.7),
        absolute_magnitude: 3.54,
        mass: Mass::new::<solar_mass>(1.237),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(6365.),
        age: None,
        lifetime: Time::new::<gigayear>(4.45521207),
    }
}

fn beta_coma_berenices() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Comae Berenices",
        constellation: "Coma Berenices",
        right_ascension: RightAscension::new(13, 11, 53.),
        declination: Declination::new(Sgn::Pos, 27, 52, 41.),
        apparent_magnitude: 4.26,
        distance: Length::new::<light_year>(29.95),
        absolute_magnitude: 4.46,
        mass: Mass::new::<solar_mass>(1.15),
        radius: Some(Length::new::<solar_radius>(1.106)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5936.),
        age: Some(Time::new::<gigayear>(2.)),
        lifetime: Time::new::<gigayear>(5.9461393),
    }
}

fn gamma_coma_berenices() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Comae Berenices",
        constellation: "Coma Berenices",
        right_ascension: RightAscension::new(12, 26, 56.),
        declination: Declination::new(Sgn::Pos, 28, 16, 6.),
        apparent_magnitude: 4.36,
        distance: Length::new::<light_year>(169.),
        absolute_magnitude: 0.76,
        mass: Mass::new::<solar_mass>(1.65),
        radius: Some(Length::new::<solar_radius>(11.76)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4652.),
        age: Some(Time::new::<gigayear>(1.8)),
        lifetime: Time::new::<gigayear>(1.89665739),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [diadem(), beta_coma_berenices(), gamma_coma_berenices()]
}
