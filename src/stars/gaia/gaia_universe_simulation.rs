use astro_coords::{cartesian::Cartesian, earth_equatorial::EarthEquatorial};
use gaia_access::{
    condition::GaiaCondition,
    data::gaiadr3::{
        gaia_universe_model::{gaia_universe_model, Col},
        gaiadr3,
    },
    query::GaiaQueryBuilder,
    result::*,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use simple_si_units::{
    base::{Distance, Luminosity, Mass, Temperature, Time},
    geometry::Angle,
};
use std::collections::HashMap;

use crate::{
    error::AstroUtilError,
    stars::{
        data::StarData, evolution::StarDataEvolution, physical_parameters::StarPhysicalParameters,
    },
    units::{distance::SOLAR_RADIUS, luminous_intensity::absolute_magnitude_to_luminous_intensity},
};

fn get_id(map: &HashMap<Col, GaiaCellData>) -> Option<String> {
    let id = get_float(map.get(&Col::source_id)?)?;
    Some(id.to_string())
}

fn get_temperature(map: &HashMap<Col, GaiaCellData>) -> Option<Temperature<f64>> {
    let temperature = get_float(map.get(&Col::teff)?)?;
    Some(Temperature::from_K(temperature))
}

fn get_mass(map: &HashMap<Col, GaiaCellData>) -> Option<Mass<f64>> {
    let mass = get_float(map.get(&Col::mass)?)?;
    Some(Mass::from_solar_mass(mass))
}

fn get_radius(map: &HashMap<Col, GaiaCellData>) -> Option<Distance<f64>> {
    let radius = get_float(map.get(&Col::radius)?)?;
    Some(radius * SOLAR_RADIUS)
}

fn get_luminous_intensity(map: &HashMap<Col, GaiaCellData>) -> Option<Luminosity<f64>> {
    let mag = get_float(map.get(&Col::mean_absolute_v)?)?;
    Some(absolute_magnitude_to_luminous_intensity(mag))
}

fn get_pos(map: &HashMap<Col, GaiaCellData>) -> Option<Cartesian> {
    let ra = get_float(map.get(&Col::ra)?)?;
    let ra = Angle::from_deg(ra);
    let dec = get_float(map.get(&Col::dec)?)?;
    let dec = Angle::from_deg(dec);
    let distance = get_float(map.get(&Col::barycentric_distance)?)?;
    let distance = Distance::from_parsec(distance);
    let pos = EarthEquatorial::new(ra, dec)
        .to_direction()
        .to_cartesian(distance);
    Some(pos)
}

fn get_age(map: &HashMap<Col, GaiaCellData>) -> Option<Time<f64>> {
    let age = get_float(map.get(&Col::age)?)?;
    Some(Time::from_Gyr(age))
}

fn get_evolution(map: &HashMap<Col, GaiaCellData>) -> Option<StarDataEvolution> {
    let age = get_age(map)?;
    let mass = get_mass(map)?;
    Some(StarDataEvolution::from_age_and_mass(age, mass))
}

pub(crate) fn to_star_data(result: GaiaResult<Col>) -> Result<Vec<StarData>, AstroUtilError> {
    let stars = result
        .data
        .par_iter()
        .map(|map| {
            let name = get_id(map).ok_or(AstroUtilError::DataNotAvailable("name".to_string()))?;

            let temperature = get_temperature(map).unwrap_or(Temperature::from_K(0.));
            let mass = get_mass(map);
            let radius = get_radius(map);
            let luminous_intensity = get_luminous_intensity(map).ok_or(
                AstroUtilError::DataNotAvailable("luminous_intensity".to_string()),
            )?;
            let physical_parameters =
                StarPhysicalParameters::new(mass, radius, luminous_intensity, temperature);

            let pos = get_pos(map).ok_or(AstroUtilError::DataNotAvailable("pos".to_string()))?;
            let evolution = get_evolution(map)
                .ok_or(AstroUtilError::DataNotAvailable("evolution".to_string()))?;
            let star = StarData {
                name,
                params: physical_parameters,
                pos,
                constellation: None,
                evolution,
            };
            Ok(star)
        })
        .collect::<Result<Vec<StarData>, AstroUtilError>>();
    stars
}

pub(crate) fn query_nearest_simulated_stars(
    distance_threshold: Distance<f64>,
    magnitude_threshold: Option<f64>,
) -> Result<GaiaResult<Col>, AstroUtilError> {
    let mut query = GaiaQueryBuilder::new(gaiadr3, gaia_universe_model)
        .select(vec![
            Col::source_id,
            Col::mass,
            Col::radius,
            Col::mean_absolute_v,
            Col::teff,
            Col::ra,
            Col::dec,
            Col::barycentric_distance,
            Col::age,
        ])
        .where_clause(GaiaCondition::LessThan(
            Col::barycentric_distance,
            distance_threshold.to_parsec(),
        ))
        .where_clause(GaiaCondition::GreaterThan(Col::mass, 0.08));
    if let Some(mag) = magnitude_threshold {
        query = query.where_clause(GaiaCondition::LessThan(Col::mag_g, mag));
    }
    Ok(query.do_query()?)
}

pub fn fetch_brightest_stars_simulated_data() -> Result<Vec<StarData>, AstroUtilError> {
    let max_distance = Distance::from_lyr(100_000.);
    let min_brightness = Some(6.5);
    let resp = query_nearest_simulated_stars(max_distance, min_brightness)?;
    let gaia_stars = to_star_data(resp)?;
    Ok(gaia_stars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_model_star_has_a_mass() {
        let max_distance = Distance::from_lyr(100_000.);
        let min_brightness = Some(4.);
        let resp = query_nearest_simulated_stars(max_distance, min_brightness).unwrap();
        let stars = to_star_data(resp).unwrap();
        for star in stars {
            assert!(
                star.params.mass.is_some(),
                "Star {} does not have a mass",
                star.name
            );
        }
    }
}
