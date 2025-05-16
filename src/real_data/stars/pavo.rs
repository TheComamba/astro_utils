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

fn PEACOCK() -> RealData {
    RealData {
        common_name: "Peacock",
        astronomical_name: "α Pavonis",
        constellation: "Pavo",
        radius: Some(Length::new::<solar_radii>(4.83)),
        mass: Mass::new::<solar_mass>(5.91),
        absolute_magnitude: -1.81,
        apparent_magnitude: 1.94,
        temperature: ThermodynamicTemperature::new::<kelvin>(17_711.),
        age: Some(Time::new::<gigayear>(0.048)),
        right_ascension: RightAscension::new(20, 25, 39.),
        declination: Declination::new(Sgn::Neg, 56, 44, 6.),
        distance: Length::new::<light_year>(183.),
        lifetime: Time::new::<gigayear>(0.073299383),
    }
}

fn BETA_PAVONIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Pavonis",
        constellation: "Pavo",
        right_ascension: RightAscension::new(20, 44, 57.),
        declination: Declination::new(Sgn::Neg, 66, 12, 12.),
        apparent_magnitude: 3.42,
        distance: Length::new::<light_year>(135.1),
        absolute_magnitude: 0.33,
        mass: Mass::new::<solar_mass>(2.51),
        radius: Some(Length::new::<solar_radii>(2.3)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8184.),
        age: Some(Time::new::<gigayear>(0.305)),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn DELTA_PAVONIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Pavonis",
        constellation: "Pavo",
        right_ascension: RightAscension::new(20, 8, 44.),
        declination: Declination::new(Sgn::Neg, 66, 10, 55.),
        apparent_magnitude: 3.56,
        distance: Length::new::<light_year>(19.89),
        absolute_magnitude: 4.62,
        mass: Mass::new::<solar_mass>(1.051),
        radius: Some(Length::new::<solar_radii>(1.197)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5571.),
        age: Some(Time::new::<gigayear>(6.7)),
        lifetime: Time::new::<gigayear>(8.24015833),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [PEACOCK(), BETA_PAVONIS(), DELTA_PAVONIS()] }
