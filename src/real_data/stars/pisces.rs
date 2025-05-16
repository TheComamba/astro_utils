use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

fn ALPHA_PISCIUM() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Piscium",
        constellation: "Pisces",
        right_ascension: RightAscension::new(2, 2, 3.),
        declination: Declination::new(Sgn::Pos, 2, 45, 50.),
        apparent_magnitude: 3.82,
        distance: Length::new::<light_year>(151.),
        absolute_magnitude: 0.5,
        mass: Mass::new::<solar_mass>(2.55),
        radius: Some(Length::new::<solar_radii>(2.45)),
        temperature: Temperature { K: 10_233. },
        age: None,
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn DELTA_PISCIUM() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Piscium",
        constellation: "Pisces",
        right_ascension: RightAscension::new(0, 48, 41.),
        declination: Declination::new(Sgn::Pos, 7, 35, 6.),
        apparent_magnitude: 4.416,
        distance: Length::new::<light_year>(311.),
        absolute_magnitude: -0.46,
        mass: Mass::new::<solar_mass>(1.65),
        radius: Some(Length::new::<solar_radii>(44.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(3963.),
        age: Some(Time::new::<gigayear>(0.00298)),
        lifetime: Time::new::<gigayear>(1.89665739),
    }
}

fn NU_PISCIUM() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ν Piscium",
        constellation: "Pisces",
        right_ascension: RightAscension::new(1, 41, 26.),
        declination: Declination::new(Sgn::Pos, 5, 29, 15.),
        apparent_magnitude: 4.44,
        distance: Length::new::<light_year>(363.),
        absolute_magnitude: -0.78,
        mass: Mass::new::<solar_mass>(1.66),
        radius: Some(Length::new::<solar_radii>(34.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4154.),
        age: Some(Time::new::<gigayear>(1.8)),
        lifetime: Time::new::<gigayear>(1.89665739),
    }
}

fn IOTA_PISCIUM() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ι Piscium",
        constellation: "Pisces",
        right_ascension: RightAscension::new(23, 39, 57.),
        declination: Declination::new(Sgn::Pos, 5, 37, 35.),
        apparent_magnitude: 4.13,
        distance: Length::new::<light_year>(44.73),
        absolute_magnitude: 3.43,
        mass: Mass::new::<solar_mass>(1.3),
        radius: Some(Length::new::<solar_radii>(1.595)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6288.),
        age: Some(Time::new::<gigayear>(3.8)),
        lifetime: Time::new::<gigayear>(3.9126515),
    }
}

fn OMICRON_PISCIUM() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ο Piscium",
        constellation: "Pisces",
        right_ascension: RightAscension::new(1, 45, 24.),
        declination: Declination::new(Sgn::Pos, 9, 9, 28.),
        apparent_magnitude: 4.27,
        distance: Length::new::<light_year>(280.),
        absolute_magnitude: -0.22,
        mass: Mass::new::<solar_mass>(3.03),
        radius: Some(Length::new::<solar_radii>(14.57)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5004.),
        age: Some(Time::new::<gigayear>(0.390)),
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

fn EPSILON_PISCIUM() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Piscium",
        constellation: "Pisces",
        right_ascension: RightAscension::new(1, 2, 57.),
        declination: Declination::new(Sgn::Pos, 7, 53, 24.),
        apparent_magnitude: 4.27,
        distance: Length::new::<light_year>(182.),
        absolute_magnitude: 0.44,
        mass: Mass::new::<solar_mass>(2.27),
        radius: Some(Length::new::<solar_radii>(10.9)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4814.),
        age: Some(Time::new::<gigayear>(0.9)),
        lifetime: Time::new::<gigayear>(0.964406929),
    }
}

fn THETA_PISCIUM() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "θ Piscium",
        constellation: "Pisces",
        right_ascension: RightAscension::new(23, 27, 58.),
        declination: Declination::new(Sgn::Pos, 6, 22, 44.),
        apparent_magnitude: 4.27,
        distance: Length::new::<light_year>(149.),
        absolute_magnitude: 0.83,
        mass: Mass::new::<solar_mass>(1.58),
        radius: Some(Length::new::<solar_radii>(11.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4684.),
        age: Some(Time::new::<gigayear>(0.00245)),
        lifetime: Time::new::<gigayear>(2.08398753),
    }
}

fn ETA_PISCIUM() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "η Piscium",
        constellation: "Pisces",
        right_ascension: RightAscension::new(1, 31, 29.),
        declination: Declination::new(Sgn::Pos, 15, 20, 45.),
        apparent_magnitude: 3.611,
        distance: Length::new::<light_year>(350.),
        absolute_magnitude: -1.52,
        mass: Mass::new::<solar_mass>(3.78),
        radius: Some(Length::new::<solar_radii>(26.48)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4937.),
        age: Some(Time::new::<gigayear>(0.220)),
        lifetime: Time::new::<gigayear>(0.220601963),
    }
}

fn GAMMA_PISCIUM() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Piscium",
        constellation: "Pisces",
        right_ascension: RightAscension::new(23, 17, 10.),
        declination: Declination::new(Sgn::Pos, 3, 16, 56.),
        apparent_magnitude: 3.699,
        distance: Length::new::<light_year>(135.),
        absolute_magnitude: 0.68,
        mass: Mass::new::<solar_mass>(0.97),
        radius: Some(Length::new::<solar_radii>(11.28)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4833.),
        age: Some(Time::new::<gigayear>(4.58)),
        lifetime: Time::new::<gigayear>(11.7800188),
    }
}

fn OMEGA_PISCIUM() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ω Piscium",
        constellation: "Pisces",
        right_ascension: RightAscension::new(23, 59, 19.),
        declination: Declination::new(Sgn::Pos, 6, 51, 48.),
        apparent_magnitude: 4.01,
        distance: Length::new::<light_year>(104.3),
        absolute_magnitude: 1.51,
        mass: Mass::new::<solar_mass>(1.22),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(6641.),
        age: Some(Time::new::<gigayear>(1.337)),
        lifetime: Time::new::<gigayear>(5.06543331),
    }
}

pub(crate) const STARS: [RealData; 10] = [
    ALPHA_PISCIUM,
    DELTA_PISCIUM,
    NU_PISCIUM,
    IOTA_PISCIUM,
    OMICRON_PISCIUM,
    EPSILON_PISCIUM,
    THETA_PISCIUM,
    ETA_PISCIUM,
    GAMMA_PISCIUM,
    OMEGA_PISCIUM,
];
