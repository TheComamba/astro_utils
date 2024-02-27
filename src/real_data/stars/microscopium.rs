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

const GAMMA_MICROSCOPII: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Microscopii",
    constellation: "Microscopium",
    right_ascension: RightAscension::new(21, 1, 17),
    declination: Declination::new(Sgn::Neg, 32, 15, 28),
    apparent_magnitude: 4.680,
    distance: Distance {
        m: 223. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.49,
    mass: Mass {
        kg: 2.5 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 10. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 5050. },
    age: Some(Time {
        s: 0.620 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.800458342 * BILLION_YEARS.s,
    },
};

const EPSILON_MICROSCOPII: RealData = RealData {
    common_name: "",
    astronomical_name: "ε Microscopii",
    constellation: "Microscopium",
    right_ascension: RightAscension::new(21, 17, 56),
    declination: Declination::new(Sgn::Neg, 32, 10, 21),
    apparent_magnitude: 4.71,
    distance: Distance {
        m: 166. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.97,
    mass: Mass {
        kg: 2.18 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 2.2 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 9126. },
    age: Some(Time {
        s: 0.525 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.03650581 * BILLION_YEARS.s,
    },
};

const THETA1_MICROSCOPII: RealData = RealData {
    common_name: "",
    astronomical_name: "θ¹ Microscopii",
    constellation: "Microscopium",
    right_ascension: RightAscension::new(21, 20, 46),
    declination: Declination::new(Sgn::Neg, 40, 48, 34),
    apparent_magnitude: 4.82,
    distance: Distance {
        m: 179. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.03,
    mass: Mass {
        kg: 2.32 * SOLAR_MASS.kg,
    },
    radius: Some(Distance {
        m: 2.35 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 9240. },
    age: Some(Time {
        s: 0.437 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.916355612 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] =
    [GAMMA_MICROSCOPII, EPSILON_MICROSCOPII, THETA1_MICROSCOPII];
