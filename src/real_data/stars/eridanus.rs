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

fn ACHERNAR() -> RealData {
    RealData {
        common_name: "Achernar",
        astronomical_name: "α Eridani",
        constellation: "Eridanus",
        radius: Some(Length::new::<solar_radii>(6.78)),
        mass: Mass::new::<solar_mass>(6.0),
        absolute_magnitude: -2.77,
        apparent_magnitude: 0.45,
        temperature: Temperature { K: 14_000. },
        age: Some(Time::new::<gigayear>(0.063)),
        lifetime: Time::new::<gigayear>(0.073299383),
        right_ascension: RightAscension::new(1, 37, 43.),
        declination: Declination::new(Sgn::Neg, 57, 14, 12.),
        distance: Length::new::<light_year>(144.),
    }
}

fn ZAURAK() -> RealData {
    RealData {
        common_name: "Zaurak",
        astronomical_name: "γ Eridani",
        constellation: "Eridanus",
        radius: Some(Length::new::<solar_radii>(80.)),
        mass: Mass::new::<solar_mass>(1.2),
        absolute_magnitude: -1.19,
        apparent_magnitude: 2.97,
        temperature: ThermodynamicTemperature::new::<kelvin>(3811.),
        right_ascension: RightAscension::new(3, 58, 2.),
        declination: Declination::new(Sgn::Neg, 13, 30, 31.),
        distance: Length::new::<light_year>(221.),
        age: None,
        lifetime: Time::new::<gigayear>(5.06543331),
    }
}

fn CURSA() -> RealData {
    RealData {
        common_name: "Cursa",
        astronomical_name: "β Eridani",
        constellation: "Eridanus",
        right_ascension: RightAscension::new(5, 7, 51.),
        declination: Declination::new(Sgn::Neg, 5, 5, 11.),
        apparent_magnitude: 2.796,
        distance: Length::new::<light_year>(90.),
        absolute_magnitude: 0.59,
        mass: Mass::new::<solar_mass>(2.),
        radius: Some(Length::new::<solar_radii>(2.4)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8360.),
        age: None,
        lifetime: Time::new::<gigayear>(1.36020165),
    }
}

fn ACAMAR() -> RealData {
    RealData {
        common_name: "Acamar",
        astronomical_name: "θ Eridani",
        constellation: "Eridanus",
        right_ascension: RightAscension::new(2, 58, 16.),
        declination: Declination::new(Sgn::Neg, 40, 18, 17.),
        apparent_magnitude: 3.18,
        distance: Length::new::<light_year>(164.),
        absolute_magnitude: -0.59,
        mass: Mass::new::<solar_mass>(2.6),
        radius: Some(Length::new::<solar_radii>(4.85)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8200.),
        age: None,
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

pub(crate) const STARS: [RealData; 4] = [ACHERNAR, ZAURAK, CURSA, ACAMAR];
