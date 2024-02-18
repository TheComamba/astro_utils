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

const ALPHA_TELESCOPII: RealData = RealData {
    common_name: "",
    astronomical_name: "Alpha Telescopii",
    constellation: "Telescopium",
    right_ascension: RightAscension::new(18, 26, 58),
    declination: Declination::new(Sgn::Neg, 45, 58, 6),
    apparent_magnitude: 3.51,
    distance: Distance {
        m: 278. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.25,
    mass: Some(Mass {
        kg: 5.2 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.3 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 16_700. }),
    age: Some(Time {
        s: 0.0241 * BILLION_YEARS.s,
    }),
};

const ZETA_TELESCOPII: RealData = RealData {
    common_name: "",
    astronomical_name: "Zeta Telescopii",
    constellation: "Telescopium",
    right_ascension: RightAscension::new(18, 28, 50),
    declination: Declination::new(Sgn::Neg, 49, 4, 14),
    apparent_magnitude: 4.13,
    distance: Distance {
        m: 126. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.171,
    mass: Some(Mass {
        kg: 1.53 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 9. * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4801. }),
    age: None,
};

const EPSILON_TELESCOPII: RealData = RealData {
    common_name: "",
    astronomical_name: "Epislon Telescopii",
    constellation: "Telescopium",
    right_ascension: RightAscension::new(18, 11, 14),
    declination: Declination::new(Sgn::Neg, 45, 57, 16),
    apparent_magnitude: 4.50,
    distance: Distance {
        m: 410. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.,
    mass: None,
    radius: None,
    temperature: Some(Temperature { K: 4996. }),
    age: None,
};

pub(crate) const STARS: [RealData; 3] = [ALPHA_TELESCOPII, ZETA_TELESCOPII, EPSILON_TELESCOPII];
