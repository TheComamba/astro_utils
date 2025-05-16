use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

fn ALPHA_TELESCOPII() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Telescopii",
        constellation: "Telescopium",
        right_ascension: RightAscension::new(18, 26, 58.),
        declination: Declination::new(Sgn::Neg, 45, 58, 6.),
        apparent_magnitude: 3.51,
        distance: Length::new::<light_year>(278.),
        absolute_magnitude: -1.25,
        mass: Mass::new::<solar_mass>(5.2),
        radius: Some(Length::new::<solar_radii>(3.3)),
        temperature: Temperature { K: 16_700. },
        age: Some(Time::new::<gigayear>(0.0241)),
        lifetime: Time::new::<gigayear>(0.10143918),
    }
}

fn ZETA_TELESCOPII() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Telescopii",
        constellation: "Telescopium",
        right_ascension: RightAscension::new(18, 28, 50.),
        declination: Declination::new(Sgn::Neg, 49, 4, 14.),
        apparent_magnitude: 4.13,
        distance: Length::new::<light_year>(126.),
        absolute_magnitude: 1.171,
        mass: Mass::new::<solar_mass>(1.53),
        radius: Some(Length::new::<solar_radii>(9.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4801.),
        age: None,
        lifetime: Time::new::<gigayear>(2.29668629),
    }
}

fn EPSILON_TELESCOPII() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Telescopii",
        constellation: "Telescopium",
        right_ascension: RightAscension::new(18, 11, 14.),
        declination: Declination::new(Sgn::Neg, 45, 57, 16.),
        apparent_magnitude: 4.50,
        distance: Length::new::<light_year>(410.),
        absolute_magnitude: -1.,
        mass: Mass::new::<solar_mass>(1.1),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(4996.),
        age: None,
        lifetime: Time::new::<gigayear>(6.97272616),
    }
}

pub(crate) const STARS: [RealData; 3] = [ALPHA_TELESCOPII, ZETA_TELESCOPII, EPSILON_TELESCOPII];
