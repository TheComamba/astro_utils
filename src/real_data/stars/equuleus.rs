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

fn KITALPHA() -> RealData {
    RealData {
        common_name: "Kitalpha",
        astronomical_name: "α Equulei",
        constellation: "Equuleus",
        right_ascension: RightAscension::new(21, 15, 49.),
        declination: Declination::new(Sgn::Pos, 5, 14, 52.),
        apparent_magnitude: 3.919,
        distance: Length::new::<light_year>(190.),
        absolute_magnitude: 0.17,
        mass: Mass::new::<solar_mass>(2.3),
        radius: Some(Length::new::<solar_radii>(9.2)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5100.),
        age: None,
        lifetime: Time::new::<gigayear>(0.916355612),
    }
}

fn DELTA_EQUULEI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Equulei",
        constellation: "Equuleus",
        right_ascension: RightAscension::new(21, 14, 29.),
        declination: Declination::new(Sgn::Pos, 10, 0, 25.),
        apparent_magnitude: 4.47,
        distance: Length::new::<light_year>(60.25),
        absolute_magnitude: 3.140,
        mass: Mass::new::<solar_mass>(1.192),
        radius: Some(Length::new::<solar_radii>(1.30)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6200.),
        age: Some(Time::new::<gigayear>(3.)),
        lifetime: Time::new::<gigayear>(5.06543331),
    }
}

fn GAMMA_EQUULEI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Equulei",
        constellation: "Equuleus",
        right_ascension: RightAscension::new(21, 10, 21.),
        declination: Declination::new(Sgn::Pos, 10, 7, 54.),
        apparent_magnitude: 4.6,
        distance: Length::new::<light_year>(118.),
        absolute_magnitude: 1.9,
        mass: Mass::new::<solar_mass>(1.78),
        radius: Some(Length::new::<solar_radii>(2.11)),
        temperature: ThermodynamicTemperature::new::<kelvin>(7550.),
        age: Some(Time::new::<gigayear>(1.)),
        lifetime: Time::new::<gigayear>(1.46605285),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [KITALPHA(), DELTA_EQUULEI(), GAMMA_EQUULEI()] }
