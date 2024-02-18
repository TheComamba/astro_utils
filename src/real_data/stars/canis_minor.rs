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

const PROCYON: RealData = RealData {
    common_name: "Procyon",
    astronomical_name: "Alpha Canis Minoris",
    constellation: "Canis Minor",
    radius: Some(Distance {
        m: 2.048 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.499 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 2.68,
    apparent_magnitude: 0.40,
    temperature: Some(Temperature { K: 6530. }),
    age: Some(Time {
        s: 1.37 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(7, 39, 18),
    declination: Declination::new(Sgn::Pos, 5, 13, 30),
    distance: Distance {
        m: 11. * LIGHT_YEAR.m,
    },
};

const BETA_CANIS_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Canis Minoris",
    constellation: "Canis Minor",
    right_ascension: RightAscension::new(7, 27, 9),
    declination: Declination::new(Sgn::Pos, 8, 17, 22),
    apparent_magnitude: 2.84,
    distance: Distance {
        m: 160. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.59,
    mass: Some(Mass {
        kg: 3.5 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 3.5 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 11_772. }),
    age: Some(Time {
        s: 160. * BILLION_YEARS.s,
    }),
};

const GAMMA_CANIS_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Canis Minoris",
    constellation: "Canis Minor",
    right_ascension: RightAscension::new(7, 28, 10),
    declination: Declination::new(Sgn::Pos, 8, 55, 32),
    apparent_magnitude: 4.33,
    distance: Distance {
        m: 320. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.5,
    mass: Some(Mass {
        kg: 1.88 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 36.8 * SOLAR_RADIUS.m,
    }),
    temperature: Some(Temperature { K: 4036. }),
    age: Some(Time {
        s: 1.3 * BILLION_YEARS.s,
    }),
};

pub(crate) const STARS: [RealData; 3] = [PROCYON, BETA_CANIS_MINORIS, GAMMA_CANIS_MINORIS];
