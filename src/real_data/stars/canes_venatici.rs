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

fn COR_CAROLI() -> RealData {
    RealData {
        common_name: "Cor Caroli",
        astronomical_name: "α² Canum Venaticorum",
        constellation: "Canes Venatici",
        radius: Some(Length {
            m: 2.49 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.97 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 0.246,
        apparent_magnitude: 2.89,
        temperature: Temperature { K: 11_600. },
        age: Some(Time {
            s: 0.165 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.42 * BILLION_YEARS.s,
        },

        right_ascension: RightAscension::new(12, 56, 2.),
        declination: Declination::new(Sgn::Pos, 38, 19, 6.),
        distance: Length {
            m: 110.1 * LIGHT_YEAR.m,
        },
    }
}

fn CHARA() -> RealData {
    RealData {
        common_name: "Chara",
        astronomical_name: "β Canum Venaticorum",
        constellation: "Canes Venatici",
        radius: Some(Length {
            m: 1.123 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 0.97 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 4.64,
        apparent_magnitude: 4.25,
        temperature: Temperature { K: 6043. },
        right_ascension: RightAscension::new(12, 33, 45.),
        declination: Declination::new(Sgn::Pos, 41, 21, 27.),
        distance: Length {
            m: 27.63 * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 3.4 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 11.7800188 * BILLION_YEARS.s,
        },
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
        distance: Length {
            m: 180. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.85,
        mass: Mass {
            kg: 1.74 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 1.90 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 8285. },
        age: Some(Time {
            s: 0.360 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.59501327 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 3] = [COR_CAROLI, CHARA, TWENTYFOUR_CANUM_VENATICORUM];
