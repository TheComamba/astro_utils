use serde::{Deserialize, Serialize};
use uom::si::{
    f64::{Length, LuminousIntensity, Mass, ThermodynamicTemperature, Time},
    luminous_intensity::candela,
    thermodynamic_temperature::kelvin,
    time::year,
};

use crate::units::{length::solar_radii, mass::solar_mass, time::gigayear};

use super::{data::StarData, fate::StarFate};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarDataEvolution {
    lifestage_evolution: Option<StarDataLifestageEvolution>,
    pub(super) age: Option<Time>,
    pub(super) lifetime: Time,
    pub(super) fate: StarFate,
}

impl StarDataEvolution {
    pub const NONE: StarDataEvolution = StarDataEvolution {
        lifestage_evolution: None,
        age: None,
        lifetime: Time::new::<year>(0.),
        fate: StarFate::WhiteDwarf,
    };

    pub(crate) fn new(
        lifestage_evolution: Option<StarDataLifestageEvolution>,
        age: Option<Time>,
        lifetime: Time,
        fate: StarFate,
    ) -> Self {
        Self {
            lifestage_evolution,
            age,
            lifetime,
            fate,
        }
    }

    pub(crate) fn from_age_and_mass(age: Time, mass: Mass) -> Self {
        let lifetime = Time::new::<gigayear>(10.) * mass.get::<solar_mass>().powf(-2.5); //TODO: find a better formula
        let fate = StarFate::new(mass);
        Self {
            lifestage_evolution: None,
            age: Some(age),
            lifetime,
            fate,
        }
    }

    pub(super) fn has_changed(&self, then: Time, now: Time) -> bool {
        if let (Some(until_death_then), Some(until_death_now)) =
            (self.time_until_death(then), self.time_until_death(now))
        {
            // negative because it counts time until death
            let death_timescale = Time::new::<year>(-10.);
            let zero = Time::new::<year>(0.);

            let has_crossed_death =
                until_death_then.value.signum() != until_death_now.value.signum();
            let shortly_after_death = death_timescale..zero;
            let then_was_shortly_after_death = shortly_after_death.contains(&until_death_then);
            let now_is_shortly_after_death = shortly_after_death.contains(&until_death_now);
            if has_crossed_death || then_was_shortly_after_death || now_is_shortly_after_death {
                return true;
            }
        }

        let diff = (then - now).abs();
        let evolution_timescale = Time::new::<year>(1_000.);

        if self.lifestage_evolution.is_some() && diff > evolution_timescale {
            return true;
        }
        false
    }

    pub(super) fn time_until_death(&self, time_since_epoch: Time) -> Option<Time> {
        self.age.map(|age| self.lifetime - age - time_since_epoch)
    }

    pub(crate) fn apply_to_mass(&self, mass: Mass, time_since_epoch: Time) -> Mass {
        if let Some(time_until_death) = self.time_until_death(time_since_epoch) {
            if time_until_death.value < 0. {
                return self.fate.apply_to_mass(mass);
            }
        }
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return mass + lifestage_evolution.mass_per_year * time_since_epoch.get::<year>();
        }
        mass
    }

    pub(crate) fn apply_to_radius(&self, radius: Length, time_since_epoch: Time) -> Length {
        if let Some(time_until_death) = self.time_until_death(time_since_epoch) {
            if time_until_death.value < 0. {
                return self.fate.apply_to_radius();
            }
        }
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return radius + lifestage_evolution.radius_per_year * time_since_epoch.get::<year>();
        }
        radius
    }

    pub(crate) fn apply_to_luminous_intensity(
        &self,
        luminous_intensity: LuminousIntensity,
        time_since_epoch: Time,
    ) -> LuminousIntensity {
        if let Some(time_until_death) = self.time_until_death(time_since_epoch) {
            if time_until_death.value < 0. {
                return self
                    .fate
                    .apply_to_luminous_intensity(luminous_intensity, -time_until_death);
            }
        }
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return luminous_intensity
                + lifestage_evolution.luminous_intensity_per_year * time_since_epoch.get::<year>();
        }
        luminous_intensity
    }

    pub(crate) fn apply_to_temperature(
        &self,
        temperature: ThermodynamicTemperature,
        time_since_epoch: Time,
    ) -> ThermodynamicTemperature {
        if let Some(time_until_death) = self.time_until_death(time_since_epoch) {
            if time_until_death.value < 0. {
                return self
                    .fate
                    .apply_to_temperature(temperature, -time_until_death);
            }
        }
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return temperature
                + lifestage_evolution.temperature_per_year * time_since_epoch.get::<year>();
        }
        temperature
    }

    pub fn get_lifestage_mass_per_year(&self) -> Mass {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.mass_per_year)
            .unwrap_or_else(|| Mass::new::<solar_mass>(0.))
    }

    pub fn get_lifestage_radius_per_year(&self) -> Length {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.radius_per_year)
            .unwrap_or_else(|| Length::new::<solar_radii>(0.))
    }

    pub fn get_lifestage_luminous_intensity_per_year(&self) -> LuminousIntensity {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.luminous_intensity_per_year)
            .unwrap_or(LuminousIntensity::new::<candela>(0.))
    }

    pub fn get_lifestage_temperature_per_year(&self) -> ThermodynamicTemperature {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.temperature_per_year)
            .unwrap_or_else(|| ThermodynamicTemperature::new::<kelvin>(0.))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct StarDataLifestageEvolution {
    mass_per_year: Mass,
    radius_per_year: Length,
    luminous_intensity_per_year: LuminousIntensity,
    temperature_per_year: ThermodynamicTemperature,
}

impl StarDataLifestageEvolution {
    pub(crate) fn new(now: &StarData, then: &StarData, years: f64) -> Self {
        let mass_per_year = match (now.params.mass, then.params.mass) {
            (Some(now_mass), Some(then_mass)) => (now_mass - then_mass) / years,
            _ => Mass::new::<solar_mass>(0.),
        };
        let radius_per_year = match (now.params.radius, then.params.radius) {
            (Some(now_radius), Some(then_radius)) => (now_radius - then_radius) / years,
            _ => Length::new::<solar_radii>(0.),
        };
        let luminous_intensity_per_year =
            (now.params.luminous_intensity - then.params.luminous_intensity) / years;

        let temperature_per_year = (now.params.temperature - then.params.temperature) / years;
        Self {
            mass_per_year,
            radius_per_year,
            luminous_intensity_per_year,
            temperature_per_year,
        }
    }
}

#[cfg(test)]
mod tests {
    use uom::si::time::{day, hour, minute, second};

    use super::*;

    #[test]
    fn has_changed_is_symmetric() {
        let times = vec![
            Time::new::<year>(0.),
            Time::new::<year>(1.),
            Time::new::<year>(10.),
            Time::new::<year>(100.),
            Time::new::<year>(1_000.),
            Time::new::<year>(10_000.),
            Time::new::<year>(-1.),
            Time::new::<year>(-10.),
            Time::new::<year>(-100.),
            Time::new::<year>(-1_000.),
            Time::new::<year>(-10_000.),
        ];
        let evolution =
            StarDataEvolution::new(None, None, Time::new::<year>(10_000.), StarFate::WhiteDwarf);
        for now in times.clone().into_iter() {
            for then in times.clone().into_iter() {
                assert_eq!(
                    evolution.has_changed(then, now),
                    evolution.has_changed(now, then)
                );
            }
        }
    }

    #[test]
    fn star_has_changed_if_2000_years_have_passed() {
        let then = Time::new::<year>(0.);
        let now = Time::new::<year>(2000.);
        let lifestage_evolution = StarDataLifestageEvolution {
            mass_per_year: Mass::new::<solar_mass>(0.),
            radius_per_year: Length::new::<solar_radii>(0.),
            luminous_intensity_per_year: LuminousIntensity::new::<candela>(0.),
            temperature_per_year: ThermodynamicTemperature::new::<kelvin>(0.),
        };
        let evolution = StarDataEvolution::new(
            Some(lifestage_evolution),
            None,
            Time::new::<year>(10_000.),
            StarFate::WhiteDwarf,
        );
        assert!(evolution.has_changed(then, now));
        assert!(evolution.has_changed(now, then));
    }

    #[test]
    fn star_has_changed_when_it_crosses_death() {
        let then = Time::new::<year>(0.);
        let now = Time::new::<year>(10_000.);
        let age = Some(Time::new::<year>(1_000.));
        let lifetime = Time::new::<year>(5_000.);
        let evolution = StarDataEvolution::new(None, age, lifetime, StarFate::WhiteDwarf);
        assert!(evolution.has_changed(then, now));
        assert!(evolution.has_changed(now, then));
    }

    #[test]
    fn star_changes_rapidly_shortly_after_death() {
        let lifetime = Time::new::<gigayear>(1.);
        let small_steps = vec![
            Time::new::<second>(1.),
            Time::new::<minute>(1.),
            Time::new::<hour>(1.),
            Time::new::<day>(1.),
            Time::new::<year>(1.),
        ];
        let age = Some(lifetime);
        let evolution = StarDataEvolution::new(None, age, lifetime, StarFate::WhiteDwarf);
        for step1 in small_steps.clone().into_iter() {
            for step2 in small_steps.clone().into_iter() {
                let now = step1;
                let then = step1 + step2;
                assert!(evolution.has_changed(then, now));
                assert!(evolution.has_changed(now, then));
            }
        }
    }
}
