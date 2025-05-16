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

fn COR_CAROLI() -> RealData {
    RealData {
        common_name: "Cor Caroli",
        astronomical_name: "α² Canum Venaticorum",
        constellation: "Canes Venatici",
        radius: Some(Length::new::<solar_radii>(2.49)),
        mass: Mass::new::<solar_mass>(2.97),
        absolute_magnitude: 0.246,
        apparent_magnitude: 2.89,
        temperature: ThermodynamicTemperature::new::<kelvin>(11_600.),
        age: Some(Time::new::<gigayear>(0.165)),
        lifetime: Time::new::<gigayear>(0.42),

        right_ascension: RightAscension::new(12, 56, 2.),
        declination: Declination::new(Sgn::Pos, 38, 19, 6.),
        distance: Length::new::<light_year>(110.1),
    }
}

fn CHARA() -> RealData {
    RealData {
        common_name: "Chara",
        astronomical_name: "β Canum Venaticorum",
        constellation: "Canes Venatici",
        radius: Some(Length::new::<solar_radii>(1.123)),
        mass: Mass::new::<solar_mass>(0.97),
        absolute_magnitude: 4.64,
        apparent_magnitude: 4.25,
        temperature: ThermodynamicTemperature::new::<kelvin>(6043.),
        right_ascension: RightAscension::new(12, 33, 45.),
        declination: Declination::new(Sgn::Pos, 41, 21, 27.),
        distance: Length::new::<light_year>(27.63),
        age: Some(Time::new::<gigayear>(3.4)),
        lifetime: Time::new::<gigayear>(11.7800188),
    }
}

fn TWENTYFOUR_CANUM_VENATICORUM() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "24 Canum Venaticorum",
        constellation: "Canes Venatici",
        right_ascension: RightAscension::new(13, 34, 27.),
        declination: Declination::new(Sgn::Pos, 49, 0, 58.),
        apparent_magnitude: 4.68,
        distance: Length::new::<light_year>(180.),
        absolute_magnitude: 0.85,
        mass: Mass::new::<solar_mass>(1.74),
        radius: Some(Length::new::<solar_radii>(1.90)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8285.),
        age: Some(Time::new::<gigayear>(0.360)),
        lifetime: Time::new::<gigayear>(1.59501327),
    }
}

pub(crate) fn STARS() -> [RealData; 3] { [COR_CAROLI, CHARA, TWENTYFOUR_CANUM_VENATICORUM] }
