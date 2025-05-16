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

fn GHURAB() -> RealData {
    RealData {
        common_name: "Ghurab",
        astronomical_name: "γ Corvi",
        constellation: "Corvus",
        radius: None,
        mass: Mass::new::<solar_mass>(4.2),
        absolute_magnitude: -0.94,
        apparent_magnitude: 2.58,
        temperature: Temperature { K: 12_000. },
        age: Some(Time::new::<gigayear>(0.160)),
        right_ascension: RightAscension::new(12, 15, 48.),
        declination: Declination::new(Sgn::Neg, 17, 32, 31.),
        distance: Length::new::<light_year>(165.),
        lifetime: Time::new::<gigayear>(0.170765802),
    }
}

fn KRAZ() -> RealData {
    RealData {
        common_name: "Kraz",
        astronomical_name: "β Corvi",
        constellation: "Corvus",
        radius: Some(Length::new::<solar_radii>(16.)),
        mass: Mass::new::<solar_mass>(3.7),
        absolute_magnitude: -0.61,
        apparent_magnitude: 2.65,
        temperature: ThermodynamicTemperature::new::<kelvin>(5100.),
        age: Some(Time::new::<gigayear>(0.206)),
        right_ascension: RightAscension::new(12, 34, 23.),
        declination: Declination::new(Sgn::Neg, 23, 23, 48.),
        distance: Length::new::<light_year>(146.),
        lifetime: Time::new::<gigayear>(0.254814649),
    }
}

fn ALGORAB() -> RealData {
    RealData {
        common_name: "Algorab",
        astronomical_name: "δ Corvi",
        constellation: "Corvus",
        right_ascension: RightAscension::new(12, 29, 52.),
        declination: Declination::new(Sgn::Neg, 16, 30, 56.),
        apparent_magnitude: 2.94,
        distance: Length::new::<light_year>(87.85),
        absolute_magnitude: 0.787,
        mass: Mass::new::<solar_mass>(2.74),
        radius: None,
        temperature: Temperature { K: 10_400. },
        age: Some(Time::new::<gigayear>(0.260)),
        lifetime: Time::new::<gigayear>(0.513076303),
    }
}

fn EPSILON_CORVI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Corvi",
        constellation: "Corvus",
        right_ascension: RightAscension::new(12, 10, 7.),
        declination: Declination::new(Sgn::Neg, 22, 37, 11.),
        apparent_magnitude: 3.024,
        distance: Length::new::<light_year>(318.),
        absolute_magnitude: -1.82,
        mass: Mass::new::<solar_mass>(3.2),
        radius: Some(Length::new::<solar_radii>(52.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4320.),
        age: None,
        lifetime: Time::new::<gigayear>(0.351318702),
    }
}

pub(crate) const STARS: [RealData; 4] = [GHURAB, KRAZ, ALGORAB, EPSILON_CORVI];
