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

fn ANKAA() -> RealData {
    RealData {
        common_name: "Ankaa",
        astronomical_name: "α Phoenicis",
        constellation: "Phoenix",
        radius: Some(Length::new::<solar_radii>(15.)),
        mass: Mass::new::<solar_mass>(1.57),
        absolute_magnitude: 0.52,
        apparent_magnitude: 2.4,
        temperature: ThermodynamicTemperature::new::<kelvin>(4436.),
        age: None,
        right_ascension: RightAscension::new(0, 26, 17.),
        declination: Declination::new(Sgn::Neg, 42, 18, 21.),
        distance: Length::new::<light_year>(77.),
        lifetime: Time::new::<gigayear>(2.29668629),
    }
}

fn BETA_PHOENICIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Phoenicis",
        constellation: "Phoenix",
        right_ascension: RightAscension::new(1, 6, 5.),
        declination: Declination::new(Sgn::Neg, 46, 43, 6.),
        apparent_magnitude: 3.32,
        distance: Length::new::<light_year>(198.1),
        absolute_magnitude: -0.598,
        mass: Mass::new::<solar_mass>(2.5),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(5090.),
        age: None,
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

fn GAMMA_PHOENICIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Phoenicis",
        constellation: "Phoenix",
        right_ascension: RightAscension::new(1, 28, 22.),
        declination: Declination::new(Sgn::Neg, 43, 19, 6.),
        apparent_magnitude: 3.41,
        distance: Length::new::<light_year>(234.),
        absolute_magnitude: -0.86,
        mass: Mass::new::<solar_mass>(1.3),
        radius: Some(Length::new::<solar_radii>(52.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(3802.),
        age: None,
        lifetime: Time::new::<gigayear>(3.9126515),
    }
}

pub(crate) fn STARS() -> [RealData; 3] {
    [ANKAA(), BETA_PHOENICIS(), GAMMA_PHOENICIS()]
}
