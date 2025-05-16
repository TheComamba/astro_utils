use astro_coords::ra_and_dec::*;
use simple_si_units::base::{Length, Mass, Temperature, Time};

use crate::{
    stars::real_data::RealData,
    units::{
        length::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

fn ALPHA_TELESCOPII() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "α Telescopii",
        constellation: "Telescopium",
        right_ascension: RightAscension::new(18, 26, 58.),
        declination: Declination::new(Sgn::Neg, 45, 58, 6.),
        apparent_magnitude: 3.51,
        distance: Length {
            m: 278. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -1.25,
        mass: Mass {
            kg: 5.2 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 3.3 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 16_700. },
        age: Some(Time {
            s: 0.0241 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.10143918 * BILLION_YEARS.s,
        },
    }
}

fn ZETA_TELESCOPII() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Telescopii",
        constellation: "Telescopium",
        right_ascension: RightAscension::new(18, 28, 50.),
        declination: Declination::new(Sgn::Neg, 49, 4, 14.),
        apparent_magnitude: 4.13,
        distance: Length {
            m: 126. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 1.171,
        mass: Mass {
            kg: 1.53 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 9. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4801. },
        age: None,
        lifetime: Time {
            s: 2.29668629 * BILLION_YEARS.s,
        },
    }
}

fn EPSILON_TELESCOPII() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Telescopii",
        constellation: "Telescopium",
        right_ascension: RightAscension::new(18, 11, 14.),
        declination: Declination::new(Sgn::Neg, 45, 57, 16.),
        apparent_magnitude: 4.50,
        distance: Length {
            m: 410. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -1.,
        mass: Mass {
            kg: 1.1 * SOLAR_MASS.kg,
        },
        radius: None,
        temperature: Temperature { K: 4996. },
        age: None,
        lifetime: Time {
            s: 6.97272616 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [ALPHA_TELESCOPII, ZETA_TELESCOPII, EPSILON_TELESCOPII];
