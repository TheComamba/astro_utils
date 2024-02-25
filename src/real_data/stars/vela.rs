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

const REGOR: RealData = RealData {
    common_name: "Regor",
    astronomical_name: "Gamma Velorum",
    constellation: "Vela",
    radius: Some(Distance {
        m: 17. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 28.5 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -5.31,
    apparent_magnitude: 1.75,
    temperature: Temperature { K: 35_000. },
    age: Some(Time {
        s: 0.0045 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(8, 9, 32),
    declination: Declination::new(Sgn::Neg, 47, 20, 12),
    distance: Distance {
        m: 840. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.006972406 * BILLION_YEARS.s,
    },
};

const ALSEPHINA: RealData = RealData {
    common_name: "Alsephina",
    astronomical_name: "Delta Velorum",
    constellation: "Vela",
    radius: Some(Distance {
        m: 2.4 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.27 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.01,
    apparent_magnitude: 1.93,
    temperature: Temperature { K: 9440. },
    age: Some(Time {
        s: 0.4 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(8, 44, 42),
    declination: Declination::new(Sgn::Neg, 54, 42, 32),
    distance: Distance {
        m: 80. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.964406929 * BILLION_YEARS.s,
    },
};

const SUHAIL: RealData = RealData {
    common_name: "Suhail",
    astronomical_name: "Lambda Velorum",
    constellation: "Vela",
    radius: Some(Distance {
        m: 210. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 7. * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.99,
    apparent_magnitude: 2.23,
    temperature: Temperature { K: 3900. },
    age: Some(Time {
        s: 0.0316 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(9, 7, 60),
    declination: Declination::new(Sgn::Neg, 43, 25, 57),
    distance: Distance {
        m: 573. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.052267043 * BILLION_YEARS.s,
    },
};

const MARKEB: RealData = RealData {
    common_name: "Markeb",
    astronomical_name: "Kappa Velorum",
    constellation: "Vela",
    radius: Some(Distance {
        m: 9.1 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 10.5 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.62,
    apparent_magnitude: 2.47,
    temperature: Temperature { K: 23_000. },
    age: Some(Time {
        s: 0.018 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(9, 22, 7),
    declination: Declination::new(Sgn::Neg, 55, 0, 38),
    distance: Distance {
        m: 539. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.026540021 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 4] = [REGOR, ALSEPHINA, SUHAIL, MARKEB];
