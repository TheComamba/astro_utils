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

fn MERIDIANA() -> RealData {
    RealData {
        common_name: "Meridiana",
        astronomical_name: "α Coronae Australis",
        constellation: "Corona Australis",
        right_ascension: RightAscension::new(19, 9, 28.),
        declination: Declination::new(Sgn::Neg, 37, 54, 16.),
        apparent_magnitude: 4.102,
        distance: Length::new::<light_year>(125.),
        absolute_magnitude: 1.11,
        mass: Mass::new::<solar_mass>(2.57),
        radius: Some(Length::new::<solar_radii>(2.21)),
        temperature: ThermodynamicTemperature::new::<kelvin>(9916.),
        age: Some(Time::new::<gigayear>(0.254)),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn BETA_CORONAE_AUSTRALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Coronae Australis",
        constellation: "Corona Australis",
        right_ascension: RightAscension::new(19, 10, 2.),
        declination: Declination::new(Sgn::Neg, 39, 20, 27.),
        apparent_magnitude: 4.10,
        distance: Length::new::<light_year>(470.),
        absolute_magnitude: -1.71,
        mass: Mass::new::<solar_mass>(5.17),
        radius: Some(Length::new::<solar_radii>(38.5)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4575.),
        age: None,
        lifetime: Time::new::<gigayear>(0.10143918),
    }
}

fn GAMMA_CORONAE_AUSTRALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ¹ Coronae Australis",
        constellation: "Corona Australis",
        right_ascension: RightAscension::new(19, 6, 25.),
        declination: Declination::new(Sgn::Neg, 37, 3, 48.),
        apparent_magnitude: 4.23,
        distance: Length::new::<light_year>(58.33),
        absolute_magnitude: 2.97,
        mass: Mass::new::<solar_mass>(1.15),
        radius: Some(Length::new::<solar_radii>(1.47)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6090.),
        age: Some(Time::new::<gigayear>(5.)),
        lifetime: Time::new::<gigayear>(5.9461393),
    }
}

pub(crate) const STARS: [RealData; 3] =
    [MERIDIANA, BETA_CORONAE_AUSTRALIS, GAMMA_CORONAE_AUSTRALIS];
