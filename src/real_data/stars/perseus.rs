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

fn MIRPHAK() -> RealData {
    RealData {
        common_name: "Mirphak",
        astronomical_name: "α Persei",
        constellation: "Perseus",
        radius: Some(Length::new::<solar_radii>(68.)),
        mass: Mass::new::<solar_mass>(8.5),
        absolute_magnitude: -4.50,
        apparent_magnitude: 1.79,
        temperature: ThermodynamicTemperature::new::<kelvin>(6350.),
        right_ascension: RightAscension::new(3, 24, 19.),
        declination: Declination::new(Sgn::Pos, 49, 51, 40.),
        distance: Length::new::<light_year>(592.),
        age: Some(Time::new::<gigayear>(0.040)),
        lifetime: Time::new::<gigayear>(0.040555762),
    }
}

fn ALGOL() -> RealData {
    RealData {
        common_name: "Algol",
        astronomical_name: "β Persei",
        constellation: "Perseus",
        radius: Some(Length::new::<solar_radii>(2.73)),
        mass: Mass::new::<solar_mass>(3.17),
        absolute_magnitude: -0.18,
        apparent_magnitude: 2.09,
        temperature: ThermodynamicTemperature::new::<kelvin>(13_000.),
        right_ascension: RightAscension::new(3, 8, 10.),
        declination: Declination::new(Sgn::Pos, 40, 57, 20.),
        distance: Length::new::<light_year>(93.),
        age: Some(Time::new::<gigayear>(0.3)),
        lifetime: Time::new::<gigayear>(0.351318702),
    }
}

fn GORGONEA_TERTIA() -> RealData {
    RealData {
        common_name: "Gorgonea Tertia",
        astronomical_name: "ρ Persei",
        constellation: "Perseus",
        radius: Some(Length::new::<solar_radii>(143.)),
        mass: Mass::new::<solar_mass>(1.9),
        absolute_magnitude: -1.67,
        apparent_magnitude: 3.32,
        temperature: ThermodynamicTemperature::new::<kelvin>(3479.),
        age: Some(Time::new::<gigayear>(0.440)),
        right_ascension: RightAscension::new(3, 5, 11.),
        declination: Declination::new(Sgn::Pos, 38, 50, 25.),
        distance: Length::new::<light_year>(325.),
        lifetime: Time::new::<gigayear>(1.54706939),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [MIRPHAK(), ALGOL(), GORGONEA_TERTIA()] }
