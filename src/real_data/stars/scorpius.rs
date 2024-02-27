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

const ANTARES: RealData = RealData {
    common_name: "Antares",
    astronomical_name: "α Scorpii",
    constellation: "Scorpius",
    radius: Some(Distance {
        m: 680. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 13.5 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -5.28,
    apparent_magnitude: 1.06,
    temperature: Temperature { K: 3660. },
    age: Some(Time {
        s: 0.015 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(16, 29, 24),
    declination: Declination::new(Sgn::Neg, 26, 25, 55),
    distance: Distance {
        m: 604. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.015362858 * BILLION_YEARS.s,
    },
};

const SHAULA: RealData = RealData {
    common_name: "Shaula",
    astronomical_name: "λ Scorpii",
    constellation: "Scorpius",
    radius: Some(Distance {
        m: 8.8 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 10.4 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -4.8,
    apparent_magnitude: 1.62,
    temperature: Temperature { K: 25_000. },
    right_ascension: RightAscension::new(17, 33, 37),
    declination: Declination::new(Sgn::Neg, 37, 6, 14),
    distance: Distance {
        m: 600. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.026 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.026540021 * BILLION_YEARS.s,
    },
};

const SARGAS: RealData = RealData {
    common_name: "Sargas",
    astronomical_name: "θ Scorpii",
    constellation: "Scorpius",
    radius: Some(Distance {
        m: 26.3 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 3.1 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -2.75,
    apparent_magnitude: 1.86,
    temperature: Temperature { K: 6294. },
    age: None,
    right_ascension: RightAscension::new(17, 37, 19),
    declination: Declination::new(Sgn::Neg, 42, 59, 52),
    distance: Distance {
        m: 272. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.420724107 * BILLION_YEARS.s,
    },
};

const DSCHUBBA: RealData = RealData {
    common_name: "Dschubba",
    astronomical_name: "δ Scorpii",
    constellation: "Scorpius",
    radius: Some(Distance {
        m: 6.7 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 13. * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.16,
    apparent_magnitude: 2.29,
    temperature: Temperature { K: 27_400. },
    age: Some(Time {
        s: 0.0095 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(16, 0, 20),
    declination: Declination::new(Sgn::Neg, 22, 37, 18),
    distance: Distance {
        m: 401.5 * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.019450199 * BILLION_YEARS.s,
    },
};

const LARAWAG: RealData = RealData {
    common_name: "Larawag",
    astronomical_name: "ε Scorpii",
    constellation: "Scorpius",
    radius: Some(Distance {
        m: 12.6 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.24 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 0.78,
    apparent_magnitude: 2.29,
    temperature: Temperature { K: 4560. },
    age: None,
    right_ascension: RightAscension::new(16, 50, 10),
    declination: Declination::new(Sgn::Neg, 34, 17, 36),
    distance: Distance {
        m: 65. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 4.45521207 * BILLION_YEARS.s,
    },
};

const GIRTAB: RealData = RealData {
    common_name: "Girtab",
    astronomical_name: "κ Scorpii",
    constellation: "Scorpius",
    radius: Some(Distance {
        m: 6.8 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 17. * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.38,
    apparent_magnitude: 2.39,
    temperature: Temperature { K: 23_400. },
    right_ascension: RightAscension::new(17, 42, 29),
    declination: Declination::new(Sgn::Neg, 39, 1, 48),
    distance: Distance {
        m: 464. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.012 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.012799766 * BILLION_YEARS.s,
    },
};

const ACRAB: RealData = RealData {
    common_name: "Acrab",
    astronomical_name: "β Scorpii",
    constellation: "Scorpius",
    radius: Some(Distance {
        m: 6.3 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 15.0 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.50,
    apparent_magnitude: 2.56,
    temperature: Temperature { K: 28_000. },
    right_ascension: RightAscension::new(16, 5, 26),
    declination: Declination::new(Sgn::Neg, 19, 48, 20),
    distance: Distance {
        m: 530. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.015 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.015362858 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 7] = [ANTARES, SHAULA, SARGAS, DSCHUBBA, LARAWAG, GIRTAB, ACRAB];
