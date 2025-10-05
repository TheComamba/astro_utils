use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn alpha_sculptoris() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Sculptoris",
        constellation: "Sculptor",
        right_ascension: RightAscension::new(0, 58, 36.),
        declination: Declination::new(Sgn::Neg, 29, 21, 27.),
        apparent_magnitude: 4.30,
        distance: Length::new::<light_year>(780.),
        absolute_magnitude: -2.58,
        mass: Mass::new::<solar_mass>(5.01),
        radius: Some(Length::new::<solar_radii>(7.52)),
        temperature: ThermodynamicTemperature::new::<kelvin>(13_600.),
        age: Some(Time::new::<gigayear>(0.093)),
        lifetime: Time::new::<gigayear>(0.111319448),
    }
}

fn beta_sculptoris() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Sculptoris",
        constellation: "Sculptor",
        right_ascension: RightAscension::new(23, 32, 58.),
        declination: Declination::new(Sgn::Neg, 37, 49, 6.),
        apparent_magnitude: 4.37,
        distance: Length::new::<light_year>(174.),
        absolute_magnitude: 0.74,
        mass: Mass::new::<solar_mass>(2.98),
        radius: Some(Length::new::<solar_radii>(2.0)),
        temperature: ThermodynamicTemperature::new::<kelvin>(12_110.),
        age: None,
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

fn gamma_sulptoris() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Sculptoris",
        constellation: "Sculptor",
        right_ascension: RightAscension::new(23, 18, 49.),
        declination: Declination::new(Sgn::Neg, 32, 31, 55.),
        apparent_magnitude: 4.41,
        distance: Length::new::<light_year>(182.),
        absolute_magnitude: 0.67,
        mass: Mass::new::<solar_mass>(1.6),
        radius: Some(Length::new::<solar_radii>(12.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4578.),
        age: Some(Time::new::<gigayear>(2.)),
        lifetime: Time::new::<gigayear>(2.08398753),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [alpha_sculptoris(), beta_sculptoris(), gamma_sulptoris()]
}
