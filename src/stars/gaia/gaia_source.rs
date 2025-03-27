use crate::{
    color::srgb::sRGBColor,
    error::AstroUtilError,
    stars::appearance::StarAppearance,
    units::{illuminance::apparent_magnitude_to_illuminance, time::TIME_ZERO},
};
use astro_coords::{ecliptic::Ecliptic, spherical::Spherical};
use gaia_access::{
    condition::GaiaCondition,
    data::gaiadr3::{
        gaia_source::{gaia_source, Col},
        gaiadr3,
    },
    query::GaiaQueryBuilder,
    result::{get_float, get_string, GaiaCellData, GaiaResult},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Instant};
use uom::si::f64::{Angle, ThermodynamicTemperature};

#[derive(Serialize, Deserialize)]
struct GaiaMetadataLine {
    name: String,
    datatype: String,
    xtype: Option<String>,
    arraysize: Option<String>,
    description: String,
    unit: Option<String>,
    ucd: String,
    utype: Option<String>,
}

fn get_designation(map: &HashMap<Col, GaiaCellData>) -> Option<String> {
    get_string(map.get(&Col::designation)?)
}

fn get_ecl_lon(map: &HashMap<Col, GaiaCellData>) -> Option<Angle> {
    let ecl_lon = get_float(map.get(&Col::ecl_lon)?)?;
    Some(Angle::from_degrees(ecl_lon))
}

fn get_ecl_lat(map: &HashMap<Col, GaiaCellData>) -> Option<Angle> {
    let ecl_lat = get_float(map.get(&Col::ecl_lat)?)?;
    Some(Angle::from_degrees(ecl_lat))
}

fn get_temperature(map: &HashMap<Col, GaiaCellData>) -> Option<ThermodynamicTemperature> {
    let temperature = get_float(map.get(&Col::teff_gspphot)?)?;
    Some(Temperature::from_K(temperature))
}

fn get_illuminance(map: &HashMap<Col, GaiaCellData>) -> Option<Illuminance<f64>> {
    let mag = get_float(map.get(&Col::phot_g_mean_mag)?)?;
    Some(apparent_magnitude_to_illuminance(mag))
}

fn to_star_appearances(result: GaiaResult<Col>) -> Result<Vec<StarAppearance>, AstroUtilError> {
    let stars = result
        .data
        .par_iter()
        .map(|map| {
            let name =
                get_designation(map).ok_or(AstroUtilError::DataNotAvailable("name".to_string()))?;
            let illuminance = get_illuminance(map)
                .ok_or(AstroUtilError::DataNotAvailable("illuminance".to_string()))?;
            let temperature = get_temperature(map).unwrap_or(Temperature::from_K(4000.));
            let color = sRGBColor::from_temperature(temperature);
            let lon =
                get_ecl_lon(map).ok_or(AstroUtilError::DataNotAvailable("lon".to_string()))?;
            let lat =
                get_ecl_lat(map).ok_or(AstroUtilError::DataNotAvailable("lat".to_string()))?;
            let pos = Ecliptic::new(Spherical::new(lon, lat));

            let star = StarAppearance {
                name,
                illuminance,
                color,
                pos,
                time_since_epoch: TIME_ZERO,
            };
            Ok(star)
        })
        .collect::<Result<Vec<StarAppearance>, AstroUtilError>>();
    stars
}

fn query_stars_by_brightness(
    brightest_mag: f64,
    dimmest_mag: f64,
) -> Result<GaiaResult<Col>, AstroUtilError> {
    Ok(GaiaQueryBuilder::new(gaiadr3, gaia_source)
        .select(vec![
            Col::designation,
            Col::ecl_lon,
            Col::ecl_lat,
            Col::phot_g_mean_mag,
            Col::teff_gspphot,
        ])
        .where_clause(GaiaCondition::GreaterThanOrEqual(
            Col::phot_g_mean_mag,
            brightest_mag,
        ))
        .where_clause(GaiaCondition::LessThan(Col::phot_g_mean_mag, dimmest_mag))
        .do_query()?)
}

pub fn star_is_already_known(new_star: &StarAppearance, known_stars: &[StarAppearance]) -> bool {
    known_stars
        .iter()
        .any(|known_star| known_star.apparently_the_same(new_star))
}

pub fn fetch_brightest_stars(
    magnitude_threshold: f64,
) -> Result<Vec<StarAppearance>, AstroUtilError> {
    const LIMIT_FOR_FIRST_BATCH: f64 = 4.;

    let mut gaia_stars = vec![];
    let mut brighter_limit = 0.;
    let mut dimmer_limit = f64::min(magnitude_threshold, LIMIT_FOR_FIRST_BATCH);
    while brighter_limit < magnitude_threshold {
        let start = Instant::now();
        let resp = query_stars_by_brightness(brighter_limit, dimmer_limit)?;
        let duration = start.elapsed();
        let gaia_stars_in_this_range = to_star_appearances(resp)?;
        println!(
            "Fetched {} stars with brightness between {} and {} in {} seconds.",
            gaia_stars_in_this_range.len(),
            brighter_limit,
            dimmer_limit,
            duration.as_secs()
        );
        gaia_stars.extend(gaia_stars_in_this_range);
        brighter_limit = dimmer_limit;
        let step = 1000. / dimmer_limit.powi(4);
        dimmer_limit = f64::min(dimmer_limit + step, magnitude_threshold);
    }
    Ok(gaia_stars)
}

#[cfg(test)]
mod tests {
    use crate::{
        astro_display::AstroDisplay,
        real_data::stars::all::get_many_stars,
        units::{
            angle::angle_to_arcsecs,
            illuminance::{illuminance_to_apparent_magnitude, IRRADIANCE_ZERO},
        },
    };

    use super::*;

    fn find_closest_star(
        gaia_star: &StarAppearance,
        known_stars: &[StarAppearance],
    ) -> Option<StarAppearance> {
        let mut closest_star = None;
        let mut closest_distance = Angle::from_degrees(90.);
        for known_star in known_stars.iter() {
            let distance = gaia_star.pos.angle_to(&known_star.pos);
            if distance < closest_distance {
                closest_star = Some(known_star);
                closest_distance = distance;
            }
        }
        closest_star.cloned()
    }

    #[test]
    fn all_bright_gaia_stars_are_already_known() {
        // Gaia finds R Doradus to be much brighter than all other literature.
        const PROBLEMATIC_STAR: &str = "Gaia DR3 4677205714465503104";

        let mut known_stars = vec![];
        for star_data in get_many_stars() {
            known_stars.push(star_data.to_star_appearance());
        }

        let gaia_response = query_stars_by_brightness(0., 2.5).unwrap();
        let gaia_stars = to_star_appearances(gaia_response).unwrap();

        println!("known_stars.len(): {}", known_stars.len());
        assert!(known_stars.len() > 30);
        println!("gaia_stars.len(): {}", gaia_stars.len());
        assert!(gaia_stars.len() > 30);
        let mut failure_count = 0;
        for gaia_star in gaia_stars.iter() {
            if gaia_star.name == PROBLEMATIC_STAR {
                continue;
            }
            let is_known = star_is_already_known(gaia_star, &known_stars);
            if !is_known {
                println!("\ngaia_star is not known");
                let closest = find_closest_star(gaia_star, &known_stars).unwrap();
                println!("gaia_star: {:?}", gaia_star.name);
                println!("closest_star: {:?}", closest.name);
                let (gaia_ra, gaia_dec) = gaia_star.pos.spherical.to_ra_and_dec();
                let (closest_ra, closest_dec) = closest.pos.spherical.to_ra_and_dec();
                let angle_difference = gaia_star.pos.angle_to(&closest.pos);
                if angle_difference > Angle::from_degrees(0.03) {
                    println!("gaia_star position: {}, {}", gaia_ra, gaia_dec);
                    println!("closest_star position: {}, {}", closest_ra, closest_dec);
                    println!(
                        "Angle difference: {} arcsecs",
                        angle_to_arcsecs(&angle_difference)
                    );
                } else {
                    println!("gaia_star_illuminance: {}", gaia_star.illuminance);
                    println!("closest_star_illuminance: {}", closest.illuminance);
                    println!(
                        "Illuminance difference: {} lx",
                        (gaia_star.illuminance - closest.illuminance).to_lux()
                    );
                }
                failure_count += 1;
            }
        }
        println!("failure_count: {} of {}", failure_count, gaia_stars.len());
        assert!(failure_count == 0);
    }

    #[test]
    fn all_not_too_bright_stars_are_in_gaia() {
        const PROBLEMATIC_STARS: [&str; 16] = [
            "R Doradus", // Gaia finds R Doradus to be much brighter than all other literature.
            "Scheat",
            "Suhail",
            "Eltanin",
            "Dschubba",
            "ε Centauri",
            "Menkar",
            "Ghurab",
            "ζ Centauri",
            "η Centauri", // Only in Gaia DR2
            "Enif",       // Only in Gaia DR2
            "ε Antliae",  // Only in Gaia DR2
            "β Arietis",
            "ο Ceti", // Only in Gaia DR2
            "γ Circini",
            "Rasalgethi",
        ];

        const UPPER_BRIGHTNESS_THRESHOLD: f64 = 2.2;
        const LOWER_BRIGHTNESS_THRESHOLD: f64 = 7.;

        let mut known_stars = vec![];
        for star_data in get_many_stars() {
            if star_data.apparent_magnitude > UPPER_BRIGHTNESS_THRESHOLD
                && !PROBLEMATIC_STARS.contains(&star_data.common_name)
                && !PROBLEMATIC_STARS.contains(&star_data.astronomical_name)
            {
                known_stars.push(star_data.to_star_appearance());
            }
        }

        let gaia_response = query_stars_by_brightness(0., LOWER_BRIGHTNESS_THRESHOLD).unwrap();
        let gaia_stars = to_star_appearances(gaia_response).unwrap();

        assert!(
            known_stars.len() > 30,
            "known_stars.len(): {}",
            known_stars.len()
        );
        assert!(
            gaia_stars.len() > 30,
            "gaia_stars.len(): {}",
            gaia_stars.len()
        );

        let brightest_gaia_star = gaia_stars
            .iter()
            .min_by_key(|star| (star.illuminance.to_lux() * 1e5) as u64)
            .unwrap();
        assert!(
            illuminance_to_apparent_magnitude(&brightest_gaia_star.illuminance)
                < UPPER_BRIGHTNESS_THRESHOLD,
            "Brightest gaia star illuminance: {} mag",
            illuminance_to_apparent_magnitude(&brightest_gaia_star.illuminance)
        );

        for known_star in known_stars.iter() {
            let is_known = star_is_already_known(known_star, &gaia_stars);
            let closest_gaia_star = find_closest_star(known_star, &gaia_stars).unwrap();
            assert!(
                is_known,
                "A known star is not found in Gaia\n\n{}\n\nClosest gaia star:\n{}",
                known_star.astro_display(),
                closest_gaia_star.astro_display()
            );
        }
    }

    #[test]
    fn known_stars_brightness_is_the_same() {
        let mut known_stars = vec![];
        for star_data in get_many_stars() {
            known_stars.push(star_data.to_star_appearance());
        }

        let gaia_response = query_stars_by_brightness(0., 3.5).unwrap();
        let gaia_stars = to_star_appearances(gaia_response).unwrap();
        let mut star_pairs = vec![];
        for gaia_star in gaia_stars.iter() {
            for known_star in known_stars.iter() {
                if gaia_star.apparently_the_same(known_star) {
                    star_pairs.push((gaia_star, known_star));
                }
            }
        }
        assert!(
            star_pairs.len() > 15,
            "star_pairs.len(): {}",
            star_pairs.len()
        );
        let mut mean_brightness_difference = IRRADIANCE_ZERO;
        for (gaia_star, known_star) in star_pairs.iter() {
            let brightness_difference = known_star.illuminance - gaia_star.illuminance;
            mean_brightness_difference += brightness_difference;
        }
        mean_brightness_difference /= star_pairs.len() as f64;
        let acceptable_difference = apparent_magnitude_to_illuminance(4.);
        println!(
            "mean_brightness_difference: \n{} lx",
            mean_brightness_difference.to_lux()
        );
        println!(
            "acceptable_difference: \n{} lx",
            acceptable_difference.to_lux()
        );
        println!(
            "ratio: {}",
            mean_brightness_difference / acceptable_difference
        );
        assert!(mean_brightness_difference.to_lux().abs() < acceptable_difference.to_lux());
    }

    #[test]
    #[ignore]
    fn fetch_brightest_stars_works() {
        let gaia_response = fetch_brightest_stars(11.).unwrap();
        assert!(gaia_response.len() > 30);
    }
}
