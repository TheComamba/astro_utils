use super::trajectory::Trajectory;
use crate::error::AstroUtilError;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

lazy_static! {
    pub(crate) static ref PARSEC_DATA: Mutex<Result<ParsecData, AstroUtilError>> =
        Mutex::new(ParsecData::new());
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ParsecData {
    pub(super) data: Vec<Trajectory>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        coordinates::cartesian::CartesianCoordinates,
        real_data::stars::{all::get_many_stars, SUN},
        stars::random::parsec::line::ParsedParsecLine,
        tests::eq_within,
        units::{
            distance::SOLAR_RADIUS, luminous_intensity::SOLAR_LUMINOUS_INTENSITY, mass::SOLAR_MASS,
            time::BILLION_YEARS,
        },
    };
    use simple_si_units::base::Mass;

    pub(super) fn get_params_for_current_mass_and_age<'a>(
        data: &'a ParsecData,
        mass: &Mass<f64>,
        age_in_years: f64,
    ) -> &'a ParsedParsecLine {
        let mut mass_index = ParsecData::get_closest_mass_index(mass.to_solar_mass());
        let mut trajectory = &data.data[mass_index];
        let mut index = trajectory.get_closest_params_index(age_in_years);
        let mut params = trajectory.get_params_by_index_unchecked(index);
        while params.mass_in_solar_masses < mass.to_solar_mass()
            && mass_index < ParsecData::SORTED_MASSES.len() - 1
        {
            mass_index += 1;
            trajectory = &data.data[mass_index];
            index = trajectory.get_closest_params_index(age_in_years);
            params = trajectory.get_params_by_index_unchecked(index);
        }
        params
    }

    #[test]
    fn test_caluclate_sun() {
        let mass = SUN.mass;
        let age = SUN.age.unwrap();
        let calculated_sun = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            get_params_for_current_mass_and_age(&parsec_data, &mass, age.to_yr())
                .to_star(CartesianCoordinates::ORIGIN)
        };
        let real_sun = SUN.to_star_data();
        println!(
            "calculated mass: {}, real mass: {}",
            calculated_sun.get_mass_at_epoch().unwrap(),
            real_sun.get_mass_at_epoch().unwrap()
        );
        println!(
            "calculated radius: {}, real radius: {}",
            calculated_sun.get_radius_at_epoch().unwrap(),
            real_sun.get_radius_at_epoch().unwrap()
        );
        println!(
            "calculated luminous_intensity: {}, real luminous_intensity: {}",
            calculated_sun.get_luminous_intensity_at_epoch(),
            real_sun.get_luminous_intensity_at_epoch()
        );
        println!(
            "calculated temperature: {}, real temperature: {}",
            calculated_sun.get_temperature_at_epoch(),
            real_sun.get_temperature_at_epoch()
        );
        assert!(eq_within(
            calculated_sun.get_mass_at_epoch().unwrap().kg,
            real_sun.get_mass_at_epoch().unwrap().kg,
            1e-2 * SOLAR_MASS.kg
        ));
        assert!(eq_within(
            calculated_sun.get_radius_at_epoch().unwrap().m,
            real_sun.get_radius_at_epoch().unwrap().m,
            1e-1 * SOLAR_RADIUS.m
        ));
        assert!(eq_within(
            calculated_sun.get_luminous_intensity_at_epoch().cd,
            real_sun.get_luminous_intensity_at_epoch().cd,
            0.5 * SOLAR_LUMINOUS_INTENSITY.cd
        ));
        assert!(eq_within(
            calculated_sun.get_temperature_at_epoch().K,
            real_sun.get_temperature_at_epoch().K,
            500.
        ));
    }

    #[test]
    fn test_calculate_star() {
        let mut num_success = 0;
        let mut num_fail = 0;
        {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            for data in get_many_stars().iter() {
                if let Some(age) = data.age {
                    let age = age.to_yr();
                    let mass_index = ParsecData::get_closest_mass_index(data.mass.to_solar_mass());
                    let trajectory = parsec_data.get_trajectory_via_index(mass_index);
                    let age_expectancy = trajectory.lifetime;
                    if age_expectancy < 0.3 * BILLION_YEARS {
                        // Numerics get really unstable for stars with short life expectancies.
                        continue;
                    }

                    let current_params =
                        get_params_for_current_mass_and_age(&parsec_data, &data.mass, age);
                    let calculated_star = current_params.to_star(CartesianCoordinates::ORIGIN);
                    let real_star = data.to_star_data();
                    if calculated_star.similar_within_order_of_magnitude(&real_star) {
                        num_success += 1;
                    } else {
                        println!("Comparing data for {} failed.\n\n", data.common_name);
                        num_fail += 1;
                    }
                }
            }
        }
        println!("\nnum_success: {}", num_success);
        println!("num_fail: {}", num_fail);
        assert!(num_success > num_fail);
    }
}
