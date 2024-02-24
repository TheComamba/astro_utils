use crate::{
    coordinates::{
        declination::{Declination, Sgn},
        right_ascension::RightAscension,
    },
    stars::{fate::StarFate, real_data::RealData},
    units::{
        distance::{LIGHT_YEAR, SOLAR_RADIUS},
        mass::SOLAR_MASS,
        time::BILLION_YEARS,
    },
};
use simple_si_units::base::{Distance, Mass, Temperature, Time};

const HAMAL: RealData = RealData {
    common_name: "Hamal",
    astronomical_name: "Alpha Arietis",
    constellation: "Aries",
    radius: Some(Distance {
        m: 14.9 * SOLAR_RADIUS.m,
    }),
    mass: Some(Mass {
        kg: 1.5 * SOLAR_MASS.kg,
    }),
    absolute_magnitude: 0.48,
    apparent_magnitude: 2.01,
    temperature: Temperature { K: 4480. },
    age: Some(Time {
        s: 3.4 * BILLION_YEARS.s,
    }),
    right_ascension: RightAscension::new(2, 7, 10),
    declination: Declination::new(Sgn::Pos, 23, 27, 45),
    distance: Distance {
        m: 66. * LIGHT_YEAR.m,
    },
    lifetime: Time {
        s: 7.2 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
};

const BETA_ARIETIS: RealData = RealData {
    common_name: "",
    astronomical_name: "Beta Arietis",
    constellation: "Aries",
    right_ascension: RightAscension::new(1, 54, 38),
    declination: Declination::new(Sgn::Pos, 20, 48, 29),
    apparent_magnitude: 2.655,
    distance: Distance {
        m: 59.6 * LIGHT_YEAR.m,
    },
    absolute_magnitude: 1.55,
    mass: Some(Mass {
        kg: 2.34 * SOLAR_MASS.kg,
    }),
    radius: Some(Distance {
        m: 23. * SOLAR_RADIUS.m,
    }),
    temperature: Temperature { K: 9000. },
    age: Some(Time {
        s: 0.3 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 1.8 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
};

const BHARANI: RealData = RealData {
    common_name: "Bharani",
    astronomical_name: "41 Arietis",
    constellation: "Aries",
    right_ascension: RightAscension::new(2, 49, 59),
    declination: Declination::new(Sgn::Pos, 27, 15, 38),
    apparent_magnitude: 3.63,
    distance: Distance {
        m: 166. * LIGHT_YEAR.m,
    },
    absolute_magnitude: 0.163,
    mass: Some(Mass {
        kg: 3.1 * SOLAR_MASS.kg,
    }),
    radius: None,
    temperature: Temperature { K: 11_900. },
    age: Some(Time {
        s: 0.130 * BILLION_YEARS.s,
    }),
    lifetime: Time {
        s: 0.360 * BILLION_YEARS.s,
    },
    fate: StarFate::WhiteDwarf,
};

pub(crate) const STARS: [RealData; 3] = [HAMAL, BETA_ARIETIS, BHARANI];
