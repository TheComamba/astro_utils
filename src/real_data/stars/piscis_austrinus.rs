use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn formalhaut() -> RealData {
    RealData {
        common_name: "Formalhaut",
        astronomical_name: "α Piscis Austrini",
        constellation: "Piscis Austrinus",
        radius: Some(Length::new::<solar_radii>(1.842)),
        mass: Mass::new::<solar_mass>(1.92),
        absolute_magnitude: 1.74,
        apparent_magnitude: 1.17,
        temperature: ThermodynamicTemperature::new::<kelvin>(8590.),
        age: Some(Time::new::<gigayear>(0.44)),
        right_ascension: RightAscension::new(22, 57, 39.),
        declination: Declination::new(Sgn::Neg, 29, 37, 20.),
        distance: Length::new::<light_year>(25.),
        lifetime: Time::new::<gigayear>(1.54706939),
    }
}

fn delta_piscis_austrini() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Piscis Austrini",
        constellation: "Piscis Austrinus",
        right_ascension: RightAscension::new(22, 55, 57.),
        declination: Declination::new(Sgn::Neg, 32, 32, 23.),
        apparent_magnitude: 4.175,
        distance: Length::new::<light_year>(172.),
        absolute_magnitude: 0.636,
        mass: Mass::new::<solar_mass>(1.42),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(4828.),
        age: Some(Time::new::<gigayear>(3.1)),
        lifetime: Time::new::<gigayear>(3.10253119),
    }
}

fn epsilon_piscis_austrini() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Piscis Austrini",
        constellation: "Piscis Austrinus",
        right_ascension: RightAscension::new(22, 40, 39.),
        declination: Declination::new(Sgn::Neg, 27, 2, 37.),
        apparent_magnitude: 4.18,
        distance: Length::new::<light_year>(744.3),
        absolute_magnitude: -2.61,
        mass: Mass::new::<solar_mass>(4.1),
        radius: Some(Length::new::<solar_radii>(3.2)),
        temperature: ThermodynamicTemperature::new::<kelvin>(11_066.),
        age: None,
        lifetime: Time::new::<gigayear>(0.193156929),
    }
}

fn iota_piscis_austrini() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ι Piscis Austrini",
        constellation: "Piscis Austrinus",
        right_ascension: RightAscension::new(21, 44, 57.),
        declination: Declination::new(Sgn::Neg, 33, 1, 33.),
        apparent_magnitude: 4.35,
        distance: Length::new::<light_year>(204.),
        absolute_magnitude: 0.37,
        mass: Mass::new::<solar_mass>(3.1),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(9330.),
        age: None,
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

pub(crate) fn stars() -> [RealData; 4] {
    [
        formalhaut(),
        delta_piscis_austrini(),
        epsilon_piscis_austrini(),
        iota_piscis_austrini(),
    ]
}
