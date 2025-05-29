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

fn dabih() -> RealData {
    RealData {
        common_name: "Dabih",
        astronomical_name: "β Capricorni",
        constellation: "Capricornus",
        right_ascension: RightAscension::new(20, 21, 1.),
        declination: Declination::new(Sgn::Neg, 14, 46, 53.),
        apparent_magnitude: 3.05,
        distance: Length::new::<light_year>(555.4),
        absolute_magnitude: -3., // Not literature value
        mass: Mass::new::<solar_mass>(3.9),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(4900.),
        age: None,
        lifetime: Time::new::<gigayear>(0.220601963),
    }
}

fn deneb_algedi() -> RealData {
    RealData {
        common_name: "Deneb Algedi",
        astronomical_name: "δ Capricorni",
        constellation: "Capricornus",
        right_ascension: RightAscension::new(21, 47, 2.),
        declination: Declination::new(Sgn::Neg, 16, 7, 38.),
        apparent_magnitude: 2.81,
        distance: Length::new::<light_year>(38.7),
        absolute_magnitude: 2.48,
        mass: Mass::new::<solar_mass>(2.),
        radius: Some(Length::new::<solar_radii>(1.91)),
        temperature: ThermodynamicTemperature::new::<kelvin>(7301.),
        age: None,
        lifetime: Time::new::<gigayear>(1.36020165),
    }
}

fn omega_capricorni() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ω Capricorni",
        constellation: "Capricornus",
        right_ascension: RightAscension::new(20, 51, 49.),
        declination: Declination::new(Sgn::Neg, 26, 55, 9.),
        apparent_magnitude: 4.12,
        distance: Length::new::<light_year>(628.1),
        absolute_magnitude: -2.3,
        mass: Mass::new::<solar_mass>(6.8),
        radius: Some(Length::new::<solar_radii>(172.1)),
        temperature: ThermodynamicTemperature::new::<kelvin>(3915.),
        age: Some(Time::new::<gigayear>(0.0481)),
        lifetime: Time::new::<gigayear>(0.052267043),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [dabih(), deneb_algedi(), omega_capricorni()]
}
