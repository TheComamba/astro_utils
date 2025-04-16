use super::{
    appearance::StarAppearance, evolution::StarDataEvolution, fate::StarFate,
    physical_parameters::StarPhysicalParameters,
};
use crate::{color::srgb::sRGBColor, units::luminous_intensity::luminous_intensity_to_illuminance};
use astro_coords::{cartesian::Cartesian, ecliptic::Ecliptic};
use serde::{Deserialize, Serialize};
use simple_si_units::base::{Distance, Luminosity, Mass, Temperature, Time};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarData {
    pub(super) name: String,
    pub(super) constellation: Option<String>,
    pub(super) params: StarPhysicalParameters,
    pub(super) pos: Cartesian,
    pub(super) evolution: StarDataEvolution,
}

impl StarData {
    pub fn new(
        name: String,
        constellation: Option<String>,
        params: StarPhysicalParameters,
        pos: Cartesian,
        evolution: StarDataEvolution,
    ) -> Self {
        Self {
            name,
            params,
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

    pub const fn get_mass_at_epoch(&self) -> Option<Mass<f64>> {
        self.params.mass
    }

    pub fn get_mass(&self, time: Time<f64>) -> Option<Mass<f64>> {
        Some(self.evolution.apply_to_mass(self.params.mass?, time))
    }

    pub const fn get_radius_at_epoch(&self) -> Option<Distance<f64>> {
        self.params.radius
    }

    pub fn get_radius(&self, time: Time<f64>) -> Option<Distance<f64>> {
        Some(self.evolution.apply_to_radius(self.params.radius?, time))
    }

    pub const fn get_luminous_intensity_at_epoch(&self) -> Luminosity<f64> {
        self.params.luminous_intensity
    }

    pub fn get_luminous_intensity(&self, time: Time<f64>) -> Luminosity<f64> {
        self.evolution
            .apply_to_luminous_intensity(self.params.luminous_intensity, time)
    }

    pub const fn get_temperature_at_epoch(&self) -> Temperature<f64> {
        self.params.temperature
    }

    pub fn get_temperature(&self, time: Time<f64>) -> Temperature<f64> {
        self.evolution
            .apply_to_temperature(self.params.temperature, time)
    }

    pub const fn get_age_at_epoch(&self) -> Option<Time<f64>> {
        self.evolution.age
    }

    pub fn get_age(&self, time: Time<f64>) -> Option<Time<f64>> {
        self.evolution.age.map(|age| age + time)
    }

    pub fn get_distance_at_epoch(&self) -> Distance<f64> {
        self.pos.length()
    }

    pub fn get_distance(&self, time: Time<f64>) -> Distance<f64> {
        self.get_pos(time).length()
    }

    pub const fn get_pos_at_epoch(&self) -> &Cartesian {
        &self.pos
    }

    pub fn get_pos(&self, _time: Time<f64>) -> Cartesian {
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
        self.params.mass = mass;
    }

    pub fn set_radius_at_epoch(&mut self, radius: Option<Distance<f64>>) {
        self.params.radius = radius;
    }

    pub fn set_luminous_intensity_at_epoch(&mut self, luminous_intensity: Luminosity<f64>) {
        self.params.luminous_intensity = luminous_intensity;
    }

    pub fn set_temperature_at_epoch(&mut self, temperature: Temperature<f64>) {
        self.params.temperature = temperature;
    }

    pub fn set_age_at_epoch(&mut self, age: Option<Time<f64>>) {
        self.evolution.age = age;
    }

    pub fn set_distance_at_epoch(&mut self, distance: Distance<f64>) {
        let direction = match self.pos.to_direction() {
            Ok(direction) => direction,
            Err(_) => return,
        };
        self.pos = direction.to_cartesian(distance);
    }

    pub fn set_pos_at_epoch(&mut self, pos: Cartesian) {
        self.pos = pos;
    }

    pub fn get_evolution(&self) -> &StarDataEvolution {
        &self.evolution
    }

    pub fn has_changed(&self, then: Time<f64>, now: Time<f64>) -> bool {
        self.evolution.has_changed(then, now)
    }

    pub fn to_star_appearance(&self, time_since_epoch: Time<f64>) -> StarAppearance {
        let luminous_intensity = self.get_luminous_intensity(time_since_epoch);
        let illuminance = luminous_intensity_to_illuminance(
            &luminous_intensity,
            &self.get_distance(time_since_epoch),
        );

        let color = sRGBColor::from_temperature(self.get_temperature(time_since_epoch));

        let pos = self
            .get_pos(time_since_epoch)
            .to_ecliptic()
            .unwrap_or(Ecliptic::X_DIRECTION);

        StarAppearance {
            name: self.name.clone(),
            illuminance,
            color,
            pos,
            time_since_epoch,
        }
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
            if star.params.mass.unwrap() < Mass::from_solar_mass(8.0) {
                assert_eq!(star.get_fate(), &StarFate::WhiteDwarf);
            }
        }
    }

    #[test]
    fn stars_above_8_sun_masses_go_supernova() {
        let star_data: Vec<StarData> = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        for star in star_data {
            if star.params.mass.unwrap() > Mass::from_solar_mass(8.0) {
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
                    Some(star.get_luminous_intensity_at_epoch().cd),
                    Some(star.get_luminous_intensity(TIME_ZERO).cd)
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
            assert!(kinda_equal(
                Some(star.get_pos_at_epoch().x.to_lyr()),
                Some(star.get_pos(TIME_ZERO).x.to_lyr())
            ));
            assert!(kinda_equal(
                Some(star.get_pos_at_epoch().y.to_lyr()),
                Some(star.get_pos(TIME_ZERO).y.to_lyr())
            ));
            assert!(kinda_equal(
                Some(star.get_pos_at_epoch().z.to_lyr()),
                Some(star.get_pos(TIME_ZERO).z.to_lyr())
            ));
        }
    }
}
