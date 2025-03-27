use serde::{Deserialize, Serialize};
use uom::si::{
    f64::{Mass, ThermodynamicTemperature, Time},
    length::Length,
};

use crate::units::{
    distance::DISTANCE_ZERO, luminous_intensity::LUMINOSITY_ZERO, mass::MASS_ZERO,
    temperature::TEMPERATURE_ZERO, time::TIME_ZERO,
};

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
        lifetime: TIME_ZERO,
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
        let lifetime = Time::from_Gyr(10.) * mass.to_solar_mass().powf(-2.5); //TODO: find a better formula
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
            const DEATH_TIMESCALE: Time = Time {
                s: -10. * 365.25 * 24. * 60. * 60.,
            }; // 10 years (negative because it counts time until death)

            let has_crossed_death = until_death_then.s.signum() != until_death_now.s.signum();
            let shortly_after_death = DEATH_TIMESCALE.s..0.;
            let then_was_shortly_after_death = shortly_after_death.contains(&until_death_then.s);
            let now_is_shortly_after_death = shortly_after_death.contains(&until_death_now.s);
            if has_crossed_death || then_was_shortly_after_death || now_is_shortly_after_death {
                return true;
            }
        }

        let diff = Time {
            s: (then.s - now.s).abs(),
        };
        const EVOLUTION_TIMESCALE: Time = Time {
            s: 1_000. * 365.25 * 24. * 60. * 60.,
        }; // 1_000 years

        if self.lifestage_evolution.is_some() && diff > EVOLUTION_TIMESCALE {
            return true;
        }
        false
    }

    pub(super) fn time_until_death(&self, time_since_epoch: Time) -> Option<Time> {
        self.age.map(|age| self.lifetime - age - time_since_epoch)
    }

    pub(crate) fn apply_to_mass(&self, mass: Mass, time_since_epoch: Time) -> Mass {
        if let Some(time_until_death) = self.time_until_death(time_since_epoch) {
            if time_until_death < TIME_ZERO {
                return self.fate.apply_to_mass(mass);
            }
        }
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return mass + lifestage_evolution.mass_per_year * time_since_epoch.to_yr();
        }
        mass
    }

    pub(crate) fn apply_to_radius(&self, radius: Length, time_since_epoch: Time) -> Length {
        if let Some(time_until_death) = self.time_until_death(time_since_epoch) {
            if time_until_death < TIME_ZERO {
                return self.fate.apply_to_radius();
            }
        }
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return radius + lifestage_evolution.radius_per_year * time_since_epoch.to_yr();
        }
        radius
    }

    pub(crate) fn apply_to_luminous_intensity(
        &self,
        luminous_intensity: Luminosity<f64>,
        time_since_epoch: Time,
    ) -> Luminosity<f64> {
        if let Some(time_until_death) = self.time_until_death(time_since_epoch) {
            if time_until_death < TIME_ZERO {
                return self
                    .fate
                    .apply_to_luminous_intensity(luminous_intensity, -time_until_death);
            }
        }
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return luminous_intensity
                + lifestage_evolution.luminous_intensity_per_year * time_since_epoch.to_yr();
        }
        luminous_intensity
    }

    pub(crate) fn apply_to_temperature(
        &self,
        temperature: ThermodynamicTemperature,
        time_since_epoch: Time,
    ) -> ThermodynamicTemperature {
        if let Some(time_until_death) = self.time_until_death(time_since_epoch) {
            if time_until_death < TIME_ZERO {
                return self
                    .fate
                    .apply_to_temperature(temperature, -time_until_death);
            }
        }
        if let Some(lifestage_evolution) = &self.lifestage_evolution {
            return temperature
                + lifestage_evolution.temperature_per_year * time_since_epoch.to_yr();
        }
        temperature
    }

    pub fn get_lifestage_mass_per_year(&self) -> Mass {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.mass_per_year)
            .unwrap_or(MASS_ZERO)
    }

    pub fn get_lifestage_radius_per_year(&self) -> Length {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.radius_per_year)
            .unwrap_or(DISTANCE_ZERO)
    }

    pub fn get_lifestage_luminous_intensity_per_year(&self) -> Luminosity<f64> {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.luminous_intensity_per_year)
            .unwrap_or(LUMINOSITY_ZERO)
    }

    pub fn get_lifestage_temperature_per_year(&self) -> ThermodynamicTemperature {
        self.lifestage_evolution
            .as_ref()
            .map(|e| e.temperature_per_year)
            .unwrap_or(TEMPERATURE_ZERO)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct StarDataLifestageEvolution {
    mass_per_year: Mass,
    radius_per_year: Length,
    luminous_intensity_per_year: Luminosity<f64>,
    temperature_per_year: ThermodynamicTemperature,
}

impl StarDataLifestageEvolution {
    pub(crate) fn new(now: &StarData, then: &StarData, years: f64) -> Self {
        let mass_per_year = match (now.params.mass, then.params.mass) {
            (Some(now_mass), Some(then_mass)) => (now_mass - then_mass) / years,
            _ => MASS_ZERO,
        };
        let radius_per_year = match (now.params.radius, then.params.radius) {
            (Some(now_radius), Some(then_radius)) => (now_radius - then_radius) / years,
            _ => DISTANCE_ZERO,
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
    use super::*;

    #[test]
    fn has_changed_is_symmetric() {
        let times = vec![
            Time::from_yr(0.),
            Time::from_yr(1.),
            Time::from_yr(10.),
            Time::from_yr(100.),
            Time::from_yr(1_000.),
            Time::from_yr(10_000.),
            Time::from_yr(-1.),
            Time::from_yr(-10.),
            Time::from_yr(-100.),
            Time::from_yr(-1_000.),
            Time::from_yr(-10_000.),
        ];
        let evolution =
            StarDataEvolution::new(None, None, Time::from_yr(10_000.), StarFate::WhiteDwarf);
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
        let then = Time::from_yr(0.);
        let now = Time::from_yr(2000.);
        let lifestage_evolution = StarDataLifestageEvolution {
            mass_per_year: MASS_ZERO,
            radius_per_year: DISTANCE_ZERO,
            luminous_intensity_per_year: LUMINOSITY_ZERO,
            temperature_per_year: TEMPERATURE_ZERO,
        };
        let evolution = StarDataEvolution::new(
            Some(lifestage_evolution),
            None,
            Time::from_yr(10_000.),
            StarFate::WhiteDwarf,
        );
        assert!(evolution.has_changed(then, now));
        assert!(evolution.has_changed(now, then));
    }

    #[test]
    fn star_has_changed_when_it_crosses_death() {
        let then = Time::from_yr(0.);
        let now = Time::from_yr(10_000.);
        let age = Some(Time::from_yr(1_000.));
        let lifetime = Time::from_yr(5_000.);
        let evolution = StarDataEvolution::new(None, age, lifetime, StarFate::WhiteDwarf);
        assert!(evolution.has_changed(then, now));
        assert!(evolution.has_changed(now, then));
    }

    #[test]
    fn star_changes_rapidly_shortly_after_death() {
        let lifetime = Time::from_Gyr(1.);
        let small_steps = vec![
            Time::from_s(1.),
            Time::from_min(1.),
            Time::from_hr(1.),
            Time::from_days(1.),
            Time::from_yr(1.),
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
