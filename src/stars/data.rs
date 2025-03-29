use super::{
    appearance::StarAppearance, evolution::StarDataEvolution, fate::StarFate,
    physical_parameters::StarPhysicalParameters,
};
use crate::{color::srgb::sRGBColor, units::luminous_intensity::luminous_intensity_to_illuminance};
use astro_coords::{cartesian::Cartesian, ecliptic::Ecliptic};
use serde::{Deserialize, Serialize};
use uom::si::f64::{Length, Mass, ThermodynamicTemperature, Time};

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

    pub const fn get_mass_at_epoch(&self) -> Option<Mass> {
        self.params.mass
    }

    pub fn get_mass(&self, time: Time) -> Option<Mass> {
        Some(self.evolution.apply_to_mass(self.params.mass?, time))
    }

    pub const fn get_radius_at_epoch(&self) -> Option<Length> {
        self.params.radius
    }

    pub fn get_radius(&self, time: Time) -> Option<Length> {
        Some(self.evolution.apply_to_radius(self.params.radius?, time))
    }

    pub const fn get_luminous_intensity_at_epoch(&self) -> Luminosity<f64> {
        self.params.luminous_intensity
    }

    pub fn get_luminous_intensity(&self, time: Time) -> Luminosity<f64> {
        self.evolution
            .apply_to_luminous_intensity(self.params.luminous_intensity, time)
    }

    pub const fn get_temperature_at_epoch(&self) -> ThermodynamicTemperature {
        self.params.temperature
    }

    pub fn get_temperature(&self, time: Time) -> ThermodynamicTemperature {
        self.evolution
            .apply_to_temperature(self.params.temperature, time)
    }

    pub const fn get_age_at_epoch(&self) -> Option<Time> {
        self.evolution.age
    }

    pub fn get_age(&self, time: Time) -> Option<Time> {
        self.evolution.age.map(|age| age + time)
    }

    pub fn get_distance_at_epoch(&self) -> Length {
        self.pos.length()
    }

    pub fn get_distance(&self, time: Time) -> Length {
        self.get_pos(time).length()
    }

    pub const fn get_pos_at_epoch(&self) -> &Cartesian {
        &self.pos
    }

    pub fn get_pos(&self, _time: Time) -> Cartesian {
        self.pos.clone()
    }

    pub fn get_time_until_death(&self, time_since_epoch: Time) -> Option<Time> {
        self.evolution.time_until_death(time_since_epoch)
    }

    pub fn get_lifetime(&self) -> Time {
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

    pub fn set_mass_at_epoch(&mut self, mass: Option<Mass>) {
        self.params.mass = mass;
    }

    pub fn set_radius_at_epoch(&mut self, radius: Option<Length>) {
        self.params.radius = radius;
    }

    pub fn set_luminous_intensity_at_epoch(&mut self, luminous_intensity: Luminosity<f64>) {
        self.params.luminous_intensity = luminous_intensity;
    }

    pub fn set_temperature_at_epoch(&mut self, temperature: ThermodynamicTemperature) {
        self.params.temperature = temperature;
    }

    pub fn set_age_at_epoch(&mut self, age: Option<Time>) {
        self.evolution.age = age;
    }

    pub fn set_distance_at_epoch(&mut self, distance: Length) {
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

    pub fn has_changed(&self, then: Time, now: Time) -> bool {
        self.evolution.has_changed(then, now)
    }

    pub fn to_star_appearance(&self, time_since_epoch: Time) -> StarAppearance {
        let luminous_intensity = self.get_luminous_intensity(time_since_epoch);
        let illuminance = luminous_intensity_to_illuminance(
            &luminous_intensity,
            &self.get_distance(time_since_epoch),
        );

        let color = sRGBColor::from_temperature(self.get_temperature(time_since_epoch));

        let pos = self
            .get_pos(time_since_epoch)
            .to_ecliptic()
            .unwrap_or(Ecliptic::x_direction());

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

        let mass_ratio = match (self.params.mass, other.params.mass) {
            (Some(self_mass), Some(other_mass)) => self_mass / other_mass,
            _ => 1.0,
        };
        let radius_ratio = match (self.params.radius, other.params.radius) {
            (Some(self_radius), Some(other_radius)) => self_radius / other_radius,
            _ => 1.0,
        };
        let luminous_intensity_difference =
            (luminous_intensity_to_absolute_magnitude(self.params.luminous_intensity)
                - luminous_intensity_to_absolute_magnitude(other.params.luminous_intensity))
            .abs();
        let temperature_ratio = self.params.temperature / other.params.temperature;
        let age_ratio = match (self.evolution.age, other.evolution.age) {
            (Some(self_age), Some(other_age)) => self_age / other_age,
            _ => 1.0,
        };
        let mut result = true;
        if mass_ratio < 0.1 || mass_ratio > 10.0 {
            println!(
                "mass1: {}, mass2: {}, ratio: {}",
                self.params.mass.unwrap(),
                other.params.mass.unwrap(),
                mass_ratio
            );
            result = false;
        }
        if radius_ratio < 0.1 || radius_ratio > 10.0 {
            println!(
                "radius1: {}, radius2: {}, ratio: {}",
                self.params.radius.unwrap(),
                other.params.radius.unwrap(),
                radius_ratio
            );
            result = false;
        }
        if luminous_intensity_difference > 1.0 {
            println!(
                "luminous_intensity1: {}, luminous_intensity2: {}, difference: {}",
                self.params.luminous_intensity,
                other.params.luminous_intensity,
                luminous_intensity_difference
            );
            result = false;
        }
        if temperature_ratio < 0.1 || temperature_ratio > 10.0 {
            println!(
                "temperature1: {}, temperature2: {}, ratio: {}",
                self.params.temperature, other.params.temperature, temperature_ratio
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
    use uom::si::thermodynamic_temperature::kelvin;

    use super::*;
    use crate::real_data::stars::all::get_many_stars;

    #[test]
    fn real_stars_have_a_non_vanishing_lifetime() {
        let star_data: Vec<StarData> = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        for star in star_data {
            assert!(star.get_lifetime() > Time::new::<year>(0.));
        }
    }

    #[test]
    fn real_stars_have_not_died_yet() {
        let star_data: Vec<StarData> = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        for star in star_data {
            if let Some(time_until_death) = star.get_time_until_death(Time::new::<year>(0.)) {
                assert!(time_until_death > Time::new::<year>(0.));
            }
        }
    }

    #[test]
    fn stars_below_8_sun_masses_become_white_dwarfs() {
        let star_data: Vec<StarData> = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        for star in star_data {
            if star.params.mass.unwrap() < Mass::new::<solar_mass>(8.0) {
                assert_eq!(star.get_fate(), &StarFate::WhiteDwarf);
            }
        }
    }

    #[test]
    fn stars_above_8_sun_masses_go_supernova() {
        let star_data: Vec<StarData> = get_many_stars().iter().map(|s| s.to_star_data()).collect();
        for star in star_data {
            if star.params.mass.unwrap() > Mass::new::<solar_mass>(8.0) {
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
                    star.get_mass(Time::new::<year>(0.)).map(|m| m.kg),
                ),
                "Star {}\nMass {:?}\nMass {:?}",
                star.get_name(),
                star.get_mass_at_epoch(),
                star.get_mass(Time::new::<year>(0.))
            );
            assert!(
                kinda_equal(
                    star.get_radius_at_epoch().map(|r| r.m),
                    star.get_radius(Time::new::<year>(0.)).map(|r| r.m)
                ),
                "Star {}\nRadius {:?}\nRadius {:?}",
                star.get_name(),
                star.get_radius_at_epoch(),
                star.get_radius(Time::new::<year>(0.))
            );
            assert!(
                kinda_equal(
                    Some(star.get_luminous_intensity_at_epoch().cd),
                    Some(star.get_luminous_intensity(Time::new::<year>(0.)).cd)
                ),
                "Star {}\nLuminous intensity {:?}\nLuminous intensity {:?}",
                star.get_name(),
                star.get_luminous_intensity_at_epoch(),
                star.get_luminous_intensity(Time::new::<year>(0.))
            );
            assert!(
                kinda_equal(
                    Some(star.get_temperature_at_epoch().get::<kelvin>()),
                    Some(star.get_temperature(Time::new::<year>(0.)).get::<kelvin>())
                ),
                "Star {}\nTemperature {:?}\nTemperature {:?}",
                star.get_name(),
                star.get_temperature_at_epoch(),
                star.get_temperature(Time::new::<year>(0.))
            );
            assert!(
                kinda_equal(
                    star.get_age_at_epoch().map(|t| t.s),
                    star.get_age(Time::new::<year>(0.)).map(|t| t.s)
                ),
                "Star {}\nAge {:?}\nAge {:?}",
                star.get_name(),
                star.get_age_at_epoch(),
                star.get_age(Time::new::<year>(0.))
            );
            assert!(
                kinda_equal(
                    Some(star.get_distance_at_epoch().m),
                    Some(star.get_distance(Time::new::<year>(0.)).m)
                ),
                "Star {}\nLength {:?}\nLength {:?}",
                star.get_name(),
                star.get_distance_at_epoch(),
                star.get_distance(Time::new::<year>(0.))
            );
            assert!(kinda_equal(
                Some(star.get_pos_at_epoch().x.to_lyr()),
                Some(star.get_pos(Time::new::<year>(0.)).x.to_lyr())
            ));
            assert!(kinda_equal(
                Some(star.get_pos_at_epoch().y.to_lyr()),
                Some(star.get_pos(Time::new::<year>(0.)).y.to_lyr())
            ));
            assert!(kinda_equal(
                Some(star.get_pos_at_epoch().z.to_lyr()),
                Some(star.get_pos(Time::new::<year>(0.)).z.to_lyr())
            ));
        }
    }
}
