use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn naos() -> RealData {
    RealData {
        common_name: "Naos",
        astronomical_name: "ζ Puppis",
        constellation: "Puppis",
        radius: Some(Length::new::<solar_radius>(20.)),
        mass: Mass::new::<solar_mass>(56.1),
        absolute_magnitude: -5.95,
        apparent_magnitude: 2.21,
        temperature: ThermodynamicTemperature::new::<kelvin>(40_000.),
        age: Some(Time::new::<gigayear>(0.0032)),
        right_ascension: RightAscension::new(8, 3, 35.),
        declination: Declination::new(Sgn::Neg, 40, 0, 12.),
        distance: Length::new::<light_year>(1399.),
        lifetime: Time::new::<gigayear>(0.00435),
    }
}

fn ahadi() -> RealData {
    RealData {
        common_name: "Ahadi",
        astronomical_name: "π Puppis",
        constellation: "Puppis",
        radius: Some(Length::new::<solar_radius>(235.)),
        mass: Mass::new::<solar_mass>(11.7),
        absolute_magnitude: -4.92,
        apparent_magnitude: 2.71,
        temperature: ThermodynamicTemperature::new::<kelvin>(4000.),
        right_ascension: RightAscension::new(7, 17, 9.),
        declination: Declination::new(Sgn::Neg, 37, 5, 51.),
        distance: Length::new::<light_year>(1094.),
        age: Some(Time::new::<gigayear>(0.019)),
        lifetime: Time::new::<gigayear>(0.019450199),
    }
}

fn rho_puppis() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ρ Puppis",
        constellation: "Puppis",
        right_ascension: RightAscension::new(8, 7, 33.),
        declination: Declination::new(Sgn::Neg, 24, 18, 16.),
        apparent_magnitude: 2.78,
        distance: Length::new::<light_year>(63.5),
        absolute_magnitude: 1.41,
        mass: Mass::new::<solar_mass>(1.85),
        radius: Some(Length::new::<solar_radius>(3.41)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6920.),
        age: Some(Time::new::<gigayear>(1.5)),
        lifetime: Time::new::<gigayear>(1.65092742),
    }
}

fn tau_puppis() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "τ Puppis",
        constellation: "Puppis",
        right_ascension: RightAscension::new(6, 49, 56.),
        declination: Declination::new(Sgn::Neg, 50, 36, 52.),
        apparent_magnitude: 2.95,
        distance: Length::new::<light_year>(174.),
        absolute_magnitude: -0.80,
        mass: Mass::new::<solar_mass>(3.19),
        radius: Some(Length::new::<solar_radius>(27.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4489.),
        age: Some(Time::new::<gigayear>(0.3)),
        lifetime: Time::new::<gigayear>(0.351318702),
    }
}

fn nu_puppis() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ν Puppis",
        constellation: "Puppis",
        right_ascension: RightAscension::new(6, 37, 46.),
        declination: Declination::new(Sgn::Neg, 43, 11, 45.),
        apparent_magnitude: 3.173,
        distance: Length::new::<light_year>(370.),
        absolute_magnitude: -2.11,
        mass: Mass::new::<solar_mass>(5.2),
        radius: Some(Length::new::<solar_radius>(4.2)),
        temperature: ThermodynamicTemperature::new::<kelvin>(12_120.),
        age: None,
        lifetime: Time::new::<gigayear>(0.10143918),
    }
}

pub(crate) fn stars() -> [RealData; 5] {
    [naos(), ahadi(), rho_puppis(), tau_puppis(), nu_puppis()]
}
