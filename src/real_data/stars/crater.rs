use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn alkes() -> RealData {
    RealData {
        common_name: "Alkes",
        astronomical_name: "α Crateris",
        constellation: "Crater",
        right_ascension: RightAscension::new(10, 59, 46.),
        declination: Declination::new(Sgn::Neg, 18, 17, 56.),
        apparent_magnitude: 4.08,
        distance: Length::new::<light_year>(174.2),
        absolute_magnitude: 0.44,
        mass: Mass::new::<solar_mass>(1.81),
        radius: Some(Length::new::<solar_radius>(12.32)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4691.),
        age: Some(Time::new::<gigayear>(1.4)),
        lifetime: Time::new::<gigayear>(1.46605285),
    }
}

fn beta_crateris() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Crateris",
        constellation: "Crater",
        right_ascension: RightAscension::new(11, 11, 39.),
        declination: Declination::new(Sgn::Neg, 22, 49, 33.),
        apparent_magnitude: 4.46,
        distance: Length::new::<light_year>(296.),
        absolute_magnitude: -0.62,
        mass: Mass::new::<solar_mass>(2.6),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(8830.),
        age: None,
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn gamma_crateris() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Crateris",
        constellation: "Crater",
        right_ascension: RightAscension::new(11, 24, 53.),
        declination: Declination::new(Sgn::Neg, 17, 41, 2.),
        apparent_magnitude: 4.06,
        distance: Length::new::<light_year>(85.6),
        absolute_magnitude: 2.05,
        mass: Mass::new::<solar_mass>(1.81),
        radius: Some(Length::new::<solar_radius>(1.3)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8020.),
        age: Some(Time::new::<gigayear>(0.757)),
        lifetime: Time::new::<gigayear>(1.46605285),
    }
}

fn delta_crateris() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Crateris",
        constellation: "Crater",
        right_ascension: RightAscension::new(11, 19, 20.),
        declination: Declination::new(Sgn::Neg, 14, 46, 42.),
        apparent_magnitude: 3.56,
        distance: Length::new::<light_year>(194.6),
        absolute_magnitude: -0.321,
        mass: Mass::new::<solar_mass>(1.56),
        radius: Some(Length::new::<solar_radius>(22.44)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4510.),
        age: Some(Time::new::<gigayear>(2.2)),
        lifetime: Time::new::<gigayear>(2.29668629),
    }
}

pub(crate) fn stars() -> [RealData; 4] {
    [alkes(), beta_crateris(), gamma_crateris(), delta_crateris()]
}
