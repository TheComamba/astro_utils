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

fn ALDERAMIN() -> RealData {
    RealData {
        common_name: "Alderamin",
        astronomical_name: "α Cephei",
        constellation: "Cepheus",
        radius: Some(Length {
            m: 2.4 * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 2. * SOLAR_MASS.kg,
        },
        absolute_magnitude: 1.58,
        apparent_magnitude: 2.45,
        temperature: Temperature { K: 7700. },
        right_ascension: RightAscension::new(21, 18, 35.),
        declination: Declination::new(Sgn::Pos, 62, 35, 8.),
        distance: Length {
            m: 49. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.82 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.36020165 * BILLION_YEARS.s,
        },
    }
}

fn ALFIRK() -> RealData {
    RealData {
        common_name: "Alfirk",
        astronomical_name: "β Cephei",
        constellation: "Cepheus",
        right_ascension: RightAscension::new(21, 28, 40.),
        declination: Declination::new(Sgn::Pos, 70, 33, 39.),
        apparent_magnitude: 3.23,
        distance: Length {
            m: 594.9 * LIGHT_YEAR.m,
        },
        absolute_magnitude: -3.08,
        mass: Mass {
            kg: 7.4 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 5.6 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 27_000. },
        age: Some(Time {
            s: 0.0087 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.052267043 * BILLION_YEARS.s,
        },
    }
}

fn ERRAI() -> RealData {
    RealData {
        common_name: "Errai",
        astronomical_name: "γ Cephei",
        constellation: "Cepheus",
        right_ascension: RightAscension::new(23, 39, 21.),
        declination: Declination::new(Sgn::Pos, 77, 37, 57.),
        apparent_magnitude: 3.21,
        distance: Length {
            m: 44.98 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 2.62,
        mass: Mass {
            kg: 1.294 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 4.93 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4792. },
        age: Some(Time {
            s: 3.25 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 3.9126515 * BILLION_YEARS.s,
        },
    }
}

fn DELTA_CEPHERI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "δ Cephei",
        constellation: "Cepheus",
        right_ascension: RightAscension::new(22, 29, 10.),
        declination: Declination::new(Sgn::Pos, 58, 24, 55.),
        apparent_magnitude: 4.07,
        distance: Length {
            m: 981.9 * LIGHT_YEAR.m,
        },
        absolute_magnitude: -3.32,
        mass: Mass {
            kg: 4.5 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 44.5 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 6000. },
        age: Some(Time {
            s: 0.079 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.151849866 * BILLION_YEARS.s,
        },
    }
}

fn ETA_CEPHEI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "η Cephei",
        constellation: "Cepheus",
        right_ascension: RightAscension::new(20, 45, 17.),
        declination: Declination::new(Sgn::Pos, 61, 50, 20.),
        apparent_magnitude: 3.426,
        distance: Length {
            m: 46.53 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 2.631,
        mass: Mass {
            kg: 1.6 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 4.12 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4950. },
        age: Some(Time {
            s: 1.9 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 2.08398753 * BILLION_YEARS.s,
        },
    }
}

fn IOTA_CEPHEI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ι Cephei",
        constellation: "Cepheus",
        right_ascension: RightAscension::new(22, 49, 41.),
        declination: Declination::new(Sgn::Pos, 66, 12, 1.),
        apparent_magnitude: 3.507,
        distance: Length {
            m: 115.3 * LIGHT_YEAR.m,
        },
        absolute_magnitude: 0.76,
        mass: Mass {
            kg: 2.15 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 11.08 * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4768. },
        age: Some(Time {
            s: 1.0 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 1.09929685 * BILLION_YEARS.s,
        },
    }
}

fn ZETA_CEPHEI() -> RealData {
    RealData {
        common_name: "",
        astronomical_name: "ζ Cephei",
        constellation: "Cepheus",
        right_ascension: RightAscension::new(22, 10, 51.),
        declination: Declination::new(Sgn::Pos, 58, 12, 5.),
        apparent_magnitude: 3.39,
        distance: Length {
            m: 726.1 * LIGHT_YEAR.m,
        },
        absolute_magnitude: -3.35,
        mass: Mass {
            kg: 7.9 * SOLAR_MASS.kg,
        },
        radius: Some(Length {
            m: 94. * SOLAR_RADIUS.m,
        }),
        temperature: Temperature { K: 4072. },
        age: None,
        lifetime: Time {
            s: 0.040555762 * BILLION_YEARS.s,
        },
    }
}

fn ERAKIS() -> RealData {
    RealData {
        common_name: "Erakis",
        astronomical_name: "μ Cephei",
        constellation: "Cepheus",
        radius: Some(Length {
            m: 972. * SOLAR_RADIUS.m,
        }),
        mass: Mass {
            kg: 19.2 * SOLAR_MASS.kg,
        },
        absolute_magnitude: -6.5,
        apparent_magnitude: 3.43,
        temperature: Temperature { K: 3551. },
        right_ascension: RightAscension::new(21, 43, 30.),
        declination: Declination::new(Sgn::Pos, 58, 46, 48.),
        distance: Length {
            m: 3066. * LIGHT_YEAR.m,
        },
        age: Some(Time {
            s: 0.0097 * BILLION_YEARS.s,
        }),
        lifetime: Time {
            s: 0.009767659 * BILLION_YEARS.s,
        },
    }
}

pub(crate) const STARS: [RealData; 8] = [
    ALDERAMIN,
    ALFIRK,
    ERRAI,
    DELTA_CEPHERI,
    ETA_CEPHEI,
    IOTA_CEPHEI,
    ZETA_CEPHEI,
    ERAKIS,
];
