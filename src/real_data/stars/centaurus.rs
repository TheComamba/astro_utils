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

fn RIGEL_KENTAURUS() -> RealData {
    RealData {
        common_name: "Rigel Kentaurus",
        astronomical_name: "α Centauri",
        constellation: "Centaurus",
        radius: Some(Length::new::<solar_radii>(1.2175)),
        mass: Mass::new::<solar_mass>(1.0788),
        absolute_magnitude: 4.34,
        apparent_magnitude: -0.27,
        temperature: ThermodynamicTemperature::new::<kelvin>(5790.),
        right_ascension: RightAscension::new(14, 39, 36.),
        declination: Declination::new(Sgn::Neg, 60, 50, 2.),
        distance: Length::new::<light_year>(4.),
        age: Some(Time::new::<gigayear>(4.85)),
        lifetime: Time::new::<gigayear>(6.97272616),
    }
}

fn HADAR() -> RealData {
    RealData {
        common_name: "Hadar",
        astronomical_name: "β Centauri",
        constellation: "Centaurus",
        radius: Some(Length::new::<solar_radii>(9.)),
        mass: Mass::new::<solar_mass>(12.02),
        absolute_magnitude: -5.42,
        apparent_magnitude: 0.61,
        temperature: ThermodynamicTemperature::new::<kelvin>(25_000.),
        right_ascension: RightAscension::new(14, 3, 49.),
        declination: Declination::new(Sgn::Neg, 60, 22, 23.),
        distance: Length::new::<light_year>(526.),
        age: Some(Time::new::<gigayear>(0.0141)),
        lifetime: Time::new::<gigayear>(0.019450199),
    }
}

fn MENKENT() -> RealData {
    RealData {
        common_name: "Menkent",
        astronomical_name: "θ Centauri",
        constellation: "Centaurus",
        radius: Some(Length::new::<solar_radii>(10.6)),
        mass: Mass::new::<solar_mass>(1.27),
        absolute_magnitude: 0.70,
        apparent_magnitude: 2.06,
        temperature: ThermodynamicTemperature::new::<kelvin>(4980.),
        right_ascension: RightAscension::new(14, 6, 41.),
        declination: Declination::new(Sgn::Neg, 36, 22, 11.),
        distance: Length::new::<light_year>(61.),
        age: None,
        lifetime: Time::new::<gigayear>(4.45521207),
    }
}

fn MUHLIFAIN() -> RealData {
    RealData {
        common_name: "Muhlifain",
        astronomical_name: "γ Centauri",
        constellation: "Centaurus",
        radius: None,
        mass: Mass::new::<solar_mass>(2.91),
        absolute_magnitude: -0.81,
        apparent_magnitude: 2.20,
        temperature: ThermodynamicTemperature::new::<kelvin>(9082.),
        right_ascension: RightAscension::new(12, 41, 31.),
        declination: Declination::new(Sgn::Neg, 48, 57, 35.),
        distance: Length::new::<light_year>(130.),
        age: None,
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

fn EPSILON_CENTAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Centauri",
        constellation: "Centaurus",
        radius: None,
        mass: Mass::new::<solar_mass>(11.6),
        absolute_magnitude: -3.02,
        apparent_magnitude: 2.29,
        temperature: ThermodynamicTemperature::new::<kelvin>(24_000.),
        right_ascension: RightAscension::new(13, 39, 53.),
        declination: Declination::new(Sgn::Neg, 53, 27, 59.),
        distance: Length::new::<light_year>(376.),
        age: Some(Time::new::<gigayear>(0.0158)),
        lifetime: Time::new::<gigayear>(0.019450199),
    }
}

fn ETA_CENTAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "η Centauri",
        constellation: "Centaurus",
        radius: Some(Length::new::<solar_radii>(6.1)),
        mass: Mass::new::<solar_mass>(12.0),
        absolute_magnitude: -2.55,
        apparent_magnitude: 2.29,
        temperature: ThermodynamicTemperature::new::<kelvin>(25_700.),
        right_ascension: RightAscension::new(14, 35, 30.),
        declination: Declination::new(Sgn::Neg, 42, 9, 28.),
        distance: Length::new::<light_year>(308.),
        age: Some(Time::new::<gigayear>(0.0056)),
        lifetime: Time::new::<gigayear>(0.019450199),
    }
}

fn ZETA_CENTAURI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Centauri",
        constellation: "Centaurus",
        radius: Some(Length::new::<solar_radii>(5.8)),
        mass: Mass::new::<solar_mass>(7.8),
        absolute_magnitude: -2.81,
        apparent_magnitude: 2.55,
        temperature: ThermodynamicTemperature::new::<kelvin>(23_561.),
        right_ascension: RightAscension::new(13, 55, 33.),
        declination: Declination::new(Sgn::Neg, 47, 17, 18.),
        distance: Length::new::<light_year>(384.),
        age: Some(Time::new::<gigayear>(0.04)),
        lifetime: Time::new::<gigayear>(0.040555762),
    }
}

fn MA_WEI() -> RealData {
    RealData {
        common_name: "Ma Wei",
        astronomical_name: "δ Centauri",
        constellation: "Centaurus",
        radius: Some(Length::new::<solar_radii>(6.5)),
        mass: Mass::new::<solar_mass>(8.7),
        absolute_magnitude: -2.84,
        apparent_magnitude: 2.58,
        temperature: ThermodynamicTemperature::new::<kelvin>(22_360.),
        right_ascension: RightAscension::new(12, 8, 21.),
        declination: Declination::new(Sgn::Neg, 50, 43, 21.),
        distance: Length::new::<light_year>(395.),
        age: Some(Time::new::<gigayear>(0.02)),
        lifetime: Time::new::<gigayear>(0.03224554),
    }
}

pub(crate) const STARS: [RealData; 8] = [
    RIGEL_KENTAURUS,
    HADAR,
    MENKENT,
    MUHLIFAIN,
    EPSILON_CENTAURI,
    ETA_CENTAURI,
    ZETA_CENTAURI,
    MA_WEI,
];
