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

const PROCYON: RealData = RealData {
    common_name: "Procyon",
    astronomical_name: "α Canis Minoris",
    constellation: "Canis Minor",
    radius: Some(Length {
        m: 2.048 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 1.499 * SOLAR_MASS.kg,
    },
    absolute_magnitude: 2.68,
    apparent_magnitude: 0.40,
    temperature: Temperature { K: 6530. },
    right_ascension: RightAscension::new(7, 39, 18.),
    declination: Declination::new(Sgn::Pos, 5, 13, 30.),
    distance: Length {
        m: 11. * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 1.37 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 2.54186931 * BILLION_YEARS.s,
    },
};

const GOMEISA: RealData = RealData {
    common_name: "Gomeisa",
    astronomical_name: "β Canis Minoris",
    constellation: "Canis Minor",
    right_ascension: RightAscension::new(7, 27, 9.),
    declination: Declination::new(Sgn::Pos, 8, 17, 22.),
    apparent_magnitude: 2.84,
    distance: Length {
        m: 160. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.59,
    mass: Mass {
        kg: 3.5 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 3.5 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 11_772. },
    age: Some(Time {
        s: 0.160 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.297402042 * BILLION_YEARS.s,
    },
};

const GAMMA_CANIS_MINORIS: RealData = RealData {
    common_name: "",
    astronomical_name: "γ Canis Minoris",
    constellation: "Canis Minor",
    right_ascension: RightAscension::new(7, 28, 10.),
    declination: Declination::new(Sgn::Pos, 8, 55, 32.),
    apparent_magnitude: 4.33,
    distance: Length {
        m: 320. * LIGHT_YEAR.m,
    },
    absolute_magnitude: -0.5,
    mass: Mass {
        kg: 1.88 * SOLAR_MASS.kg,
    },
    radius: Some(Length {
        m: 36.8 * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 4036. },
    age: Some(Time {
        s: 1.3 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.54706939 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [PROCYON, GOMEISA, GAMMA_CANIS_MINORIS];
