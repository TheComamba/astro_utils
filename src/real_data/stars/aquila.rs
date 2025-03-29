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

const ALTAIR: RealData = RealData {
    common_name: "Altair",
    astronomical_name: "α Aquilae",
    constellation: "Aquila",
    radius: Some(Length {
        m: 1.63 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.86 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 2.20,
    apparent_magnitude: 0.76,
    temperature: Temperature { K: 7670. },
    right_ascension: RightAscension::new(19, 50, 47.),
    declination: Declination::new(Sgn::Pos, 8, 52, 6.),
    distance: Length {
        m: 17. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.100 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.65092742 * BILLION_YEARS.s,
    },
};

const TARAZED: RealData = RealData {
    common_name: "Tarazed",
    astronomical_name: "γ Aquilae",
    constellation: "Aquila",
    radius: Some(Length {
        m: 91.82 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 3.51 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.03,
    apparent_magnitude: 2.72,
    temperature: Temperature { K: 4098. },
    right_ascension: RightAscension::new(19, 46, 16.),
    declination: Declination::new(Sgn::Pos, 10, 36, 48.),
    distance: Length {
        m: 460.5 * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.250 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.254814649 * BILLION_YEARS.s,
    },
};

const OKAB: RealData = RealData {
    common_name: "Okab",
    astronomical_name: "ζ Aquilae",
    constellation: "Aquila",
    right_ascension: RightAscension::new(19, 5, 25.),
    declination: Declination::new(Sgn::Pos, 13, 51, 49.),
    apparent_magnitude: 2.983,
    distance: Length {
        m: 83.0 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.96,
    mass: Mass {
        kg: 2.37 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 2.27 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 9620. },
    age: Some(Time {
        s: 0.1 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.800458342 * BILLION_YEARS.s,
    },
};

const THETA_AQUILAE: RealData = RealData {
    common_name: "",
    astronomical_name: "θ Aquilae",
    constellation: "Aquila",
    right_ascension: RightAscension::new(20, 11, 18.),
    declination: Declination::new(Sgn::Neg, 0, 49, 17.),
    apparent_magnitude: 3.26,
    distance: Length {
        m: 286. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -1.39,
    mass: Mass {
        kg: 3.564 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 4.76 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 10_300. },
    age: Some(Time {
        s: 0.0209 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.254814649 * BILLION_YEARS.s,
    },
};

const DELTA_AQUILAE: RealData = RealData {
    common_name: "",
    astronomical_name: "δ Aquilae",
    constellation: "Aquila",
    right_ascension: RightAscension::new(19, 25, 30.),
    declination: Declination::new(Sgn::Pos, 3, 6, 53.),
    apparent_magnitude: 3.365,
    distance: Length {
        m: 50.6 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 2.46,
    mass: Mass {
        kg: 1.65 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 2.04 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 7016. },
    age: None,
    lifetime: Time {
        s: 1.89665739 * BILLION_YEARS.s,
    },
};

const LAMBDA_AQUILAE: RealData = RealData {
    common_name: "",
    astronomical_name: "λ Aquilae",
    constellation: "Aquila",
    right_ascension: RightAscension::new(19, 6, 15.),
    declination: Declination::new(Sgn::Neg, 4, 52, 57.),
    apparent_magnitude: 3.43,
    distance: Length {
        m: 125. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.54,
    mass: Mass {
        kg: 3.1 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 1.9 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 11_780. },
    age: Some(Time {
        s: 0.160 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.420724107 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 6] = [
    ALTAIR,
    TARAZED,
    OKAB,
    THETA_AQUILAE,
    DELTA_AQUILAE,
    LAMBDA_AQUILAE,
];
