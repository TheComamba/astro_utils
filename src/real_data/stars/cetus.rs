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

fn DIPHDA() -> RealData {
    RealData {
        common_name: "Diphda",
        astronomical_name: "β Ceti",
        constellation: "Cetus",
        radius: Some(Length {
            m: 16.78 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.8 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -0.30,
        apparent_magnitude: 2.04,
        temperature: Temperature { K: 4797. },
        right_ascension: RightAscension::new(0, 43, 35.),
        declination: Declination::new(Sgn::Neg, 17, 59, 12.),
        distance: Length {
            m: 96. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.4 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.513076303 * BILLION_YEARS.s,
        },
    }
}

fn MENKAR() -> RealData {
    RealData {
        common_name: "Menkar",
        astronomical_name: "α Ceti",
        constellation: "Cetus",
        radius: Some(Length {
            m: 89. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2.3 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -1.61,
        apparent_magnitude: 2.54,
        temperature: Temperature { K: 3795. },
        right_ascension: RightAscension::new(3, 2, 17.),
        declination: Declination::new(Sgn::Pos, 4, 5, 23.),
        distance: Length {
            m: 220. * LIGHT_YEAR.m,
        },
        age: None,
        lifetime: Time {
            s: 0.916355612 * BILLION_YEARS.s,
        },
    }
}

fn MIRA() -> RealData {
    RealData {
        common_name: "Mira",
        astronomical_name: "ο Ceti",
        constellation: "Cetus",
        right_ascension: RightAscension::new(2, 19, 21.),
        declination: Declination::new(Sgn::Neg, 2, 58, 39.),
        apparent_magnitude: 6.47,
        distance: Length {
            m: 418.5 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.928,
        mass: Mass {
            kg: 1.18 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 350. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 3000. },
        age: Some(Time {
            s: 4.5 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 5.06543331 * BILLION_YEARS.s,
        },
    }
}

fn BATEN_KAITOS() -> RealData {
    RealData {
        common_name: "Baten Kaitos",
        astronomical_name: "ζ Ceti",
        constellation: "Cetus",
        right_ascension: RightAscension::new(1, 51, 28.),
        declination: Declination::new(Sgn::Neg, 10, 20, 6.),
        apparent_magnitude: 3.742,
        distance: Length {
            m: 235. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -0.54,
        mass: Mass {
            kg: 2.34 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 25. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4581. },
        age: Some(Time {
            s: 0.9 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.916355612 * BILLION_YEARS.s,
        },
    }
}

fn KAFFALJIDHMA() -> RealData {
    RealData {
        common_name: "Kaffaljidhma",
        astronomical_name: "γ Ceti",
        constellation: "Cetus",
        right_ascension: RightAscension::new(2, 43, 18.),
        declination: Declination::new(Sgn::Pos, 3, 14, 9.),
        apparent_magnitude: 3.47,
        distance: Length {
            m: 80. * LIGHT_YEAR.m,
        },
        absolute_magnitude: 1.53,
        mass: Mass {
            kg: 1.88 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 1.9 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 8551. },
        age: Some(Time {
            s: 0.647 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.54706939 * BILLION_YEARS.s,
        },
    }
}

fn IOTA_CETI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ι Ceti",
        constellation: "Cetus",
        right_ascension: RightAscension::new(0, 19, 26.),
        declination: Declination::new(Sgn::Neg, 8, 49, 26.),
        apparent_magnitude: 3.562,
        distance: Length {
            m: 275. * LIGHT_YEAR.m,
        },
        absolute_magnitude: -1.2,
        mass: Mass {
            kg: 2.78 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 34. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4446. },
        age: Some(Time {
            s: 0.5 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.513076303 * BILLION_YEARS.s,
        },
    }
}

fn DENEB_ALGENUBI() -> RealData {
    RealData {
        common_name: "Deneb Algenubi",
        astronomical_name: "η Ceti",
        constellation: "Cetus",
        right_ascension: RightAscension::new(1, 8, 35.),
        declination: Declination::new(Sgn::Neg, 10, 10, 56.),
        apparent_magnitude: 3.446,
        distance: Length {
            m: 123.9 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.68,
        mass: Mass {
            kg: 1.84 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 15.10 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4543. },
        age: Some(Time {
            s: 1.6 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.65092742 * BILLION_YEARS.s,
        },
    }
}

fn TAU_CETI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "τ Ceti",
        constellation: "Cetus",
        right_ascension: RightAscension::new(1, 44, 4.),
        declination: Declination::new(Sgn::Neg, 15, 56, 15.),
        apparent_magnitude: 3.50,
        distance: Length {
            m: 11.912 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 5.69,
        mass: Mass {
            kg: 0.783 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 0.793 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 5320. },
        age: Some(Time {
            s: 9. * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 21.4199307 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 8] = [
    DIPHDA,
    MENKAR,
    MIRA,
    BATEN_KAITOS,
    KAFFALJIDHMA,
    IOTA_CETI,
    DENEB_ALGENUBI,
    TAU_CETI,
];
