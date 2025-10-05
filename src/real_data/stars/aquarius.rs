use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

fn sadalsuud() -> RealData {
    RealData {
        common_name: "Sadalsuud",
        astronomical_name: "β Aquarii",
        constellation: "Aquarius",
        radius: Some(Length::new::<solar_radius>(47.88)),
        mass: Mass::new::<solar_mass>(4.97),
        absolute_magnitude: -3.04,
        apparent_magnitude: 2.87,
        temperature: ThermodynamicTemperature::new::<kelvin>(5608.),
        age: Some(Time::new::<gigayear>(0.110)),
        lifetime: Time::new::<gigayear>(0.111),
        right_ascension: RightAscension::new(21, 31, 34.),
        declination: Declination::new(Sgn::Neg, 5, 34, 16.),
        distance: Length::new::<light_year>(540.),
    }
}

fn sadalmelik() -> RealData {
    RealData {
        common_name: "Sadalmelik",
        astronomical_name: "α Aquarii",
        constellation: "Aquarius",
        radius: Some(Length::new::<solar_radius>(53.89)),
        mass: Mass::new::<solar_mass>(5.13),
        absolute_magnitude: -3.882,
        apparent_magnitude: 2.942,
        temperature: ThermodynamicTemperature::new::<kelvin>(5383.),
        right_ascension: RightAscension::new(22, 5, 47.),
        declination: Declination::new(Sgn::Neg, 0, 19, 11.),
        distance: Length::new::<light_year>(758.1),
        age: Some(Time::new::<gigayear>(0.053)),
        lifetime: Time::new::<gigayear>(0.10143918),
    }
}

fn skat() -> RealData {
    RealData {
        common_name: "Skat",
        astronomical_name: "δ Aquarii",
        constellation: "Aquarius",
        radius: Some(Length::new::<solar_radius>(2.4)),
        mass: Mass::new::<solar_mass>(2.51),
        absolute_magnitude: -0.178,
        apparent_magnitude: 3.27,
        temperature: ThermodynamicTemperature::new::<kelvin>(8650.),
        right_ascension: RightAscension::new(22, 54, 39.),
        declination: Declination::new(Sgn::Neg, 15, 49, 15.),
        distance: Length::new::<light_year>(159.5),
        age: Some(Time::new::<gigayear>(0.3)),
        lifetime: Time::new::<gigayear>(0.63513384),
    }
}

pub(crate) fn stars() -> [RealData; 3] {
    [sadalsuud(), sadalmelik(), skat()]
}
