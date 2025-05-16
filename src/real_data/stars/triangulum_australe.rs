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

fn ATRIA() -> RealData {
    RealData {
        common_name: "Atria",
        astronomical_name: "α Trianguli Australis",
        constellation: "Triangulum Australe",
        radius: Some(Length::new::<solar_radii>(143.)),
        mass: Mass::new::<solar_mass>(7.),
        absolute_magnitude: -3.62,
        apparent_magnitude: 1.91,
        temperature: ThermodynamicTemperature::new::<kelvin>(4150.),
        age: Some(Time::new::<gigayear>(0.048)),
        right_ascension: RightAscension::new(16, 48, 40.),
        declination: Declination::new(Sgn::Neg, 69, 1, 40.),
        distance: Length::new::<light_year>(415.),
        lifetime: Time::new::<gigayear>(0.052267043),
    }
}

fn BETA_TRIANGULI_AUSTRALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Trianguli Australis",
        constellation: "Triangulum Australe",
        right_ascension: RightAscension::new(15, 55, 9.),
        declination: Declination::new(Sgn::Neg, 63, 25, 51.),
        apparent_magnitude: 2.85,
        distance: Length::new::<light_year>(40.37),
        absolute_magnitude: 2.37,
        mass: Mass::new::<solar_mass>(1.56),
        radius: Some(Length::new::<solar_radii>(1.976)),
        temperature: ThermodynamicTemperature::new::<kelvin>(7171.),
        age: Some(Time::new::<gigayear>(0.674)),
        lifetime: Time::new::<gigayear>(2.29668629),
    }
}

fn GAMMA_TRIANGULI_AUSTRALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Trianguli Australis",
        constellation: "Triangulum Australe",
        right_ascension: RightAscension::new(15, 18, 55.),
        declination: Declination::new(Sgn::Neg, 68, 40, 46.),
        apparent_magnitude: 2.87,
        distance: Length::new::<light_year>(184.),
        absolute_magnitude: -0.89,
        mass: Mass::new::<solar_mass>(1.99),
        radius: Some(Length::new::<solar_radii>(5.86)),
        temperature: ThermodynamicTemperature::new::<kelvin>(9306.),
        age: Some(Time::new::<gigayear>(0.260)),
        lifetime: Time::new::<gigayear>(1.36020165),
    }
}

pub(crate) const STARS: [RealData; 3] =
    [ATRIA, BETA_TRIANGULI_AUSTRALIS, GAMMA_TRIANGULI_AUSTRALIS];
