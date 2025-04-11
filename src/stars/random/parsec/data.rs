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
    use astro_coords::cartesian::Cartesian;
    use uom::si::{f64::Time, luminous_intensity::candela, thermodynamic_temperature::kelvin};

    use crate::{
        astro_display::AstroDisplay,
        real_data::stars::{all::get_many_stars, SUN},
        tests::eq_within,
        units::{
            length::solar_radii, luminous_intensity::solar_luminous_intensity, mass::solar_mass,
            time::gigayear,
        },
    };

    use super::*;

    #[test]
    fn test_caluclate_sun() {
        let mass = SUN.mass;
        let age = SUN.age.unwrap();
        let mass_index = ParsecData::get_closest_mass_index(mass.get::<solar_mass>());
        let calculated_sun = {
            let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
            let parsec_data = parsec_data_mutex.as_ref().unwrap();
            parsec_data
                .get_trajectory_via_index(mass_index)
                .to_star(age, Cartesian::origin())
        };
        let real_sun = SUN.to_star_data();
        println!(
            "calculated mass: {}, real mass: {}",
            calculated_sun.get_mass_at_epoch().unwrap().astro_display(),
            real_sun.get_mass_at_epoch().unwrap().astro_display()
        );
        println!(
            "calculated radius: {}, real radius: {}",
            calculated_sun
                .get_radius_at_epoch()
                .unwrap()
                .astro_display(),
            real_sun.get_radius_at_epoch().unwrap().astro_display()
        );
        println!(
            "calculated luminous_intensity: {}, real luminous_intensity: {}",
            calculated_sun
                .get_luminous_intensity_at_epoch()
                .astro_display(),
            real_sun.get_luminous_intensity_at_epoch().astro_display()
        );
        println!(
            "calculated temperature: {}, real temperature: {}",
            calculated_sun.get_temperature_at_epoch().astro_display(),
            real_sun.get_temperature_at_epoch().astro_display()
        );
        assert!(eq_within(
            calculated_sun
                .get_mass_at_epoch()
                .unwrap()
                .get::<solar_mass>(),
            real_sun.get_mass_at_epoch().unwrap().get::<solar_mass>(),
            1e-2
        ));
        assert!(eq_within(
            calculated_sun
                .get_radius_at_epoch()
                .unwrap()
                .get::<solar_radii>(),
            real_sun.get_radius_at_epoch().unwrap().get::<solar_radii>(),
            1e-1
        ));
        assert!(eq_within(
            calculated_sun
                .get_luminous_intensity_at_epoch()
                .get::<candela>(),
            real_sun.get_luminous_intensity_at_epoch().get::<candela>(),
            0.5 * solar_luminous_intensity().get::<candela>()
        ));
        assert!(eq_within(
            calculated_sun.get_temperature_at_epoch().get::<kelvin>(),
            real_sun.get_temperature_at_epoch().get::<kelvin>(),
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
                    let age = age;
                    let mass_index =
                        ParsecData::get_closest_mass_index(data.mass.get::<solar_mass>());
                    let trajectory = parsec_data.get_trajectory_via_index(mass_index);
                    let age_expectancy = trajectory.lifetime;
                    if age_expectancy < Time::new::<gigayear>(0.3) {
                        // Numerics get really unstable for stars with short life expectancies.
                        continue;
                    }

                    let calculated_star = parsec_data
                        .get_trajectory_via_index(mass_index)
                        .to_star(age, Cartesian::origin());
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
