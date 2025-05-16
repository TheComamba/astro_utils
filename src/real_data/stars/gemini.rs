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

fn POLLUX() -> RealData {
    RealData {
        common_name: "Pollux",
        astronomical_name: "β Geminorum",
        constellation: "Gemini",
        radius: Some(Length {
            m: 9.06 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 1.91 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 1.09,
        apparent_magnitude: 1.16,
        temperature: Temperature { K: 4586. },
        age: Some(Time {
            s: 0.724 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.54706939 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(7, 45, 19.),
        declination: Declination::new(Sgn::Pos, 28, 1, 34.),
        distance: Length {
            m: 34. * LIGHT_YEAR.m,
        },
    }
}

fn CASTOR() -> RealData {
    RealData {
        common_name: "Castor",
        astronomical_name: "α Geminorum",
        constellation: "Gemini",
        radius: Some(Length {
            m: 2.089 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.37 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 0.59,
        apparent_magnitude: 1.58,
        temperature: Temperature { K: 10_286. },
        age: Some(Time {
            s: 0.290 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.800458342 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(7, 34, 36.),
        declination: Declination::new(Sgn::Pos, 31, 53, 18.),
        distance: Length {
            m: 52. * LIGHT_YEAR.m,
        },
    }
}

fn ALHENA() -> RealData {
    RealData {
        common_name: "Alhena",
        astronomical_name: "γ Geminorum",
        constellation: "Gemini",
        radius: Some(Length {
            m: 3.3 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.81 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -0.60,
        apparent_magnitude: 1.93,
        temperature: Temperature { K: 9260. },
        age: None,
        lifetime: Time {
            s: 0.513076303 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(6, 37, 43.),
        declination: Declination::new(Sgn::Pos, 16, 23, 57.),
        distance: Length {
            m: 105. * LIGHT_YEAR.m,
        },
    }
}

fn TEJAT() -> RealData {
    RealData {
        common_name: "Tejat",
        astronomical_name: "μ Geminorum",
        constellation: "Gemini",
        radius: Some(Length {
            m: 90. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.1 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -1.42,
        apparent_magnitude: 2.75,
        temperature: Temperature { K: 3460. },
        age: None,
        lifetime: Time {
            s: 1.17901142 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(6, 22, 58.),
        declination: Declination::new(Sgn::Pos, 22, 30, 49.),
        distance: Length {
            m: 230. * LIGHT_YEAR.m,
        },
    }
}

fn PROPUS() -> RealData {
    RealData {
        common_name: "Propus",
        astronomical_name: "η Geminorum",
        constellation: "Gemini",
        radius: Some(Length {
            m: 275. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.5 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -1.84,
        apparent_magnitude: 3.31,
        temperature: Temperature { K: 3502. },
        age: Some(Time {
            s: 0.8 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.800458342 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(6, 14, 53.),
        declination: Declination::new(Sgn::Pos, 22, 30, 24.),
        distance: Length {
            m: 349. * LIGHT_YEAR.m,
        },
    }
}

pub(crate) const STARS: [RealData; 5] = [POLLUX, CASTOR, ALHENA, TEJAT, PROPUS];
