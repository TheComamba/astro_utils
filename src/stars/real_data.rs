use super::{
    appearance::StarAppearance, appearance_evolution::StarAppearanceEvolution, data::StarData,
    data_evolution::StarDataEvolution,
};
use crate::{
    color::srgb::sRGBColor,
    coordinates::{
        declination::Declination, earth_equatorial::EarthEquatorialCoordinates,
        right_ascension::RightAscension,
    },
    units::{
        illuminance::apparent_magnitude_to_illuminance,
        luminous_intensity::absolute_magnitude_to_luminous_intensity,
    },
};
use simple_si_units::base::{Distance, Mass, Temperature, Time};
use std::fmt::Display;

pub struct RealData {
    pub common_name: &'static str,
    pub astronomical_name: &'static str,
    pub constellation: &'static str,
    pub mass: Mass<f64>,
    pub radius: Option<Distance<f64>>,
    pub absolute_magnitude: f64,
    pub apparent_magnitude: f64,
    pub temperature: Temperature<f64>,
    pub age: Option<Time<f64>>,
    pub lifetime: Time<f64>,
    pub right_ascension: RightAscension,
    pub declination: Declination,
    pub distance: Distance<f64>,
}

impl Display for RealData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Distance<f64> as Display>::fmt(&self.distance, f)
    }
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
        let ra = self.right_ascension.to_angle();
        let dec = self.declination.to_angle();
        let pos = EarthEquatorialCoordinates::new(ra, dec).to_ecliptic();
        StarData {
            name: name.to_string(),
            mass: Some(self.mass),
            constellation,
            radius: self.radius,
            luminous_intensity: Some(luminous_intensity),
            temperature: self.temperature,
            age: self.age,
            distance: self.distance,
            pos,
            evolution: StarDataEvolution::NONE,
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
        let pos = EarthEquatorialCoordinates::new(ra, dec).to_ecliptic();
        let illuminance = apparent_magnitude_to_illuminance(self.apparent_magnitude);
        let color = sRGBColor::from_temperature(self.temperature);
        StarAppearance {
            name: name.to_string(),
            illuminance,
            color,
            pos,
            evolution: StarAppearanceEvolution::NONE,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        real_data::stars::all::get_many_stars,
        units::{
            illuminance::illuminance_to_apparent_magnitude,
            luminous_intensity::luminous_intensity_to_illuminance, time::TIME_ZERO,
        },
    };

    #[test]
    fn calculate_apparent_magnitude() {
        for star_data in get_many_stars() {
            let star = star_data.to_star_data();
            let luminous_intensity = star.get_luminous_intensity_at_epoch().unwrap();
            let illuminance =
                luminous_intensity_to_illuminance(&luminous_intensity, &star.distance);
            let apparent_magnitude = illuminance_to_apparent_magnitude(&illuminance);
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
            let star_appearance = star_data.to_star_appearance();
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
                    age.to_yr() < 13.8e9,
                    "{} is older than the universe",
                    star_data.astronomical_name
                );
            }
        }
    }

    #[test]
    fn no_star_is_older_than_its_lifetime() {
        for star_data in get_many_stars() {
            assert!(star_data.lifetime > TIME_ZERO);
            if let Some(age) = star_data.age {
                assert!(
                    age < star_data.lifetime,
                    "{} is older than its lifetime",
                    star_data.astronomical_name
                );
            }
        }
    }
}
