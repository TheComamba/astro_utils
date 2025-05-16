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

fn NAOS() -> RealData {
    RealData {
        common_name: "Naos",
        astronomical_name: "ζ Puppis",
        constellation: "Puppis",
        radius: Some(Length {
            m: 20. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 56.1 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -5.95,
        apparent_magnitude: 2.21,
        temperature: Temperature { K: 40_000. },
        age: Some(Time {
            s: 0.0032 * BILLION_YEARS.s,
        }),
        right_ascension: RightAscension::new(8, 3, 35.),
        declination: Declination::new(Sgn::Neg, 40, 0, 12.),
        distance: Length {
            m: 1399. * LIGHT_YEAR.m,
        },
        lifetime: Time {
            s: 0.00435 * BILLION_YEARS.s,
        },
    }
}

fn AHADI() -> RealData {
    RealData {
        common_name: "Ahadi",
        astronomical_name: "π Puppis",
        constellation: "Puppis",
        radius: Some(Length {
            m: 235. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 11.7 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -4.92,
        apparent_magnitude: 2.71,
        temperature: Temperature { K: 4000. },
        right_ascension: RightAscension::new(7, 17, 9.),
        declination: Declination::new(Sgn::Neg, 37, 5, 51.),
        distance: Length {
            m: 1094. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.019 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.019450199 * BILLION_YEARS.s,
        },
    }
}

fn RHO_PUPPIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ρ Puppis",
        constellation: "Puppis",
        right_ascension: RightAscension::new(8, 7, 33.),
        declination: Declination::new(Sgn::Neg, 24, 18, 16.),
        apparent_magnitude: 2.78,
        distance: Length {
            m: 63.5 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 1.41,
        mass: Mass {
            kg: 1.85 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 3.41 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 6920. },
        age: Some(Time {
            s: 1.5 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.65092742 * BILLION_YEARS.s,
        },
    }
}

fn TAU_PUPPIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "τ Puppis",
        constellation: "Puppis",
        right_ascension: RightAscension::new(6, 49, 56.),
        declination: Declination::new(Sgn::Neg, 50, 36, 52.),
        apparent_magnitude: 2.95,
        distance: Length {
            m: 174. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -0.80,
        mass: Mass {
            kg: 3.19 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 27. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4489. },
        age: Some(Time {
            s: 0.3 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.351318702 * BILLION_YEARS.s,
        },
    }
}

fn NU_PUPPIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ν Puppis",
        constellation: "Puppis",
        right_ascension: RightAscension::new(6, 37, 46.),
        declination: Declination::new(Sgn::Neg, 43, 11, 45.),
        apparent_magnitude: 3.173,
        distance: Length {
            m: 370. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -2.11,
        mass: Mass {
            kg: 5.2 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 4.2 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 12_120. },
        age: None,
        lifetime: Time {
            s: 0.10143918 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 5] = [NAOS, AHADI, RHO_PUPPIS, TAU_PUPPIS, NU_PUPPIS];
