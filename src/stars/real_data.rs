use super::{star_appearance::StarAppearance, star_data::StarData};
use crate::{
    color::sRGBColor,
    coordinates::{
        declination::Declination, earth_equatorial::EarthEquatorialCoordinates,
        right_ascension::RightAscension,
    },
};
use simple_si_units::{
    base::{Distance, Mass, Temperature, Time},
    electromagnetic::Illuminance,
};
use std::fmt::Display;

pub struct RealData {
    pub common_name: &'static str,
    pub astronomical_name: &'static str,
    pub mass: Option<Mass<f64>>,
    pub radius: Option<Distance<f64>>,
    pub absolute_magnitude: f64,
    pub apparent_magnitude: f64,
    pub temperature: Option<Temperature<f64>>,
    pub age: Option<Time<f64>>,
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
        let ra = self.right_ascension.to_angle();
        let dec = self.declination.to_angle();
        let direction_in_ecliptic = EarthEquatorialCoordinates::new(ra, dec).to_direction();
        StarData {
            name: name.to_string(),
            mass: self.mass,
            radius: self.radius,
            luminosity: Some(self.luminosity),
            temperature: self.temperature,
            age: self.age,
            distance: Some(self.distance),
            direction_in_ecliptic,
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
        let direction_in_ecliptic = EarthEquatorialCoordinates::new(ra, dec).to_direction();
        let illuminance = Illuminance::from_apparent_magnitude(self.apparent_magnitude);
        let color = match self.temperature {
            Some(temperature) => sRGBColor::from_temperature(temperature),
            None => sRGBColor::DEFAULT,
        };
        StarAppearance {
            name: name.to_string(),
            illuminance,
            color,
            direction_in_ecliptic,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::real_data::stars::BRIGHTEST_STARS;

    #[test]
    fn calculate_apparent_magnitude() {
        let mut failed = false;
        for star_data in BRIGHTEST_STARS {
            let star = star_data.to_star_data();
            let apparent_magnitude = star
                .get_luminosity()
                .unwrap()
                .to_illuminance(&star.distance.unwrap())
                .to_apparent_magnitude();
            let difference = star_data.apparent_magnitude - apparent_magnitude;
            if difference.abs() > 0.1 {
                println!(
                    "{}:\nexpected: {}, actual: {}, difference: {}",
                    star.name, star_data.apparent_magnitude, apparent_magnitude, difference
                );
                failed = true;
            }
        }
        assert!(!failed);
    }

    #[test]
    fn every_star_hto_a_name() {
        for star_data in BRIGHTEST_STARS {
            let star_data = star_data.to_star_data();
            assert!(!star_data.name.is_empty());
            let star_appearance = star_data.to_star_appearance();
            assert!(!star_appearance.name.is_empty());
        }
    }
}
