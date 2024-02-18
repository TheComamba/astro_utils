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

const GAMMA1_VOLANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma1 Volantis",
    constellation: "Volans",
    right_ascension: RightAscension::new(7, 8, 42),
    declination: Declination::new(Sgn::Neg, 70, 29, 50),
    apparent_magnitude: 5.704,
    distance: Distance {
        m: 143. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.51,
    mass: Some(Mass {
        kg: 1.69 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 6541. }),
    age: Some(Time {
        s: 1.4 * BILLION_YEARS.s,
    }),
};

const BETA_VOLANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Volantis",
    constellation: "Volans",
    right_ascension: RightAscension::new(8, 25, 44),
    declination: Declination::new(Sgn::Neg, 66, 8, 13),
    apparent_magnitude: 3.75,
    distance: Distance {
        m: 107.5 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.18,
    mass: Some(Mass {
        kg: 1.62 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Some(Temperature { K: 4546. }),
    age: None,
};

const ZETA_VOLANTIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeta Volantis",
    constellation: "Volans",
    right_ascension: RightAscension::new(7, 41, 49),
    declination: Declination::new(Sgn::Neg, 72, 36, 22),
    apparent_magnitude: 3.93,
    distance: Distance {
        m: 141. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.75,
    mass: Some(Mass {
        kg: 1.74 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 11. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4721. }),
    age: Some(Time {
        s: 5.27 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 3] = [GAMMA1_VOLANTIS, BETA_VOLANTIS, ZETA_VOLANTIS];
