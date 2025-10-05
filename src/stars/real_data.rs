use astro_coords::{
    earth_equatorial::EarthEquatorial,
    ra_and_dec::{Declination, RightAscension},
};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    time::year,
};

use crate::{
    color::srgb::sRGBColor,
    units::{
        illuminance::apparent_magnitude_to_illuminance,
        luminous_intensity::absolute_magnitude_to_luminous_intensity,
    },
};

use super::{
    appearance::StarAppearance, data::StarData, evolution::StarDataEvolution, fate::StarFate,
    physical_parameters,
};

pub struct RealData {
    pub common_name: &'static str,
    pub astronomical_name: &'static str,
    pub constellation: &'static str,
    pub mass: Mass,
    pub radius: Option<Length>,
    pub absolute_magnitude: f64,
    pub apparent_magnitude: f64,
    pub temperature: ThermodynamicTemperature,
    pub age: Option<Time>,
    pub lifetime: Time,
    pub right_ascension: RightAscension,
    pub declination: Declination,
    pub distance: Length,
}

impl RealData {
    pub fn to_star_data(&self) -> StarData {
        let name = if self.common_name.is_empty() {
            self.astronomical_name
        } else {
            self.common_name
        };
        let constellation = if self.constellation.is_empty() {
            None
        } else {
            Some(self.constellation.to_string())
        };

        let luminous_intensity = absolute_magnitude_to_luminous_intensity(self.absolute_magnitude);

        let physical_parameters = physical_parameters::StarPhysicalParameters::new(
            Some(self.mass),
            self.radius,
            luminous_intensity,
            self.temperature,
        );

        let ra = self.right_ascension.to_angle();
        let dec = self.declination.to_angle();
        let pos = EarthEquatorial::new(ra, dec)
            .to_direction()
            .to_cartesian(self.distance);

        let evolution =
            StarDataEvolution::new(None, self.age, self.lifetime, StarFate::new(self.mass));
        StarData {
            name: name.to_string(),
            constellation,
            params: physical_parameters,
            pos,
            evolution,
        }
    }

    pub fn to_star_appearance(&self) -> StarAppearance {
        let name = if self.common_name.is_empty() {
            self.astronomical_name
        } else {
            self.common_name
        };
        let ra = self.right_ascension.to_angle();
        let dec = self.declination.to_angle();
        let pos = EarthEquatorial::new(ra, dec).to_ecliptic();
        let illuminance = apparent_magnitude_to_illuminance(self.apparent_magnitude);
        let color = sRGBColor::from_temperature(self.temperature);
        StarAppearance {
            name: name.to_string(),
            illuminance,
            color,
            pos,
            time_since_epoch: Time::new::<year>(0.),
        }
    }
}

#[cfg(test)]
mod tests {

    use uom::si::{
        f64::{Mass, Time},
        time::year,
    };

    use crate::{
        real_data::stars::all::get_many_stars,
        units::{
            illuminance::illuminance_to_apparent_magnitude,
            luminous_intensity::luminous_intensity_to_illuminance,
        },
    };

    #[test]
    fn calculate_apparent_magnitude() {
        for star_data in get_many_stars() {
            let star = star_data.to_star_data();
            let luminous_intensity = star.get_luminous_intensity_at_epoch();
            let illuminance =
                luminous_intensity_to_illuminance(luminous_intensity, star.get_distance_at_epoch());
            let apparent_magnitude = illuminance_to_apparent_magnitude(illuminance);
            let difference = star_data.apparent_magnitude - apparent_magnitude;
            assert!(
                difference.abs() < 0.3,
                "{}:\nexpected: {}, actual: {}, difference: {}",
                star.name,
                star_data.apparent_magnitude,
                apparent_magnitude,
                difference
            );
        }
    }

    #[test]
    fn every_star_has_a_name() {
        for star_data in get_many_stars() {
            let star_data = star_data.to_star_data();
            assert!(!star_data.name.is_empty());
            let star_appearance = star_data.to_star_appearance(Time::new::<year>(0.));
            assert!(!star_appearance.name.is_empty());
        }
    }

    #[test]
    fn all_common_names_are_distinct() {
        let mut names = Vec::new();
        for star_data in get_many_stars() {
            if star_data.common_name.is_empty() {
                continue;
            }
            assert!(
                !names.contains(&star_data.common_name),
                "{} is a duplicate",
                star_data.common_name
            );
            names.push(star_data.common_name);
        }
    }

    #[test]
    fn all_stars_have_a_distinct_astronomical_name() {
        let mut names = Vec::new();
        for star_data in get_many_stars() {
            assert!(!star_data.astronomical_name.is_empty());
            assert!(
                !names.contains(&star_data.astronomical_name),
                "{} is a duplicate",
                star_data.astronomical_name
            );
            names.push(star_data.astronomical_name);
        }
    }

    #[test]
    fn no_star_is_older_than_the_universe() {
        for star_data in get_many_stars() {
            if let Some(age) = star_data.age {
                assert!(
                    age.get::<year>() < 13.8e9,
                    "{} is older than the universe",
                    star_data.astronomical_name
                );
            }
        }
    }

    #[test]
    fn no_star_is_older_than_its_lifetime() {
        for star_data in get_many_stars() {
            assert!(star_data.lifetime > Time::new::<year>(0.));
            if let Some(age) = star_data.age {
                assert!(
                    age < star_data.lifetime,
                    "{} is older than its lifetime",
                    star_data.astronomical_name
                );
            }
        }
    }

    #[test]
    fn all_supernova_stars_have_a_time_until_death() {
        for star_data in get_many_stars() {
            if star_data.mass > Mass::new::<solar_mass>(8.) {
                assert!(
                    star_data
                        .to_star_data()
                        .get_time_until_death(Time::new::<year>(0.))
                        .is_some(),
                    "{} is a supernova star without a time until death",
                    star_data.astronomical_name
                );
            }
        }
    }

    #[test]
    fn real_stars_age_is_passed_to_star_data() {
        for star_data in get_many_stars() {
            if let Some(age) = star_data.age {
                assert_eq!(
                    star_data.to_star_data().get_age_at_epoch().unwrap(),
                    age,
                    "{}",
                    star_data.astronomical_name
                );
            }
        }
    }
}
