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

fn ALDERAMIN() -> RealData {
    RealData {
        common_name: "Alderamin",
        astronomical_name: "α Cephei",
        constellation: "Cepheus",
        radius: Some(Length::new::<solar_radii>(2.4)),
        mass: Mass::new::<solar_mass>(2.),
        absolute_magnitude: 1.58,
        apparent_magnitude: 2.45,
        temperature: ThermodynamicTemperature::new::<kelvin>(7700.),
        right_ascension: RightAscension::new(21, 18, 35.),
        declination: Declination::new(Sgn::Pos, 62, 35, 8.),
        distance: Length::new::<light_year>(49.),
        age: Some(Time::new::<gigayear>(0.82)),
        lifetime: Time::new::<gigayear>(1.36020165),
    }
}

fn ALFIRK() -> RealData {
    RealData {
        common_name: "Alfirk",
        astronomical_name: "β Cephei",
        constellation: "Cepheus",
        right_ascension: RightAscension::new(21, 28, 40.),
        declination: Declination::new(Sgn::Pos, 70, 33, 39.),
        apparent_magnitude: 3.23,
        distance: Length::new::<light_year>(594.9),
        absolute_magnitude: -3.08,
        mass: Mass::new::<solar_mass>(7.4),
        radius: Some(Length::new::<solar_radii>(5.6)),
        temperature: ThermodynamicTemperature::new::<kelvin>(27_000.),
        age: Some(Time::new::<gigayear>(0.0087)),
        lifetime: Time::new::<gigayear>(0.052267043),
    }
}

fn ERRAI() -> RealData {
    RealData {
        common_name: "Errai",
        astronomical_name: "γ Cephei",
        constellation: "Cepheus",
        right_ascension: RightAscension::new(23, 39, 21.),
        declination: Declination::new(Sgn::Pos, 77, 37, 57.),
        apparent_magnitude: 3.21,
        distance: Length::new::<light_year>(44.98),
        absolute_magnitude: 2.62,
        mass: Mass::new::<solar_mass>(1.294),
        radius: Some(Length::new::<solar_radii>(4.93)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4792.),
        age: Some(Time::new::<gigayear>(3.25)),
        lifetime: Time::new::<gigayear>(3.9126515),
    }
}

fn DELTA_CEPHERI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Cephei",
        constellation: "Cepheus",
        right_ascension: RightAscension::new(22, 29, 10.),
        declination: Declination::new(Sgn::Pos, 58, 24, 55.),
        apparent_magnitude: 4.07,
        distance: Length::new::<light_year>(981.9),
        absolute_magnitude: -3.32,
        mass: Mass::new::<solar_mass>(4.5),
        radius: Some(Length::new::<solar_radii>(44.5)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6000.),
        age: Some(Time::new::<gigayear>(0.079)),
        lifetime: Time::new::<gigayear>(0.151849866),
    }
}

fn ETA_CEPHEI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "η Cephei",
        constellation: "Cepheus",
        right_ascension: RightAscension::new(20, 45, 17.),
        declination: Declination::new(Sgn::Pos, 61, 50, 20.),
        apparent_magnitude: 3.426,
        distance: Length::new::<light_year>(46.53),
        absolute_magnitude: 2.631,
        mass: Mass::new::<solar_mass>(1.6),
        radius: Some(Length::new::<solar_radii>(4.12)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4950.),
        age: Some(Time::new::<gigayear>(1.9)),
        lifetime: Time::new::<gigayear>(2.08398753),
    }
}

fn IOTA_CEPHEI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ι Cephei",
        constellation: "Cepheus",
        right_ascension: RightAscension::new(22, 49, 41.),
        declination: Declination::new(Sgn::Pos, 66, 12, 1.),
        apparent_magnitude: 3.507,
        distance: Length::new::<light_year>(115.3),
        absolute_magnitude: 0.76,
        mass: Mass::new::<solar_mass>(2.15),
        radius: Some(Length::new::<solar_radii>(11.08)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4768.),
        age: Some(Time::new::<gigayear>(1.0)),
        lifetime: Time::new::<gigayear>(1.09929685),
    }
}

fn ZETA_CEPHEI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Cephei",
        constellation: "Cepheus",
        right_ascension: RightAscension::new(22, 10, 51.),
        declination: Declination::new(Sgn::Pos, 58, 12, 5.),
        apparent_magnitude: 3.39,
        distance: Length::new::<light_year>(726.1),
        absolute_magnitude: -3.35,
        mass: Mass::new::<solar_mass>(7.9),
        radius: Some(Length::new::<solar_radii>(94.)),
        temperature: ThermodynamicTemperature::new::<kelvin>(4072.),
        age: None,
        lifetime: Time::new::<gigayear>(0.040555762),
    }
}

fn ERAKIS() -> RealData {
    RealData {
        common_name: "Erakis",
        astronomical_name: "μ Cephei",
        constellation: "Cepheus",
        radius: Some(Length::new::<solar_radii>(972.)),
        mass: Mass::new::<solar_mass>(19.2),
        absolute_magnitude: -6.5,
        apparent_magnitude: 3.43,
        temperature: ThermodynamicTemperature::new::<kelvin>(3551.),
        right_ascension: RightAscension::new(21, 43, 30.),
        declination: Declination::new(Sgn::Pos, 58, 46, 48.),
        distance: Length::new::<light_year>(3066.),
        age: Some(Time::new::<gigayear>(0.0097)),
        lifetime: Time::new::<gigayear>(0.009767659),
    }
}

pub(crate) fn stars() -> [RealData; 8] {
    [
        ALDERAMIN(),
        ALFIRK(),
        ERRAI(),
        DELTA_CEPHERI(),
        ETA_CEPHEI(),
        IOTA_CEPHEI(),
        ZETA_CEPHEI(),
        ERAKIS(),
    ]
}
