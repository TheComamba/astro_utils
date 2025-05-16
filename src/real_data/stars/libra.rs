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

fn ZUBENELGENUBI() -> RealData {
    RealData {
        common_name: "Zubenelgenubi",
        astronomical_name: "α² Librae",
        constellation: "Libra",
        right_ascension: RightAscension::new(14, 50, 53.),
        declination: Declination::new(Sgn::Neg, 16, 2, 30.),
        apparent_magnitude: 2.741,
        distance: Length::new::<light_year>(75.8),
        absolute_magnitude: 0.879,
        mass: Mass::new::<solar_mass>(1.95),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(8128.),
        age: None,
        lifetime: Time::new::<gigayear>(1.46316038),
    }
}

fn ZUBENESCHAMALI() -> RealData {
    RealData {
        common_name: "Zubeneschamali",
        astronomical_name: "β Librae",
        constellation: "Libra",
        right_ascension: RightAscension::new(15, 17, 0.),
        declination: Declination::new(Sgn::Neg, 9, 22, 58.),
        apparent_magnitude: 2.61,
        distance: Length::new::<light_year>(185.),
        absolute_magnitude: -1.16,
        mass: Mass::new::<solar_mass>(3.5),
        radius: Some(Length::new::<solar_radii>(4.9)),
        temperature: ThermodynamicTemperature::new::<kelvin>(12_300.),
        age: Some(Time::new::<gigayear>(0.08)),
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

fn BRACHIUM() -> RealData {
    RealData {
        common_name: "Brachium",
        astronomical_name: "σ Librae",
        constellation: "Libra",
        radius: Some(Length::new::<solar_radii>(108.)),
        mass: Mass::new::<solar_mass>(2.2),
        absolute_magnitude: -1.5,
        apparent_magnitude: 3.21,
        temperature: ThermodynamicTemperature::new::<kelvin>(3596.),
        age: None,
        lifetime: Time::new::<gigayear>(1.03650581),
        right_ascension: RightAscension::new(15, 4, 4.),
        declination: Declination::new(Sgn::Neg, 25, 16, 55.),
        distance: Length::new::<light_year>(288.),
    }
}

pub(crate) const STARS: [RealData; 3] = [ZUBENELGENUBI, ZUBENESCHAMALI, BRACHIUM];
