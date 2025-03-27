use astro_coords::ra_and_dec::*;
use simple_si_units::base::{Length, Mass, Temperature, Time};

use crate::{
    stars::real_data::RealData,
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};

const RIGEL: RealData = RealData {
    common_name: "Rigel",
    astronomical_name: "β Orionis",
    constellation: "Orion",
    radius: Some(Length {
        m: 78.9 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 21. * SOLAR_MASS.kg,
    },
    absolute_magnitude: -6.69,
    apparent_magnitude: 0.18,
    temperature: Temperature { K: 12_100. },
    age: Some(Time {
        s: 0.008 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 14, 32.),
    declination: Declination::new(Sgn::Neg, 8, 12, 6.),
    distance: Length {
        m: 773. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.009767659 * BILLION_YEARS.s,
    },
};

const BETELGEUSE: RealData = RealData {
    common_name: "Betelgeuse",
    astronomical_name: "α Orionis",
    constellation: "Orion",
    radius: Some(Length {
        m: 887. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 16.5 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -5.14,
    apparent_magnitude: 0.9,
    temperature: Temperature { K: 3600. },
    right_ascension: RightAscension::new(5, 55, 10.),
    declination: Declination::new(Sgn::Pos, 7, 24, 25.),
    distance: Length {
        m: 522. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.012799766 * BILLION_YEARS.s - 100. * 365.25 * 24. * 60. * 60.,
    }),
    lifetime: Time {
        s: 0.012799766 * BILLION_YEARS.s,
    },
};

const BELLATRIX: RealData = RealData {
    common_name: "Bellatrix",
    astronomical_name: "γ Orionis",
    constellation: "Orion",
    radius: Some(Length {
        m: 5.75 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 7.7 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -2.72,
    apparent_magnitude: 1.64,
    temperature: Temperature { K: 21_800. },
    age: Some(Time {
        s: 0.0252 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 25, 8.),
    declination: Declination::new(Sgn::Pos, 6, 20, 59.),
    distance: Length {
        m: 243. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.040555762 * BILLION_YEARS.s,
    },
};

const ALNILAM: RealData = RealData {
    common_name: "Alnilam",
    astronomical_name: "ε Orionis",
    constellation: "Orion",
    radius: Some(Length {
        m: 42. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 34.6 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -6.38,
    apparent_magnitude: 1.69,
    temperature: Temperature { K: 27_000. },
    right_ascension: RightAscension::new(5, 36, 13.),
    declination: Declination::new(Sgn::Neg, 1, 12, 7.),
    distance: Length {
        m: 1342. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.0057 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.005807621 * BILLION_YEARS.s,
    },
};

const ALNITAK: RealData = RealData {
    common_name: "Alnitak",
    astronomical_name: "ζ Orionis",
    constellation: "Orion",
    radius: Some(Length {
        m: 20. * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 31.0 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -5.26,
    apparent_magnitude: 1.74,
    temperature: Temperature { K: 29_500. },
    right_ascension: RightAscension::new(5, 40, 46.),
    declination: Declination::new(Sgn::Neg, 1, 56, 34.),
    distance: Length {
        m: 817. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.0064 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.006573099 * BILLION_YEARS.s,
    },
};

const SAIPH: RealData = RealData {
    common_name: "Saiph",
    astronomical_name: "κ Orionis",
    constellation: "Orion",
    radius: Some(Length {
        m: 22.2 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 15.5 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -4.65,
    apparent_magnitude: 2.07,
    temperature: Temperature { K: 26_500. },
    age: Some(Time {
        s: 0.0111 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(5, 47, 45.),
    declination: Declination::new(Sgn::Neg, 9, 40, 11.),
    distance: Length {
        m: 721.2 * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 0.012799766 * BILLION_YEARS.s,
    },
};

const MINTAKA: RealData = RealData {
    common_name: "Mintaka",
    astronomical_name: "δ Orionis",
    constellation: "Orion",
    radius: Some(Length {
        m: 16.5 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 24. * SOLAR_MASS.kg,
    },
    absolute_magnitude: -4.99,
    apparent_magnitude: 2.25,
    temperature: Temperature { K: 29_500. },
    right_ascension: RightAscension::new(5, 32, 0.),
    declination: Declination::new(Sgn::Neg, 0, 17, 57.),
    distance: Length {
        m: 916. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.008 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.008063854 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 7] = [
    RIGEL, BETELGEUSE, BELLATRIX, ALNILAM, ALNITAK, SAIPH, MINTAKA,
];
