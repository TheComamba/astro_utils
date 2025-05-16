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

fn PROCYON() -> RealData {
    RealData {
        common_name: "Procyon",
        astronomical_name: "α Canis Minoris",
        constellation: "Canis Minor",
        radius: Some(Length::new::<solar_radii>(2.048)),
        mass: Mass::new::<solar_mass>(1.499),
        absolute_magnitude: 2.68,
        apparent_magnitude: 0.40,
        temperature: ThermodynamicTemperature::new::<kelvin>(6530.),
        right_ascension: RightAscension::new(7, 39, 18.),
        declination: Declination::new(Sgn::Pos, 5, 13, 30.),
        distance: Length::new::<light_year>(11.),
        age: Some(Time::new::<gigayear>(1.37)),
        lifetime: Time::new::<gigayear>(2.54186931),
    }
}

fn GOMEISA() -> RealData {
    RealData {
        common_name: "Gomeisa",
        astronomical_name: "β Canis Minoris",
        constellation: "Canis Minor",
        right_ascension: RightAscension::new(7, 27, 9.),
        declination: Declination::new(Sgn::Pos, 8, 17, 22.),
        apparent_magnitude: 2.84,
        distance: Length::new::<light_year>(160.),
        absolute_magnitude: -0.59,
        mass: Mass::new::<solar_mass>(3.5),
        radius: Some(Length::new::<solar_radii>(3.5)),
        temperature: ThermodynamicTemperature::new::<kelvin>(11_772.),
        age: Some(Time::new::<gigayear>(0.160)),
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

fn GAMMA_CANIS_MINORIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Canis Minoris",
        constellation: "Canis Minor",
        right_ascension: RightAscension::new(7, 28, 10.),
        declination: Declination::new(Sgn::Pos, 8, 55, 32.),
        apparent_magnitude: 4.33,
        distance: Length::new::<light_year>(320.),
        absolute_magnitude: -0.5,
        mass: Mass::new::<solar_mass>(1.88),
        radius: Some(Length::new::<solar_radii>(36.8)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4036.),
        age: Some(Time::new::<gigayear>(1.3)),
        lifetime: Time::new::<gigayear>(1.54706939),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [PROCYON(), GOMEISA(), GAMMA_CANIS_MINORIS()] }
