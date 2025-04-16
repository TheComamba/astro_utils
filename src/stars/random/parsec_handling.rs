use astro_coords::cartesian::Cartesian;
use parsec_access::{
    getters::{get_closest_age_index, get_masses_in_solar, get_parameters, get_trajectory},
    line::ParsecLine,
    trajectory::Trajectory,
};
use rand_distr::WeightedAliasIndex;
use simple_si_units::base::{Luminosity, Mass, Time};

use crate::{
    error::AstroUtilError,
    stars::{
        data::StarData,
        evolution::{StarDataEvolution, StarDataLifestageEvolution},
        fate::{StarFate, TYPE_II_SUPERNOVA_PEAK_MAGNITUDE},
        physical_parameters::StarPhysicalParameters,
    },
    units::{
        luminous_intensity::{
            absolute_magnitude_to_luminous_intensity, LUMINOSITY_ZERO, SOLAR_LUMINOUS_INTENSITY,
        },
        time::TEN_MILLENIA,
    },
};

use super::random_stars::{get_min_age, DIMMEST_ILLUMINANCE, METALLICITY_INDEX};

const MIN_MASS_FOR_HYDROGEN_FUSION: f64 = 0.08;
