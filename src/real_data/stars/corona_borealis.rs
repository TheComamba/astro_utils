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

fn ALPHECCA() -> RealData {
    RealData {
        common_name: "Alphecca",
        astronomical_name: "α Coronae Borealis",
        constellation: "Corona Borealis",
        radius: Some(Length::new::<solar_radii>(3.)),
        mass: Mass::new::<solar_mass>(2.58),
        absolute_magnitude: 0.42,
        apparent_magnitude: 2.22,
        temperature: ThermodynamicTemperature::new::<kelvin>(9700.),
        age: Some(Time::new::<gigayear>(0.314)),
        lifetime: Time::new::<gigayear>(0.63513384),
        right_ascension: RightAscension::new(15, 34, 41.),
        declination: Declination::new(Sgn::Pos, 26, 42, 53.),
        distance: Length::new::<light_year>(75.),
    }
}

fn NAUSAKAN() -> RealData {
    RealData {
        common_name: "Nausakan",
        astronomical_name: "β Coronae Borealis",
        constellation: "Corona Borealis",
        right_ascension: RightAscension::new(15, 27, 50.),
        declination: Declination::new(Sgn::Pos, 29, 6, 21.),
        apparent_magnitude: 3.7,
        distance: Length::new::<light_year>(112.),
        absolute_magnitude: 0.942,
        mass: Mass::new::<solar_mass>(2.09),
        radius: Some(Length::new::<solar_radii>(2.63)),
        temperature: ThermodynamicTemperature::new::<kelvin>(7980.),
        age: None,
        lifetime: Time::new::<gigayear>(1.17901142),
    }
}

fn GAMMA_CORONAE_BOREALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Coronae Borealis",
        constellation: "Corona Borealis",
        right_ascension: RightAscension::new(15, 42, 45.),
        declination: Declination::new(Sgn::Pos, 26, 17, 44.),
        apparent_magnitude: 3.80,
        distance: Length::new::<light_year>(146.),
        absolute_magnitude: 0.56,
        mass: Mass::new::<solar_mass>(2.51),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(7649.),
        age: Some(Time::new::<gigayear>(0.4)),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn DELTA_CORONAE_BOREALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Coronae Borealis",
        constellation: "Corona Borealis",
        right_ascension: RightAscension::new(15, 49, 36.),
        declination: Declination::new(Sgn::Pos, 26, 4, 6.),
        apparent_magnitude: 4.57,
        distance: Length::new::<light_year>(165.),
        absolute_magnitude: 1.18,
        mass: Mass::new::<solar_mass>(2.4),
        radius: Some(Length::new::<solar_radii>(7.4)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5180.),
        age: Some(Time::new::<gigayear>(0.8)),
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

fn EPSILON_CORONAE_BOREALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Coronae Borealis",
        constellation: "Corona Borealis",
        right_ascension: RightAscension::new(15, 57, 35.),
        declination: Declination::new(Sgn::Pos, 26, 52, 40.),
        apparent_magnitude: 4.13,
        distance: Length::new::<light_year>(242.),
        absolute_magnitude: -0.02,
        mass: Mass::new::<solar_mass>(1.44),
        radius: Some(Length::new::<solar_radii>(21.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4365.),
        age: Some(Time::new::<gigayear>(2.8)),
        lifetime: Time::new::<gigayear>(2.82957282),
    }
}

fn IOTA_CORONAE_BOREALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ι Coronae Borealis",
        constellation: "Corona Borealis",
        right_ascension: RightAscension::new(16, 1, 27.),
        declination: Declination::new(Sgn::Pos, 29, 51, 4.),
        apparent_magnitude: 4.96,
        distance: Length::new::<light_year>(312.),
        absolute_magnitude: 0.08,
        mass: Mass::new::<solar_mass>(3.),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(10_727.),
        age: None,
        lifetime: Time {
            s: 0.420724107 * .s, //no idea
        },
    }
}

fn THETA_CORONAE_BOREALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "θ Coronae Borealis",
        constellation: "Corona Borealis",
        right_ascension: RightAscension::new(15, 32, 56.),
        declination: Declination::new(Sgn::Pos, 31, 21, 33.),
        apparent_magnitude: 4.1,
        distance: Length::new::<light_year>(380.),
        absolute_magnitude: -1.16,
        mass: Mass::new::<solar_mass>(4.2),
        radius: Some(Length::new::<solar_radii>(3.3)),
        temperature: ThermodynamicTemperature::new::<kelvin>(14_000.),
        age: Some(Time::new::<gigayear>(0.085)),
        lifetime: Time::new::<gigayear>(0.170765802),
    }
}

pub(crate) const STARS: [RealData; 7] = [
    ALPHECCA(),
    NAUSAKAN(),
    GAMMA_CORONAE_BOREALIS(),
    DELTA_CORONAE_BOREALIS(),
    EPSILON_CORONAE_BOREALIS(),
    IOTA_CORONAE_BOREALIS(),
    THETA_CORONAE_BOREALIS(),
];
