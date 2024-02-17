use crate::{
    coordinates::{
        declination::{Declination, Sgn},
        right_ascension::RightAscension,
    },
    stars::real_data::RealData,
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};
use simple_si_units::base::{Distance, Mass, Temperature, Time};

const COR_CAROLI: RealData = RealData {
    common_name: "Cor Caroli",
    astronomical_name: "Alpha Canum Venaticorum",
    constellation: "Canes Venatici",
    radius: Some(Distance {
        m: 2.49 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 2.97 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.16,
    apparent_magnitude: 2.9,
    temperature: Some(Temperature { K: 11_600. }),
    age: Some(Time {
        s: 0.165 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 56, 2),
    declination: Declination::new(Sgn::Pos, 38, 19, 6),
    distance: Distance {
        m: 100. * LIGHT_YEAR.m,
    },
};

const CHARA: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Canum Venaticorum",
    constellation: "Canes Venatici",
    radius: Some(Distance {
        m: 1.123 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 0.97 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 4.64,
    apparent_magnitude: 4.25,
    temperature: Some(Temperature { K: 6043. }),
    age: Some(Time {
        s: 3.4 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(12, 33, 45),
    declination: Declination::new(Sgn::Pos, 41, 21, 27),
    distance: Distance {
        m: 27.63 * LIGHT_YEAR.m,
    },
};

const TWENTYFOUR_CANUM_VENATICORUM: RealData = RealData {
    common_name: "",
    astronomical_name: "24 Canum Venaticorum",
    constellation: "Canes Venatici",
    right_ascension: RightAscension::new(13, 34, 27),
    declination: Declination::new(Sgn::Pos, 49, 0, 58),
    apparent_magnitude: 4.68,
    distance: Distance {
        m: 180. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.85,
    mass: Some(Mass {
        kg: 1.74 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 1.90 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 8285. }),
    age: Some(Time {
        s: 0.360 * BILLION_YEARS.s,
    }),
};
pub(crate) const STARS: [RealData; 3] = [COR_CAROLI, CHARA, TWENTYFOUR_CANUM_VENATICORUM];
