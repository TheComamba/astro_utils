use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn gamma_microscopii() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Microscopii",
        constellation: "Microscopium",
        right_ascension: RightAscension::new(21, 1, 17.),
        declination: Declination::new(Sgn::Neg, 32, 15, 28.),
        apparent_magnitude: 4.680,
        distance: Length::new::<light_year>(223.),
        absolute_magnitude: 0.49,
        mass: Mass::new::<solar_mass>(2.5),
        radius: Some(Length::new::<solar_radii>(10.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5050.),
        age: Some(Time::new::<gigayear>(0.620)),
        lifetime: Time::new::<gigayear>(0.800458342),
    }
}

fn epsilon_microscopii() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Microscopii",
        constellation: "Microscopium",
        right_ascension: RightAscension::new(21, 17, 56.),
        declination: Declination::new(Sgn::Neg, 32, 10, 21.),
        apparent_magnitude: 4.71,
        distance: Length::new::<light_year>(166.),
        absolute_magnitude: 0.97,
        mass: Mass::new::<solar_mass>(2.18),
        radius: Some(Length::new::<solar_radii>(2.2)),
        temperature: ThermodynamicTemperature::new::<kelvin>(9126.),
        age: Some(Time::new::<gigayear>(0.525)),
        lifetime: Time::new::<gigayear>(1.03650581),
    }
}

fn theta1_microscopii() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "θ¹ Microscopii",
        constellation: "Microscopium",
        right_ascension: RightAscension::new(21, 20, 46.),
        declination: Declination::new(Sgn::Neg, 40, 48, 34.),
        apparent_magnitude: 4.82,
        distance: Length::new::<light_year>(179.),
        absolute_magnitude: 1.03,
        mass: Mass::new::<solar_mass>(2.32),
        radius: Some(Length::new::<solar_radii>(2.35)),
        temperature: ThermodynamicTemperature::new::<kelvin>(9240.),
        age: Some(Time::new::<gigayear>(0.437)),
        lifetime: Time::new::<gigayear>(0.916355612),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [
        gamma_microscopii(),
        epsilon_microscopii(),
        theta1_microscopii(),
    ]
}
