use astro_coords::ra_and_dec::*;
use astro_units::{length::solar_radius, mass::solar_mass, time::gigayear};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::light_year,
    thermodynamic_temperature::kelvin,
};

use crate::stars::real_data::RealData;

//https://web.pa.msu.edu/people/horvatin/Astronomy_Facts/brightest_stars.html
// Gaia data that was not found in the list of 100 brightest stars
// Designations can be checked under http://simbad.cds.unistra.fr/simbad/sim-fid
// http://www.avastronomyclub.org/skymap/d/skymap.php

// RealData = RealData {
//     common_name: "",
//     astronomical_name: "",
//     constellation: "",
//     right_ascension: RightAscension::new(),
//     declination: Declination::new(),
//     apparent_magnitude: ,
//     distance: Length{m:  * .m},
//     absolute_magnitude: ,
//     mass: Some(Mass{kg: * .kg}),
//     radius: Some(Length{m: * .m}),
//     temperature: Temperature{K:}),
//     age: Some(Time{s:* .s}),
// };

pub fn sun() -> RealData {
    RealData {
        common_name: "Sun",
        astronomical_name: "Sol",
        constellation: "",
        mass: Mass::new::<solar_mass>(1.0),
        radius: Some(Length::new::<solar_radius>(1.0)),
        absolute_magnitude: 4.83,
        apparent_magnitude: -26.74, //seen from earth
        temperature: ThermodynamicTemperature::new::<kelvin>(5778.0),
        age: Some(Time::new::<gigayear>(4.6)),
        right_ascension: RightAscension::new(0, 0, 0.),
        declination: Declination::new(Sgn::Pos, 0, 0, 0.),
        distance: Length::new::<light_year>(0.),
        lifetime: Time::new::<gigayear>(10.0),
    }
}

pub mod all;
pub mod andromeda;
pub mod antlia;
pub mod apus;
pub mod aquarius;
pub mod aquila;
pub mod ara;
pub mod aries;
pub mod auriga;
pub mod bootes;
pub mod caelum;
pub mod camelopardalis;
pub mod cancer;
pub mod canes_venatici;
pub mod canis_major;
pub mod canis_minor;
pub mod capricornus;
pub mod carina;
pub mod cassiopeia;
pub mod centaurus;
pub mod cepheus;
pub mod cetus;
pub mod chamaeleon;
pub mod circinus;
pub mod columba;
pub mod coma_berenices;
pub mod corona_australis;
pub mod corona_borealis;
pub mod corvus;
pub mod crater;
pub mod crux;
pub mod cygnus;
pub mod delphinus;
pub mod dorado;
pub mod draco;
pub mod equuleus;
pub mod eridanus;
pub mod fornax;
pub mod gemini;
pub mod grus;
pub mod hercules;
pub mod horologium;
pub mod hydra;
pub mod hydrus;
pub mod indus;
pub mod lacerta;
pub mod leo;
pub mod leo_minor;
pub mod lepus;
pub mod libra;
pub mod lupus;
pub mod lynx;
pub mod lyra;
pub mod mensa;
pub mod microscopium;
pub mod monoceros;
pub mod musca;
pub mod norma;
pub mod octans;
pub mod ophiuchus;
pub mod orion;
pub mod pavo;
pub mod pegasus;
pub mod perseus;
pub mod phoenix;
pub mod pictor;
pub mod pisces;
pub mod piscis_austrinus;
pub mod puppis;
pub mod pyxis;
pub mod reticulum;
pub mod sagitta;
pub mod sagittarius;
pub mod scorpius;
pub mod sculptor;
pub mod scutum;
pub mod serpens;
pub mod sextans;
pub mod taurus;
pub mod telescopium;
pub mod triangulum;
pub mod triangulum_australe;
pub mod tucana;
pub mod ursa_major;
pub mod ursa_minor;
pub mod vela;
pub mod virgo;
pub mod volans;
pub mod vulpecula;
