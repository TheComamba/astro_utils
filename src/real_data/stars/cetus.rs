use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn diphda() -> RealData {
    RealData {
        common_name: "Diphda",
        astronomical_name: "β Ceti",
        constellation: "Cetus",
        radius: Some(Length::new::<solar_radius>(16.78)),
        mass: Mass::new::<solar_mass>(2.8),
        absolute_magnitude: -0.30,
        apparent_magnitude: 2.04,
        temperature: ThermodynamicTemperature::new::<kelvin>(4797.),
        right_ascension: RightAscension::new(0, 43, 35.),
        declination: Declination::new(Sgn::Neg, 17, 59, 12.),
        distance: Length::new::<light_year>(96.),
        age: Some(Time::new::<gigayear>(0.4)),
        lifetime: Time::new::<gigayear>(0.513076303),
    }
}

fn menkar() -> RealData {
    RealData {
        common_name: "Menkar",
        astronomical_name: "α Ceti",
        constellation: "Cetus",
        radius: Some(Length::new::<solar_radius>(89.)),
        mass: Mass::new::<solar_mass>(2.3),
        absolute_magnitude: -1.61,
        apparent_magnitude: 2.54,
        temperature: ThermodynamicTemperature::new::<kelvin>(3795.),
        right_ascension: RightAscension::new(3, 2, 17.),
        declination: Declination::new(Sgn::Pos, 4, 5, 23.),
        distance: Length::new::<light_year>(220.),
        age: None,
        lifetime: Time::new::<gigayear>(0.916355612),
    }
}

fn mira() -> RealData {
    RealData {
        common_name: "Mira",
        astronomical_name: "ο Ceti",
        constellation: "Cetus",
        right_ascension: RightAscension::new(2, 19, 21.),
        declination: Declination::new(Sgn::Neg, 2, 58, 39.),
        apparent_magnitude: 6.47,
        distance: Length::new::<light_year>(418.5),
        absolute_magnitude: 0.928,
        mass: Mass::new::<solar_mass>(1.18),
        radius: Some(Length::new::<solar_radius>(350.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(3000.),
        age: Some(Time::new::<gigayear>(4.5)),
        lifetime: Time::new::<gigayear>(5.06543331),
    }
}

fn baten_kaitos() -> RealData {
    RealData {
        common_name: "Baten Kaitos",
        astronomical_name: "ζ Ceti",
        constellation: "Cetus",
        right_ascension: RightAscension::new(1, 51, 28.),
        declination: Declination::new(Sgn::Neg, 10, 20, 6.),
        apparent_magnitude: 3.742,
        distance: Length::new::<light_year>(235.),
        absolute_magnitude: -0.54,
        mass: Mass::new::<solar_mass>(2.34),
        radius: Some(Length::new::<solar_radius>(25.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4581.),
        age: Some(Time::new::<gigayear>(0.9)),
        lifetime: Time::new::<gigayear>(0.916355612),
    }
}

fn kaffaljidhma() -> RealData {
    RealData {
        common_name: "Kaffaljidhma",
        astronomical_name: "γ Ceti",
        constellation: "Cetus",
        right_ascension: RightAscension::new(2, 43, 18.),
        declination: Declination::new(Sgn::Pos, 3, 14, 9.),
        apparent_magnitude: 3.47,
        distance: Length::new::<light_year>(80.),
        absolute_magnitude: 1.53,
        mass: Mass::new::<solar_mass>(1.88),
        radius: Some(Length::new::<solar_radius>(1.9)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8551.),
        age: Some(Time::new::<gigayear>(0.647)),
        lifetime: Time::new::<gigayear>(1.54706939),
    }
}

fn iota_ceti() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ι Ceti",
        constellation: "Cetus",
        right_ascension: RightAscension::new(0, 19, 26.),
        declination: Declination::new(Sgn::Neg, 8, 49, 26.),
        apparent_magnitude: 3.562,
        distance: Length::new::<light_year>(275.),
        absolute_magnitude: -1.2,
        mass: Mass::new::<solar_mass>(2.78),
        radius: Some(Length::new::<solar_radius>(34.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4446.),
        age: Some(Time::new::<gigayear>(0.5)),
        lifetime: Time::new::<gigayear>(0.513076303),
    }
}

fn deneb_algenubi() -> RealData {
    RealData {
        common_name: "Deneb Algenubi",
        astronomical_name: "η Ceti",
        constellation: "Cetus",
        right_ascension: RightAscension::new(1, 8, 35.),
        declination: Declination::new(Sgn::Neg, 10, 10, 56.),
        apparent_magnitude: 3.446,
        distance: Length::new::<light_year>(123.9),
        absolute_magnitude: 0.68,
        mass: Mass::new::<solar_mass>(1.84),
        radius: Some(Length::new::<solar_radius>(15.10)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4543.),
        age: Some(Time::new::<gigayear>(1.6)),
        lifetime: Time::new::<gigayear>(1.65092742),
    }
}

fn tau_ceti() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "τ Ceti",
        constellation: "Cetus",
        right_ascension: RightAscension::new(1, 44, 4.),
        declination: Declination::new(Sgn::Neg, 15, 56, 15.),
        apparent_magnitude: 3.50,
        distance: Length::new::<light_year>(11.912),
        absolute_magnitude: 5.69,
        mass: Mass::new::<solar_mass>(0.783),
        radius: Some(Length::new::<solar_radius>(0.793)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5320.),
        age: Some(Time::new::<gigayear>(9.)),
        lifetime: Time::new::<gigayear>(21.4199307),
    }
}

pub(crate) fn stars() -> [RealData; 8] {
    [
        diphda(),
        menkar(),
        mira(),
        baten_kaitos(),
        kaffaljidhma(),
        iota_ceti(),
        deneb_algenubi(),
        tau_ceti(),
    ]
}
