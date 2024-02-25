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

const ATRIA: RealData = RealData {
    common_name: "Atria",
    astronomical_name: "Alpha Trianguli Australis",
    constellation: "Triangulum Australe",
    radius: Some(Distance {
        m: 143. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 7. * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.62,
    apparent_magnitude: 1.91,
    temperature: Temperature { K: 4150. },
    age: Some(Time {
        s: 0.048 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(16, 48, 40),
    declination: Declination::new(Sgn::Neg, 69, 1, 40),
    distance: Distance {
        m: 415. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.052267043 * BILLION_YEARS.s,
    },
};

const BETA_TRIANGULI_AUSTRALIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Trianguli Australis",
    constellation: "Triangulum Australe",
    right_ascension: RightAscension::new(15, 55, 9),
    declination: Declination::new(Sgn::Neg, 63, 25, 51),
    apparent_magnitude: 2.85,
    distance: Distance {
        m: 40.37 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.37,
    mass: Mass {
        kg: 1.56 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 1.976 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 7171. },
    age: Some(Time {
        s: 0.674 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 2.29668629 * BILLION_YEARS.s,
    },
};

const GAMMA_TRIANGULI_AUSTRALIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Gamma Trianguli Australis",
    constellation: "Triangulum Australe",
    right_ascension: RightAscension::new(15, 18, 55),
    declination: Declination::new(Sgn::Neg, 68, 40, 46),
    apparent_magnitude: 2.87,
    distance: Distance {
        m: 184. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.89,
    mass: Mass {
        kg: 1.99 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 5.86 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 9306. },
    age: Some(Time {
        s: 0.260 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.36020165 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] =
    [ATRIA, BETA_TRIANGULI_AUSTRALIS, GAMMA_TRIANGULI_AUSTRALIS];
