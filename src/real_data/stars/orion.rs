use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
    time::year,
};

use crate::stars::real_data::RealData;

fn rigel() -> RealData {
    RealData {
        common_name: "Rigel",
        astronomical_name: "β Orionis",
        constellation: "Orion",
        radius: Some(Length::new::<solar_radius>(78.9)),
        mass: Mass::new::<solar_mass>(21.),
        absolute_magnitude: -6.69,
        apparent_magnitude: 0.18,
        temperature: ThermodynamicTemperature::new::<kelvin>(12_100.),
        age: Some(Time::new::<gigayear>(0.008)),
        right_ascension: RightAscension::new(5, 14, 32.),
        declination: Declination::new(Sgn::Neg, 8, 12, 6.),
        distance: Length::new::<light_year>(773.),
        lifetime: Time::new::<gigayear>(0.009767659),
    }
}

fn betelgeuse() -> RealData {
    let lifetime = Time::new::<gigayear>(0.012799766);
    RealData {
        common_name: "Betelgeuse",
        astronomical_name: "α Orionis",
        constellation: "Orion",
        radius: Some(Length::new::<solar_radius>(887.)),
        mass: Mass::new::<solar_mass>(16.5),
        absolute_magnitude: -5.14,
        apparent_magnitude: 0.9,
        temperature: ThermodynamicTemperature::new::<kelvin>(3600.),
        right_ascension: RightAscension::new(5, 55, 10.),
        declination: Declination::new(Sgn::Pos, 7, 24, 25.),
        distance: Length::new::<light_year>(522.),
        age: Some(lifetime - Time::new::<year>(100.)),
        lifetime,
    }
}

fn bellatrix() -> RealData {
    RealData {
        common_name: "Bellatrix",
        astronomical_name: "γ Orionis",
        constellation: "Orion",
        radius: Some(Length::new::<solar_radius>(5.75)),
        mass: Mass::new::<solar_mass>(7.7),
        absolute_magnitude: -2.72,
        apparent_magnitude: 1.64,
        temperature: ThermodynamicTemperature::new::<kelvin>(21_800.),
        age: Some(Time::new::<gigayear>(0.0252)),
        right_ascension: RightAscension::new(5, 25, 8.),
        declination: Declination::new(Sgn::Pos, 6, 20, 59.),
        distance: Length::new::<light_year>(243.),
        lifetime: Time::new::<gigayear>(0.040555762),
    }
}

fn alnilam() -> RealData {
    RealData {
        common_name: "Alnilam",
        astronomical_name: "ε Orionis",
        constellation: "Orion",
        radius: Some(Length::new::<solar_radius>(42.)),
        mass: Mass::new::<solar_mass>(34.6),
        absolute_magnitude: -6.38,
        apparent_magnitude: 1.69,
        temperature: ThermodynamicTemperature::new::<kelvin>(27_000.),
        right_ascension: RightAscension::new(5, 36, 13.),
        declination: Declination::new(Sgn::Neg, 1, 12, 7.),
        distance: Length::new::<light_year>(1342.),
        age: Some(Time::new::<gigayear>(0.0057)),
        lifetime: Time::new::<gigayear>(0.005807621),
    }
}

fn alnitak() -> RealData {
    RealData {
        common_name: "Alnitak",
        astronomical_name: "ζ Orionis",
        constellation: "Orion",
        radius: Some(Length::new::<solar_radius>(20.)),
        mass: Mass::new::<solar_mass>(31.0),
        absolute_magnitude: -5.26,
        apparent_magnitude: 1.74,
        temperature: ThermodynamicTemperature::new::<kelvin>(29_500.),
        right_ascension: RightAscension::new(5, 40, 46.),
        declination: Declination::new(Sgn::Neg, 1, 56, 34.),
        distance: Length::new::<light_year>(817.),
        age: Some(Time::new::<gigayear>(0.0064)),
        lifetime: Time::new::<gigayear>(0.006573099),
    }
}

fn saiph() -> RealData {
    RealData {
        common_name: "Saiph",
        astronomical_name: "κ Orionis",
        constellation: "Orion",
        radius: Some(Length::new::<solar_radius>(22.2)),
        mass: Mass::new::<solar_mass>(15.5),
        absolute_magnitude: -4.65,
        apparent_magnitude: 2.07,
        temperature: ThermodynamicTemperature::new::<kelvin>(26_500.),
        age: Some(Time::new::<gigayear>(0.0111)),
        right_ascension: RightAscension::new(5, 47, 45.),
        declination: Declination::new(Sgn::Neg, 9, 40, 11.),
        distance: Length::new::<light_year>(721.2),
        lifetime: Time::new::<gigayear>(0.012799766),
    }
}

fn mintaka() -> RealData {
    RealData {
        common_name: "Mintaka",
        astronomical_name: "δ Orionis",
        constellation: "Orion",
        radius: Some(Length::new::<solar_radius>(16.5)),
        mass: Mass::new::<solar_mass>(24.),
        absolute_magnitude: -4.99,
        apparent_magnitude: 2.25,
        temperature: ThermodynamicTemperature::new::<kelvin>(29_500.),
        right_ascension: RightAscension::new(5, 32, 0.),
        declination: Declination::new(Sgn::Neg, 0, 17, 57.),
        distance: Length::new::<light_year>(916.),
        age: Some(Time::new::<gigayear>(0.008)),
        lifetime: Time::new::<gigayear>(0.008063854),
    }
}

pub(crate) fn stars() -> [RealData; 7] {
    [
        rigel(),
        betelgeuse(),
        bellatrix(),
        alnilam(),
        alnitak(),
        saiph(),
        mintaka(),
    ]
}
