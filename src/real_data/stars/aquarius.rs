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

const SADALSUUD: RealData = RealData {
    common_name: "Sadalsuud",
    astronomical_name: "β Aquarii",
    constellation: "Aquarius",
    radius: Some(Length {
        m: 47.88 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 4.97 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.04,
    apparent_magnitude: 2.87,
    temperature: Temperature { K: 5608. },
    age: Some(Time {
        s: 0.110 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.111 * BILLION_YEARS.s,
    },
    right_ascension: RightAscension::new(21, 31, 34.),
    declination: Declination::new(Sgn::Neg, 5, 34, 16.),
    distance: Length {
        m: 540. * LIGHT_YEAR.m,
    },
};

const SADALMELIK: RealData = RealData {
    common_name: "Sadalmelik",
    astronomical_name: "α Aquarii",
    constellation: "Aquarius",
    radius: Some(Length {
        m: 53.89 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 5.13 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -3.882,
    apparent_magnitude: 2.942,
    temperature: Temperature { K: 5383. },
    right_ascension: RightAscension::new(22, 5, 47.),
    declination: Declination::new(Sgn::Neg, 0, 19, 11.),
    distance: Length {
        m: 758.1 * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.053 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.10143918 * BILLION_YEARS.s,
    },
};

const SKAT: RealData = RealData {
    common_name: "Skat",
    astronomical_name: "δ Aquarii",
    constellation: "Aquarius",
    radius: Some(Length {
        m: 2.4 * SOLAR_RADIUS.m,
    }),
    mass: Mass {
        kg: 2.51 * SOLAR_MASS.kg,
    },
    absolute_magnitude: -0.178,
    apparent_magnitude: 3.27,
    temperature: Temperature { K: 8650. },
    right_ascension: RightAscension::new(22, 54, 39.),
    declination: Declination::new(Sgn::Neg, 15, 49, 15.),
    distance: Length {
        m: 159.5 * LIGHT_YEAR.m,
    },
    age: Some(Time {
        s: 0.3 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.63513384 * BILLION_YEARS.s,
    },
};

pub(crate) const STARS: [RealData; 3] = [SADALSUUD, SADALMELIK, SKAT];
