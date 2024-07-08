use astro_coords::ra_and_dec::*;
use simple_si_units::base::{Distance, Mass, Temperature, Time};

use crate::{
    stars::real_data::RealData,
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

const ACHERNAR: RealData = RealData {
    common_name: "Achernar",
    astronomical_name: "α Eridani",
    constellation: "Eridanus",
    radius: Some(Distance {
        m: 6.78 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 6.0 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -2.77,
    apparent_magnitude: 0.45,
    temperature: Temperature { K: 14_000. },
    age: Some(Time {
        s: 0.063 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.073299383 * BILLION_YEARS.s,
    },
    right_ascension: RightAscension::new(1, 37, 43.),
    declination: Declination::new(Sgn::Neg, 57, 14, 12.),
    distance: Distance {
        m: 144. * LIGHT_YEAR.m,
    },
};

const ZAURAK: RealData = RealData {
    common_name: "Zaurak",
    astronomical_name: "γ Eridani",
    constellation: "Eridanus",
    radius: Some(Distance {
        m: 80. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.2 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -1.19,
    apparent_magnitude: 2.97,
    temperature: Temperature { K: 3811. },
    right_ascension: RightAscension::new(3, 58, 2.),
    declination: Declination::new(Sgn::Neg, 13, 30, 31.),
    distance: Distance {
        m: 221. * LIGHT_YEAR.m,
    },
    age: None,
    lifetime: Time {
        s: 5.06543331 * BILLION_YEARS.s,
    },
};

const CURSA: RealData = RealData {
    common_name: "Cursa",
    astronomical_name: "β Eridani",
    constellation: "Eridanus",
    right_ascension: RightAscension::new(5, 7, 51.),
    declination: Declination::new(Sgn::Neg, 5, 5, 11.),
    apparent_magnitude: 2.796,
    distance: Distance {
        m: 90. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.59,
    mass: Mass {
        kg: 2. * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 2.4 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8360. },
    age: None,
    lifetime: Time {
        s: 1.36020165 * BILLION_YEARS.s,
    },
};

const ACAMAR: RealData = RealData {
    common_name: "Acamar",
    astronomical_name: "θ Eridani",
    constellation: "Eridanus",
    right_ascension: RightAscension::new(2, 58, 16.),
    declination: Declination::new(Sgn::Neg, 40, 18, 17.),
    apparent_magnitude: 3.18,
    distance: Distance {
        m: 164. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.59,
    mass: Mass {
        kg: 2.6 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 4.85 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 8200. },
    age: None,
    lifetime: Time {
        s: 0.63513384 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 4] = [ACHERNAR, ZAURAK, CURSA, ACAMAR];
