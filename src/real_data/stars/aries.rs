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

fn hamal() -> RealData {
    RealData {
        common_name: "Hamal",
        astronomical_name: "α Arietis",
        constellation: "Aries",
        radius: Some(Length::new::<solar_radii>(14.9)),
        mass: Mass::new::<solar_mass>(1.5),
        absolute_magnitude: 0.48,
        apparent_magnitude: 2.01,
        temperature: ThermodynamicTemperature::new::<kelvin>(4480.),
        right_ascension: RightAscension::new(2, 7, 10.),
        declination: Declination::new(Sgn::Pos, 23, 27, 45.),
        distance: Length::new::<light_year>(66.),
        age: Some(Time::new::<gigayear>(2.5)),
        lifetime: Time::new::<gigayear>(2.54186931),
    }
}

fn beta_arietis() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Arietis",
        constellation: "Aries",
        right_ascension: RightAscension::new(1, 54, 38.),
        declination: Declination::new(Sgn::Pos, 20, 48, 29.),
        apparent_magnitude: 2.655,
        distance: Length::new::<light_year>(59.6),
        absolute_magnitude: 1.55,
        mass: Mass::new::<solar_mass>(2.34),
        radius: Some(Length::new::<solar_radii>(23.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(9000.),
        age: Some(Time::new::<gigayear>(0.3)),
        lifetime: Time::new::<gigayear>(0.916355612),
    }
}

fn bharani() -> RealData {
    RealData {
        common_name: "Bharani",
        astronomical_name: "41 Arietis",
        constellation: "Aries",
        right_ascension: RightAscension::new(2, 49, 59.),
        declination: Declination::new(Sgn::Pos, 27, 15, 38.),
        apparent_magnitude: 3.63,
        distance: Length::new::<light_year>(166.),
        absolute_magnitude: 0.163,
        mass: Mass::new::<solar_mass>(3.1),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(11_900.),
        age: Some(Time::new::<gigayear>(0.130)),
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [hamal(), beta_arietis(), bharani()]
}
