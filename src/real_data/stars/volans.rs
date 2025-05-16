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

fn GAMMA1_VOLANTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ¹ Volantis",
        constellation: "Volans",
        right_ascension: RightAscension::new(7, 8, 42.),
        declination: Declination::new(Sgn::Neg, 70, 29, 50.),
        apparent_magnitude: 5.704,
        distance: Length::new::<light_year>(143.),
        absolute_magnitude: 2.51,
        mass: Mass::new::<solar_mass>(1.69),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(6541.),
        age: Some(Time::new::<gigayear>(1.4)),
        lifetime: Time::new::<gigayear>(1.73766023),
    }
}

fn BETA_VOLANTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Volantis",
        constellation: "Volans",
        right_ascension: RightAscension::new(8, 25, 44.),
        declination: Declination::new(Sgn::Neg, 66, 8, 13.),
        apparent_magnitude: 3.75,
        distance: Length::new::<light_year>(107.5),
        absolute_magnitude: 1.18,
        mass: Mass::new::<solar_mass>(1.62),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(4546.),
        age: None,
        lifetime: Time::new::<gigayear>(2.08398753),
    }
}

fn ZETA_VOLANTIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Volantis",
        constellation: "Volans",
        right_ascension: RightAscension::new(7, 41, 49.),
        declination: Declination::new(Sgn::Neg, 72, 36, 22.),
        apparent_magnitude: 3.93,
        distance: Length::new::<light_year>(141.),
        absolute_magnitude: 0.75,
        mass: Mass::new::<solar_mass>(1.74),
        radius: Some(Length::new::<solar_radii>(11.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4721.),
        age: Some(Time::new::<gigayear>(1.5)),
        lifetime: Time::new::<gigayear>(1.59501327),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [GAMMA1_VOLANTIS(), BETA_VOLANTIS(), ZETA_VOLANTIS()] }
