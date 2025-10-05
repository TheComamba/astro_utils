use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn praecipua() -> RealData {
    RealData {
        common_name: "Praecipua",
        astronomical_name: "46 Leonis Minoris",
        constellation: "Leo Minor",
        right_ascension: RightAscension::new(10, 53, 19.),
        declination: Declination::new(Sgn::Pos, 34, 12, 54.),
        apparent_magnitude: 3.83,
        distance: Length::new::<light_year>(94.9),
        absolute_magnitude: 1.45,
        mass: Mass::new::<solar_mass>(1.69),
        radius: Some(Length::new::<solar_radii>(8.22)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4670.),
        age: Some(Time::new::<gigayear>(1.7)),
        lifetime: Time::new::<gigayear>(1.73766023),
    }
}

fn beta_leonis_minoris() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "β Leonis Minoris",
        constellation: "Leo Minor",
        right_ascension: RightAscension::new(10, 27, 53.),
        declination: Declination::new(Sgn::Pos, 36, 42, 26.),
        apparent_magnitude: 4.21,
        distance: Length::new::<light_year>(154.),
        absolute_magnitude: 0.85,
        mass: Mass::new::<solar_mass>(2.98),
        radius: Some(Length::new::<solar_radii>(9.4)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4097.),
        age: Some(Time::new::<gigayear>(0.4)),
        lifetime: Time::new::<gigayear>(0.420724107),
    }
}

fn twentyfour_leonis_minoris() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "21 Leonis Minoris",
        constellation: "Leo Minor",
        right_ascension: RightAscension::new(10, 7, 26.),
        declination: Declination::new(Sgn::Pos, 35, 14, 41.),
        apparent_magnitude: 4.5,
        distance: Length::new::<light_year>(92.1),
        absolute_magnitude: 2.43,
        mass: Mass::new::<solar_mass>(1.75),
        radius: Some(Length::new::<solar_radii>(1.75)),
        temperature: ThermodynamicTemperature::new::<kelvin>(7839.),
        age: Some(Time::new::<gigayear>(0.390)),
        lifetime: Time::new::<gigayear>(1.59501327),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [
        beta_leonis_minoris(),
        praecipua(),
        twentyfour_leonis_minoris(),
    ]
}
