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

fn VEGA() -> RealData {
    RealData {
        common_name: "Vega",
        astronomical_name: "α Lyrae",
        constellation: "Lyra",
        radius: Some(Length::new::<solar_radii>(2.362)),
        mass: Mass::new::<solar_mass>(2.135),
        absolute_magnitude: 0.58,
        apparent_magnitude: 0.03,
        temperature: ThermodynamicTemperature::new::<kelvin>(9602.),
        age: Some(Time::new::<gigayear>(0.455)),
        lifetime: Time::new::<gigayear>(1.09929685),
        right_ascension: RightAscension::new(18, 36, 56.),
        declination: Declination::new(Sgn::Pos, 38, 47, 1.),
        distance: Length::new::<light_year>(25.),
    }
}

fn R_LYRAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "R Lyrae",
        constellation: "Lyra",
        radius: None,
        mass: Mass::new::<solar_mass>(1.8),
        absolute_magnitude: -1.07,
        apparent_magnitude: 4.08,
        temperature: ThermodynamicTemperature::new::<kelvin>(3313.),
        age: None,
        lifetime: Time::new::<gigayear>(1.46605285),
        right_ascension: RightAscension::new(18, 55, 20.),
        declination: Declination::new(Sgn::Pos, 43, 56, 46.),
        distance: Length::new::<light_year>(349.4),
    }
}

fn GAMMA_LYRAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Lyrae",
        constellation: "Lyra",
        right_ascension: RightAscension::new(18, 58, 57.),
        declination: Declination::new(Sgn::Pos, 32, 41, 22.),
        apparent_magnitude: 3.261,
        distance: Length::new::<light_year>(620.),
        absolute_magnitude: -3.140,
        mass: Mass::new::<solar_mass>(5.76),
        radius: Some(Length::new::<solar_radii>(15.40)),
        temperature: Temperature { K: 10_000. },
        age: None,
        lifetime: Time::new::<gigayear>(0.078916095),
    }
}

fn BETA_LYRAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Lyrae",
        constellation: "Lyra",
        right_ascension: RightAscension::new(18, 50, 5.),
        declination: Declination::new(Sgn::Pos, 33, 21, 46.),
        apparent_magnitude: 3.52,
        distance: Length::new::<light_year>(960.),
        absolute_magnitude: -3.82,
        mass: Mass::new::<solar_mass>(2.97),
        radius: Some(Length::new::<solar_radii>(15.2)),
        temperature: Temperature { K: 13_300. },
        age: Some(Time::new::<gigayear>(0.023)),
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

pub(crate) const STARS: [RealData; 4] = [VEGA, R_LYRAE, GAMMA_LYRAE, BETA_LYRAE];
