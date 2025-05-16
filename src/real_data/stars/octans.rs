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

fn NU_OCTANTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ν Octantis",
        constellation: "Octans",
        right_ascension: RightAscension::new(21, 41, 29.),
        declination: Declination::new(Sgn::Neg, 77, 23, 24.),
        apparent_magnitude: 3.73,
        distance: Length::new::<light_year>(63.3),
        absolute_magnitude: 2.10,
        mass: Mass::new::<solar_mass>(1.04),
        radius: Some(Length::new::<solar_radii>(5.9)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4860.),
        age: Some(Time::new::<gigayear>(2.5)),
        lifetime: Time::new::<gigayear>(8.24015833),
    }
}

fn BETA_OCTANTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Octantis",
        constellation: "Octans",
        right_ascension: RightAscension::new(22, 46, 4.),
        declination: Declination::new(Sgn::Neg, 81, 22, 54.),
        apparent_magnitude: 4.13,
        distance: Length::new::<light_year>(149.),
        absolute_magnitude: 0.83,
        mass: Mass::new::<solar_mass>(2.27),
        radius: Some(Length::new::<solar_radii>(3.2)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8006.),
        age: Some(Time::new::<gigayear>(0.496)),
        lifetime: Time::new::<gigayear>(0.964406929),
    }
}

fn DELTA_OCTANTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Octantis",
        constellation: "Octans",
        right_ascension: RightAscension::new(14, 26, 55.),
        declination: Declination::new(Sgn::Neg, 83, 40, 4.),
        apparent_magnitude: 4.31,
        distance: Length::new::<light_year>(299.),
        absolute_magnitude: -0.35,
        mass: Mass::new::<solar_mass>(1.06),
        radius: Some(Length::new::<solar_radii>(24.61)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4311.),
        age: None,
        lifetime: Time::new::<gigayear>(8.24015833),
    }
}

pub(crate) const STARS: [RealData; 3] = [NU_OCTANTIS, BETA_OCTANTIS, DELTA_OCTANTIS];
