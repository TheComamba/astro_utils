use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn spica() -> RealData {
    RealData {
        common_name: "Spica",
        astronomical_name: "α Virginis",
        constellation: "Virgo",
        radius: Some(Length::new::<solar_radius>(7.47)),
        mass: Mass::new::<solar_mass>(11.43),
        absolute_magnitude: -3.55,
        apparent_magnitude: 0.98,
        temperature: ThermodynamicTemperature::new::<kelvin>(22_300.),
        age: Some(Time::new::<gigayear>(0.0125)),
        right_ascension: RightAscension::new(13, 25, 12.),
        declination: Declination::new(Sgn::Neg, 11, 9, 41.),
        distance: Length::new::<light_year>(262.),
        lifetime: Time::new::<gigayear>(0.019450199),
    }
}

fn minelauva() -> RealData {
    RealData {
        common_name: "Minelauva",
        astronomical_name: "δ Virginis",
        constellation: "Virgo",
        radius: Some(Length::new::<solar_radius>(48.)),
        mass: Mass::new::<solar_mass>(1.4),
        absolute_magnitude: -0.575,
        apparent_magnitude: 3.39,
        temperature: ThermodynamicTemperature::new::<kelvin>(3999.),
        age: None,
        right_ascension: RightAscension::new(12, 55, 36.),
        declination: Declination::new(Sgn::Pos, 3, 23, 51.),
        distance: Length::new::<light_year>(202.4),
        lifetime: Time::new::<gigayear>(3.10253119),
    }
}

fn zavijava() -> RealData {
    RealData {
        common_name: "Zavijava",
        astronomical_name: "β Virginis",
        constellation: "Virgo",
        right_ascension: RightAscension::new(11, 50, 42.),
        declination: Declination::new(Sgn::Pos, 1, 45, 53.),
        apparent_magnitude: 3.604,
        distance: Length::new::<light_year>(35.65),
        absolute_magnitude: 3.41,
        mass: Mass::new::<solar_mass>(1.413),
        radius: Some(Length::new::<solar_radius>(1.681)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6132.),
        age: Some(Time::new::<gigayear>(2.9)),
        lifetime: Time::new::<gigayear>(3.10253119),
    }
}

fn syrma() -> RealData {
    RealData {
        common_name: "Syrma",
        astronomical_name: "ι Virginis",
        constellation: "Virgo",
        right_ascension: RightAscension::new(14, 16, 1.),
        declination: Declination::new(Sgn::Neg, 6, 0, 2.),
        apparent_magnitude: 4.08,
        distance: Length::new::<light_year>(72.5),
        absolute_magnitude: 2.4,
        mass: Mass::new::<solar_mass>(1.5),
        radius: Some(Length::new::<solar_radius>(2.5)),
        temperature: ThermodynamicTemperature::new::<kelvin>(6282.),
        age: None,
        lifetime: Time::new::<gigayear>(2.54186931),
    }
}

fn heze() -> RealData {
    RealData {
        common_name: "Heze",
        astronomical_name: "ζ Virginis",
        constellation: "Virgo",
        right_ascension: RightAscension::new(13, 34, 42.),
        declination: Declination::new(Sgn::Neg, 0, 35, 45.),
        apparent_magnitude: 3.376,
        distance: Length::new::<light_year>(74.1),
        absolute_magnitude: 1.64,
        mass: Mass::new::<solar_mass>(2.041),
        radius: Some(Length::new::<solar_radius>(2.079)),
        temperature: ThermodynamicTemperature::new::<kelvin>(8247.),
        age: Some(Time::new::<gigayear>(0.51)),
        lifetime: Time::new::<gigayear>(1.25731981),
    }
}

fn vindemiatrix() -> RealData {
    RealData {
        common_name: "Vindemiatrix",
        astronomical_name: "ε Virginis",
        constellation: "Virgo",
        right_ascension: RightAscension::new(13, 2, 11.),
        declination: Declination::new(Sgn::Pos, 10, 57, 33.),
        apparent_magnitude: 2.826,
        distance: Length::new::<light_year>(109.6),
        absolute_magnitude: 0.37,
        mass: Mass::new::<solar_mass>(2.64),
        radius: Some(Length::new::<solar_radius>(10.6)),
        temperature: ThermodynamicTemperature::new::<kelvin>(5086.),
        age: Some(Time::new::<gigayear>(0.560)),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

fn porrima() -> RealData {
    RealData {
        common_name: "Porrima",
        astronomical_name: "γ Virginis",
        constellation: "Virgo",
        right_ascension: RightAscension::new(12, 41, 40.),
        declination: Declination::new(Sgn::Neg, 1, 26, 58.),
        apparent_magnitude: 2.74,
        distance: Length::new::<light_year>(38.1),
        absolute_magnitude: 2.41,
        mass: Mass::new::<solar_mass>(1.56),
        radius: None,
        temperature: ThermodynamicTemperature::new::<kelvin>(6757.),
        age: Some(Time::new::<gigayear>(1.14)),
        lifetime: Time::new::<gigayear>(2.29668629),
    }
}

pub(crate) fn stars() -> [RealData; 7] {
    [
        spica(),
        minelauva(),
        zavijava(),
        syrma(),
        heze(),
        vindemiatrix(),
        porrima(),
    ]
}
