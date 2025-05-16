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

fn ALPHECCA() -> RealData {
    RealData {
        common_name: "Alphecca",
        astronomical_name: "α Coronae Borealis",
        constellation: "Corona Borealis",
        radius: Some(Length {
            m: 3. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.58 * SOLAR_MASS.kg,
        },
        absolute_magnitude: 0.42,
        apparent_magnitude: 2.22,
        temperature: Temperature { K: 9700. },
        age: Some(Time {
            s: 0.314 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.63513384 * BILLION_YEARS.s,
        },
        right_ascension: RightAscension::new(15, 34, 41.),
        declination: Declination::new(Sgn::Pos, 26, 42, 53.),
        distance: Length {
            m: 75. * LIGHT_YEAR.m,
        },
    }
}

fn NAUSAKAN() -> RealData {
    RealData {
        common_name: "Nausakan",
        astronomical_name: "β Coronae Borealis",
        constellation: "Corona Borealis",
        right_ascension: RightAscension::new(15, 27, 50.),
        declination: Declination::new(Sgn::Pos, 29, 6, 21.),
        apparent_magnitude: 3.7,
        distance: Length {
            m: 112. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.942,
        mass: Mass {
            kg: 2.09 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 2.63 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 7980. },
        age: None,
        lifetime: Time {
            s: 1.17901142 * BILLION_YEARS.s,
        },
    }
}

fn GAMMA_CORONAE_BOREALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "γ Coronae Borealis",
        constellation: "Corona Borealis",
        right_ascension: RightAscension::new(15, 42, 45.),
        declination: Declination::new(Sgn::Pos, 26, 17, 44.),
        apparent_magnitude: 3.80,
        distance: Length {
            m: 146. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.56,
        mass: Mass {
            kg: 2.51 * SOLAR_MASS.kg,
        },
        radius: None,
        temperature: Temperature { K: 7649. },
        age: Some(Time {
            s: 0.4 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.63513384 * BILLION_YEARS.s,
        },
    }
}

fn DELTA_CORONAE_BOREALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Coronae Borealis",
        constellation: "Corona Borealis",
        right_ascension: RightAscension::new(15, 49, 36.),
        declination: Declination::new(Sgn::Pos, 26, 4, 6.),
        apparent_magnitude: 4.57,
        distance: Length {
            m: 165. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 1.18,
        mass: Mass {
            kg: 2.4 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 7.4 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 5180. },
        age: Some(Time {
            s: 0.8 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.800458342 * BILLION_YEARS.s,
        },
    }
}

fn EPSILON_CORONAE_BOREALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ε Coronae Borealis",
        constellation: "Corona Borealis",
        right_ascension: RightAscension::new(15, 57, 35.),
        declination: Declination::new(Sgn::Pos, 26, 52, 40.),
        apparent_magnitude: 4.13,
        distance: Length {
            m: 242. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -0.02,
        mass: Mass {
            kg: 1.44 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 21. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4365. },
        age: Some(Time {
            s: 2.8 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 2.82957282 * BILLION_YEARS.s,
        },
    }
}

fn IOTA_CORONAE_BOREALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ι Coronae Borealis",
        constellation: "Corona Borealis",
        right_ascension: RightAscension::new(16, 1, 27.),
        declination: Declination::new(Sgn::Pos, 29, 51, 4.),
        apparent_magnitude: 4.96,
        distance: Length {
            m: 312. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.08,
        mass: Mass {
            kg: 3. * SOLAR_MASS.kg,
        },
        radius: None,
        temperature: Temperature { K: 10_727. },
        age: None,
        lifetime: Time {
            s: 0.420724107 * BILLION_YEARS.s, //no idea
        },
    }
}

fn THETA_CORONAE_BOREALIS() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "θ Coronae Borealis",
        constellation: "Corona Borealis",
        right_ascension: RightAscension::new(15, 32, 56.),
        declination: Declination::new(Sgn::Pos, 31, 21, 33.),
        apparent_magnitude: 4.1,
        distance: Length {
            m: 380. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -1.16,
        mass: Mass {
            kg: 4.2 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 3.3 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 14_000. },
        age: Some(Time {
            s: 0.085 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.170765802 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 7] = [
    ALPHECCA,
    NAUSAKAN,
    GAMMA_CORONAE_BOREALIS,
    DELTA_CORONAE_BOREALIS,
    EPSILON_CORONAE_BOREALIS,
    IOTA_CORONAE_BOREALIS,
    THETA_CORONAE_BOREALIS,
];
