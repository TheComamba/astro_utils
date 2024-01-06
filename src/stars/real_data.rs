use crate::{
    coordinates::{declination::Declination, right_ascension::RightAscension},
    units::{length::Length, luminosity::Luminosity, mass::Mass, temperature::Temperature},
};

pub struct RealData {
    pub(crate) name: &'static str,
    pub(crate) mass: Mass,
    pub(crate) radius: Length,
    pub(crate) luminosity: Luminosity,
    pub(crate) temperature: Temperature,
    pub(crate) right_ascension: RightAscension,
    pub(crate) declination: Declination,
    pub(crate) distance: Length,
}
