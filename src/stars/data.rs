use super::{appearance::StarAppearance, evolution::StarDataEvolution, fate::StarFate};
use crate::{
    color::srgb::sRGBColor,
    coordinates::ecliptic::EclipticCoordinates,
    units::{illuminance::IRRADIANCE_ZERO, luminous_intensity::luminous_intensity_to_illuminance},
};
use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Luminosity, Mass, Temperature, Time};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarData {
    pub(super) name: String,
    pub(super) constellation: Option<String>,
    pub(super) mass: Option<Mass<f64>>,
    pub(super) radius: Option<Distance<f64>>,
    pub(super) luminous_intensity: Option<Luminosity<f64>>,
    pub(super) temperature: Temperature<f64>,
    pub(super) distance: Distance<f64>,
    pub(super) pos: EclipticCoordinates,
    pub(super) evolution: StarDataEvolution,
}

impl StarData {
    pub fn new(
        name: String,
        constellation: Option<String>,
        mass: Option<Mass<f64>>,
        radius: Option<Distance<f64>>,
        luminous_intensity: Option<Luminosity<f64>>,
        temperature: Temperature<f64>,
        distance: Distance<f64>,
        pos: EclipticCoordinates,
        evolution: StarDataEvolution,
    ) -> Self {
        Self {
            name,
            mass,
            radius,
            luminous_intensity,
            temperature,
            distance,
            pos,
            constellation,
            evolution,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_constellation(&self) -> &Option<String> {
        &self.constellation
    }

    pub const fn get_mass_at_epoch(&self) -> &Option<Mass<f64>> {
        &self.mass
    }

    pub fn get_mass(&self, time: Time<f64>) -> Option<Mass<f64>> {
        Some(self.evolution.apply_to_mass(self.mass?, time))
    }

    pub const fn get_radius_at_epoch(&self) -> &Option<Distance<f64>> {
        &self.radius
    }

    pub fn get_radius(&self, time: Time<f64>) -> Option<Distance<f64>> {
        Some(self.evolution.apply_to_radius(self.radius?, time))
    }

    pub const fn get_luminous_intensity_at_epoch(&self) -> &Option<Luminosity<f64>> {
        &self.luminous_intensity
    }

    pub fn get_luminous_intensity(&self, time: Time<f64>) -> Option<Luminosity<f64>> {
        Some(self.evolution.apply_to_luminous_intensity(self, time))
    }

    pub const fn get_temperature_at_epoch(&self) -> &Temperature<f64> {
        &self.temperature
    }

    pub fn get_temperature(&self, time: Time<f64>) -> Temperature<f64> {
        self.evolution.apply_to_temperature(self, time)
    }

    pub const fn get_age_at_epoch(&self) -> &Option<Time<f64>> {
        &self.evolution.age
    }

    pub fn get_age(&self, time: Time<f64>) -> Option<Time<f64>> {
        self.evolution.age.map(|age| age + time)
    }

    pub const fn get_distance_at_epoch(&self) -> &Distance<f64> {
        &self.distance
    }

    pub fn get_distance(&self, _time: Time<f64>) -> Distance<f64> {
        self.distance
    }

    pub const fn get_pos_at_epoch(&self) -> &EclipticCoordinates {
        &self.pos
    }

    pub fn get_pos(&self, _time: Time<f64>) -> EclipticCoordinates {
        self.pos.clone()
    }

    pub fn get_time_until_death(&self, time_since_epoch: Time<f64>) -> Option<Time<f64>> {
        self.evolution.time_until_death(time_since_epoch)
    }

    pub fn get_lifetime(&self) -> Time<f64> {
        self.evolution.lifetime
    }

    pub fn get_fate(&self) -> &StarFate {
        &self.evolution.fate
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_constellation(&mut self, constellation: Option<String>) {
        self.constellation = constellation;
    }

    pub fn set_mass_at_epoch(&mut self, mass: Option<Mass<f64>>) {
        self.mass = mass;
    }

    pub fn set_radius_at_epoch(&mut self, radius: Option<Distance<f64>>) {
        self.radius = radius;
    }

    pub fn set_luminous_intensity_at_epoch(&mut self, luminous_intensity: Option<Luminosity<f64>>) {
        self.luminous_intensity = luminous_intensity;
    }

    pub fn set_temperature_at_epoch(&mut self, temperature: Temperature<f64>) {
        self.temperature = temperature;
    }

    pub fn set_age_at_epoch(&mut self, age: Option<Time<f64>>) {
        self.evolution.age = age;
    }

    pub fn set_distance_at_epoch(&mut self, distance: Distance<f64>) {
        self.distance = distance;
    }

    pub fn set_pos_at_epoch(&mut self, pos: EclipticCoordinates) {
        self.pos = pos;
    }

    pub fn get_evolution(&self) -> &StarDataEvolution {
        &self.evolution
    }

    pub fn has_changed(&self, then: Time<f64>, now: Time<f64>) -> bool {
        self.evolution.has_changed(then, now)
    }

    pub fn to_star_appearance(&self, time_since_epoch: Time<f64>) -> StarAppearance {
        let illuminance = match self.get_luminous_intensity(time_since_epoch) {
            Some(luminous_intensity) => {
                luminous_intensity_to_illuminance(&luminous_intensity, &self.distance)
            }
            _ => IRRADIANCE_ZERO,
        };

        let color = sRGBColor::from_temperature(self.get_temperature(time_since_epoch));

        let pos = self.get_pos(time_since_epoch);

        StarAppearance {
            name: self.name.clone(),
            illuminance,
            color,
            pos,
            time_since_epoch,
        }
    }

    #[cfg(test)]
    pub(crate) fn similar_within_order_of_magnitude(&self, other: &Self) -> bool {
        use crate::units::luminous_intensity::luminous_intensity_to_absolute_magnitude;

        let mass_ratio = match (self.mass, other.mass) {
            (Some(self_mass), Some(other_mass)) => self_mass / other_mass,
            _ => 1.0,
        };
        let radius_ratio = match (self.radius, other.radius) {
            (Some(self_radius), Some(other_radius)) => self_radius / other_radius,
            _ => 1.0,
        };
        let luminous_intensity_difference =
            match (self.luminous_intensity, other.luminous_intensity) {
                (Some(self_luminous_intensity), Some(other_luminous_intensity)) => {
                    (luminous_intensity_to_absolute_magnitude(self_luminous_intensity)
                        - luminous_intensity_to_absolute_magnitude(other_luminous_intensity))
                    .abs()
                }
                _ => 0.0,
            };
        let temperature_ratio = self.temperature / other.temperature;
        let age_ratio = match (self.evolution.age, other.evolution.age) {
            (Some(self_age), Some(other_age)) => self_age / other_age,
            _ => 1.0,
        };
        let mut result = true;
        if mass_ratio < 0.1 || mass_ratio > 10.0 {
            println!(
                "mass1: {}, mass2: {}, ratio: {}",
                self.mass.unwrap(),
                other.mass.unwrap(),
                mass_ratio
            );
            result = false;
        }
        if radius_ratio < 0.1 || radius_ratio > 10.0 {
            println!(
                "radius1: {}, radius2: {}, ratio: {}",
                self.radius.unwrap(),
                other.radius.unwrap(),
                radius_ratio
            );
            result = false;
        }
        if luminous_intensity_difference > 1.0 {
            println!(
                "luminous_intensity1: {}, luminous_intensity2: {}, difference: {}",
                self.luminous_intensity.unwrap(),
                other.luminous_intensity.unwrap(),
                luminous_intensity_difference
            );
            result = false;
        }
        if temperature_ratio < 0.1 || temperature_ratio > 10.0 {
            println!(
                "temperature1: {}, temperature2: {}, ratio: {}",
                self.temperature, other.temperature, temperature_ratio
            );
            result = false;
        }
        if age_ratio < 0.1 || age_ratio > 10.0 {
            println!(
                "age1: {}, age2: {}, ratio: {}",
                self.evolution.age.unwrap(),
                other.evolution.age.unwrap(),
                age_ratio
            );
            result = false;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{real_data::stars::all::get_many_stars, units::time::TIME_ZERO};

    #[test]
    fn real_stars_have_a_non_vanishing_lifetime() {
        let star_data: Vec<StarData> = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        for star in star_data {
            assert!(star.get_lifetime() > TIME_ZERO);
        }
    }

    #[test]
    fn real_stars_have_not_died_yet() {
        let star_data: Vec<StarData> = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        for star in star_data {
            if let Some(time_until_death) = star.get_time_until_death(TIME_ZERO) {
                assert!(time_until_death > TIME_ZERO);
            }
        }
    }

    #[test]
    fn stars_below_8_sun_masses_become_white_dwarfs() {
        let star_data: Vec<StarData> = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        for star in star_data {
            if star.mass.unwrap() < Mass::from_solar_mass(8.0) {
                assert_eq!(star.get_fate(), &StarFate::WhiteDwarf);
            }
        }
    }

    #[test]
    fn stars_above_8_sun_masses_go_supernova() {
        let star_data: Vec<StarData> = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        for star in star_data {
            if star.mass.unwrap() > Mass::from_solar_mass(8.0) {
                assert_eq!(star.get_fate(), &StarFate::TypeIISupernova);
            }
        }
    }

    fn kinda_equal(a: Option<f64>, b: Option<f64>) -> bool {
        match (a, b) {
            (Some(a), Some(b)) => (a - b).abs() < 1e-6,
            (None, None) => true,
            _ => false,
        }
    }

    #[test]
    fn comparing_getting_stuff_at_epoch() {
        let star_data: Vec<StarData> = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        for star in star_data {
            assert!(
                kinda_equal(
                    star.get_mass_at_epoch().map(|m| m.kg),
                    star.get_mass(TIME_ZERO).map(|m| m.kg),
                ),
                "Star {}\nMass {:?}\nMass {:?}",
                star.get_name(),
                star.get_mass_at_epoch(),
                star.get_mass(TIME_ZERO)
            );
            assert!(
                kinda_equal(
                    star.get_radius_at_epoch().map(|r| r.m),
                    star.get_radius(TIME_ZERO).map(|r| r.m)
                ),
                "Star {}\nRadius {:?}\nRadius {:?}",
                star.get_name(),
                star.get_radius_at_epoch(),
                star.get_radius(TIME_ZERO)
            );
            assert!(
                kinda_equal(
                    star.get_luminous_intensity_at_epoch().map(|l| l.cd),
                    star.get_luminous_intensity(TIME_ZERO).map(|l| l.cd)
                ),
                "Star {}\nLuminous intensity {:?}\nLuminous intensity {:?}",
                star.get_name(),
                star.get_luminous_intensity_at_epoch(),
                star.get_luminous_intensity(TIME_ZERO)
            );
            assert!(
                kinda_equal(
                    Some(star.get_temperature_at_epoch().K),
                    Some(star.get_temperature(TIME_ZERO).K)
                ),
                "Star {}\nTemperature {:?}\nTemperature {:?}",
                star.get_name(),
                star.get_temperature_at_epoch(),
                star.get_temperature(TIME_ZERO)
            );
            assert!(
                kinda_equal(
                    star.get_age_at_epoch().map(|t| t.s),
                    star.get_age(TIME_ZERO).map(|t| t.s)
                ),
                "Star {}\nAge {:?}\nAge {:?}",
                star.get_name(),
                star.get_age_at_epoch(),
                star.get_age(TIME_ZERO)
            );
            assert!(
                kinda_equal(
                    Some(star.get_distance_at_epoch().m),
                    Some(star.get_distance(TIME_ZERO).m)
                ),
                "Star {}\nDistance {:?}\nDistance {:?}",
                star.get_name(),
                star.get_distance_at_epoch(),
                star.get_distance(TIME_ZERO)
            );
            assert!(
                kinda_equal(
                    Some(star.get_pos_at_epoch().get_longitude().rad),
                    Some(star.get_pos(TIME_ZERO).get_longitude().rad)
                ),
                "Star {}\nLongitude {:?}\nLongitude {:?}",
                star.get_name(),
                star.get_pos_at_epoch(),
                star.get_pos(TIME_ZERO)
            );
            assert!(
                kinda_equal(
                    Some(star.get_pos_at_epoch().get_latitude().rad),
                    Some(star.get_pos(TIME_ZERO).get_latitude().rad)
                ),
                "Star {}\nLatitude {:?}\nLatitude {:?}",
                star.get_name(),
                star.get_pos_at_epoch(),
                star.get_pos(TIME_ZERO)
            );
        }
    }
}
