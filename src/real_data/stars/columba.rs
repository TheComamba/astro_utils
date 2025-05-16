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

fn PHACT() -> RealData {
    RealData {
        common_name: "Phact",
        astronomical_name: "α Columbae",
        constellation: "Columba",
        right_ascension: RightAscension::new(5, 39, 39.),
        declination: Declination::new(Sgn::Neg, 34, 4, 27.),
        apparent_magnitude: 2.645,
        distance: Length::new::<light_year>(261.),
        absolute_magnitude: -1.87,
        mass: Mass::new::<solar_mass>(4.5),
        radius: Some(Length::new::<solar_radii>(5.8)),
        temperature: Temperature { K: 12_963. },
        age: Some(Time::new::<gigayear>(0.093)),
        lifetime: Time::new::<gigayear>(0.151849866),
    }
}

fn WAZN() -> RealData {
    RealData {
        common_name: "Wazn",
        astronomical_name: "β Columbae",
        constellation: "Columba",
        right_ascension: RightAscension::new(5, 50, 58.),
        declination: Declination::new(Sgn::Neg, 35, 46, 6.),
        apparent_magnitude: 3.105,
        distance: Length::new::<light_year>(87.41),
        absolute_magnitude: 1.01,
        mass: Mass::new::<solar_mass>(1.1),
        radius: Some(Length::new::<solar_radii>(11.5)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4545.),
        age: Some(Time::new::<gigayear>(2.)),
        lifetime: Time::new::<gigayear>(6.97272616),
    }
}

fn DELTA_COLUMBAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Columbae",
        constellation: "Columba",
        right_ascension: RightAscension::new(6, 22, 7.),
        declination: Declination::new(Sgn::Neg, 33, 26, 11.),
        apparent_magnitude: 3.85,
        distance: Length::new::<light_year>(234.),
        absolute_magnitude: -0.32,
        mass: Mass::new::<solar_mass>(3.7),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(5136.),
        age: None,
        lifetime: Time::new::<gigayear>(0.254814649),
    }
}

fn EPSILON_COLUMBAE() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Columbae",
        constellation: "Columba",
        right_ascension: RightAscension::new(5, 31, 13.),
        declination: Declination::new(Sgn::Neg, 35, 28, 14.),
        apparent_magnitude: 3.87,
        distance: Length::new::<light_year>(262.),
        absolute_magnitude: -0.67,
        mass: Mass::new::<solar_mass>(2.47),
        radius: Some(Length::new::<solar_radii>(25.2)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4575.),
        age: Some(Time::new::<gigayear>(0.8)),
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

pub(crate) const STARS: [RealData; 4] = [PHACT, WAZN, DELTA_COLUMBAE, EPSILON_COLUMBAE];
