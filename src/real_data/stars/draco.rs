use astro_coords::ra_and_dec::*;
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::{
    stars::real_data::RealData,
    units::{length::solar_radii, mass::solar_mass, time::gigayear},
};

fn ELTANIN() -> RealData {
    RealData {
        common_name: "Eltanin",
        astronomical_name: "γ Draconis",
        constellation: "Draco",
        radius: Some(Length::new::<solar_radii>(48.15)),
        mass: Mass::new::<solar_mass>(1.72),
        absolute_magnitude: -1.04,
        apparent_magnitude: 2.24,
        temperature: ThermodynamicTemperature::new::<kelvin>(3930.),
        age: None,
        lifetime: Time::new::<gigayear>(1.73766023),
        right_ascension: RightAscension::new(17, 56, 36.),
        declination: Declination::new(Sgn::Pos, 51, 29, 20.),
        distance: Length::new::<light_year>(148.),
    }
}

fn ATHEBYNE() -> RealData {
    RealData {
        common_name: "Athebyne",
        astronomical_name: "η Draconis",
        constellation: "Draco",
        radius: Some(Length::new::<solar_radii>(11.)),
        mass: Mass::new::<solar_mass>(2.55),
        absolute_magnitude: 0.58,
        apparent_magnitude: 2.73,
        temperature: ThermodynamicTemperature::new::<kelvin>(5055.),
        age: Some(Time::new::<gigayear>(0.55)),
        lifetime: Time::new::<gigayear>(0.63513384),
        right_ascension: RightAscension::new(16, 23, 59.),
        declination: Declination::new(Sgn::Pos, 61, 30, 51.),
        distance: Length::new::<light_year>(87.68),
    }
}

fn THETA_DRACONIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "θ Draconis",
        constellation: "Draco",
        right_ascension: RightAscension::new(16, 1, 53.),
        declination: Declination::new(Sgn::Pos, 58, 33, 55.),
        apparent_magnitude: 4.119,
        distance: Length::new::<light_year>(68.6),
        absolute_magnitude: 2.39,
        mass: Mass::new::<solar_mass>(1.53),
        radius: Some(Length::new::<solar_radii>(2.83)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6105.),
        age: Some(Time::new::<gigayear>(2.03)),
        lifetime: Time::new::<gigayear>(2.29668629),
    }
}

fn KAPPA_DRACONIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "κ Draconis",
        constellation: "Draco",
        right_ascension: RightAscension::new(12, 33, 29.),
        declination: Declination::new(Sgn::Pos, 69, 47, 18.),
        apparent_magnitude: 3.82,
        distance: Length::new::<light_year>(460.),
        absolute_magnitude: -1.95,
        mass: Mass::new::<solar_mass>(3.65),
        radius: Some(Length::new::<solar_radii>(5.85)),
        temperature: ThermodynamicTemperature::new::<kelvin>(13_982.),
        age: None,
        lifetime: Time::new::<gigayear>(0.254814649),
    }
}

fn TYL() -> RealData {
    RealData {
        common_name: "Tyl",
        astronomical_name: "ε Draconis",
        constellation: "Draco",
        right_ascension: RightAscension::new(19, 48, 10.),
        declination: Declination::new(Sgn::Pos, 70, 16, 5.),
        apparent_magnitude: 3.9974,
        distance: Length::new::<light_year>(153.),
        absolute_magnitude: 0.71,
        mass: Mass::new::<solar_mass>(2.7),
        radius: Some(Length::new::<solar_radii>(11.15)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4993.),
        age: Some(Time::new::<gigayear>(0.5)),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn GIAUSAR() -> RealData {
    RealData {
        common_name: "Giausar",
        astronomical_name: "λ Draconis",
        constellation: "Draco",
        right_ascension: RightAscension::new(11, 31, 24.),
        declination: Declination::new(Sgn::Pos, 69, 19, 52.),
        apparent_magnitude: 3.85,
        distance: Length::new::<light_year>(333.),
        absolute_magnitude: -1.14,
        mass: Mass::new::<solar_mass>(1.7),
        radius: Some(Length::new::<solar_radii>(71.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(3958.),
        age: None,
        lifetime: Time::new::<gigayear>(1.73766023),
    }
}

fn GRUMIUM() -> RealData {
    RealData {
        common_name: "Grumium",
        astronomical_name: "ξ Draconis",
        constellation: "Draco",
        right_ascension: RightAscension::new(17, 53, 32.),
        declination: Declination::new(Sgn::Pos, 56, 52, 22.),
        apparent_magnitude: 3.75,
        distance: Length::new::<light_year>(112.5),
        absolute_magnitude: 1.06,
        mass: Mass::new::<solar_mass>(1.45),
        radius: Some(Length::new::<solar_radii>(12.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4445.),
        age: None,
        lifetime: Time::new::<gigayear>(2.82957282),
    }
}

fn THUBAN() -> RealData {
    RealData {
        common_name: "Thuban",
        astronomical_name: "α Draconis",
        constellation: "Draco",
        right_ascension: RightAscension::new(14, 4, 23.),
        declination: Declination::new(Sgn::Pos, 64, 22, 33.),
        apparent_magnitude: 3.67,
        distance: Length::new::<light_year>(303.),
        absolute_magnitude: -1.2,
        mass: Mass::new::<solar_mass>(3.186),
        radius: Some(Length::new::<solar_radii>(4.932)),
        temperature: ThermodynamicTemperature::new::<kelvin>(10_225.),
        age: Some(Time::new::<gigayear>(0.280)),
        lifetime: Time::new::<gigayear>(0.351318702),
    }
}

fn CHI_DRACONIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "χ Draconis",
        constellation: "Draco",
        right_ascension: RightAscension::new(18, 21, 3.),
        declination: Declination::new(Sgn::Pos, 72, 43, 58.),
        apparent_magnitude: 3.570,
        distance: Length::new::<light_year>(27.17),
        absolute_magnitude: 4.04,
        mass: Mass::new::<solar_mass>(1.029),
        radius: Some(Length::new::<solar_radii>(1.2)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6150.),
        age: Some(Time::new::<gigayear>(5.3)),
        lifetime: Time::new::<gigayear>(8.24015833),
    }
}

fn EDASICH() -> RealData {
    RealData {
        common_name: "Edasich",
        astronomical_name: "ι Draconis",
        constellation: "Draco",
        right_ascension: RightAscension::new(15, 24, 56.),
        declination: Declination::new(Sgn::Pos, 58, 57, 58.),
        apparent_magnitude: 3.290,
        distance: Length::new::<light_year>(101.2),
        absolute_magnitude: 0.99,
        mass: Mass::new::<solar_mass>(1.56),
        radius: Some(Length::new::<solar_radii>(11.99)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4504.),
        age: Some(Time::new::<gigayear>(2.2)),
        lifetime: Time::new::<gigayear>(2.29668629),
    }
}

fn ALDHIBAH() -> RealData {
    RealData {
        common_name: "Aldhibah",
        astronomical_name: "ζ Draconis",
        constellation: "Draco",
        right_ascension: RightAscension::new(17, 8, 47.),
        declination: Declination::new(Sgn::Pos, 65, 42, 53.),
        apparent_magnitude: 3.17,
        distance: Length::new::<light_year>(330.),
        absolute_magnitude: -1.88,
        mass: Mass::new::<solar_mass>(3.5),
        radius: Some(Length::new::<solar_radii>(2.3)),
        temperature: ThermodynamicTemperature::new::<kelvin>(13_397.),
        age: None,
        lifetime: Time::new::<gigayear>(0.297402042),
    }
}

fn ALTAIS() -> RealData {
    RealData {
        common_name: "Altais",
        astronomical_name: "δ Draconis",
        constellation: "Draco",
        right_ascension: RightAscension::new(19, 12, 33.),
        declination: Declination::new(Sgn::Pos, 67, 39, 42.),
        apparent_magnitude: 3.07,
        distance: Length::new::<light_year>(97.4),
        absolute_magnitude: 0.62,
        mass: Mass::new::<solar_mass>(2.32),
        radius: Some(Length::new::<solar_radii>(11.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4820.),
        age: Some(Time::new::<gigayear>(0.8)),
        lifetime: Time::new::<gigayear>(0.916355612),
    }
}

fn RASTABAN() -> RealData {
    RealData {
        common_name: "Rastaban",
        astronomical_name: "β Draconis",
        constellation: "Draco",
        right_ascension: RightAscension::new(17, 30, 26.),
        declination: Declination::new(Sgn::Pos, 52, 18, 5.),
        apparent_magnitude: 2.79,
        distance: Length::new::<light_year>(380.),
        absolute_magnitude: -2.457,
        mass: Mass::new::<solar_mass>(6.),
        radius: Some(Length::new::<solar_radii>(40.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5160.),
        age: Some(Time::new::<gigayear>(0.062)),
        lifetime: Time::new::<gigayear>(0.073299383),
    }
}

pub(crate) fn stars() -> [RealData; 13] {
    [
        ELTANIN(),
        ATHEBYNE(),
        THETA_DRACONIS(),
        KAPPA_DRACONIS(),
        TYL(),
        GIAUSAR(),
        GRUMIUM(),
        THUBAN(),
        CHI_DRACONIS(),
        EDASICH(),
        ALDHIBAH(),
        ALTAIS(),
        RASTABAN(),
    ]
}
