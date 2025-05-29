use std::ops::Range;

use serde::{Deserialize, Serialize};
use uom::si::{
    f64::{Length, LuminousIntensity, Mass, ThermodynamicTemperature, Time},
    length::kilometer,
    thermodynamic_temperature::kelvin,
    time::day,
};

use crate::{
    astro_display::AstroDisplay,
    units::{
        length::solar_radii,
        luminous_intensity::{
            absolute_magnitude_to_luminous_intensity, luminous_intensity_to_absolute_magnitude,
        },
        mass::solar_mass,
        time::kiloyear,
    },
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum StarFate {
    WhiteDwarf,
    TypeIISupernova,
}

impl StarFate {
    pub(crate) fn new(initial_mass: Mass) -> Self {
        if initial_mass < Mass::new::<solar_mass>(8.) {
            StarFate::WhiteDwarf
        } else {
            StarFate::TypeIISupernova
        }
    }

    pub(crate) fn apply_to_mass(&self, mass: Mass) -> Mass {
        match self {
            StarFate::WhiteDwarf => 0.3 * mass, // rough estimate after shedding outer layers
            StarFate::TypeIISupernova => {
                if mass < Mass::new::<solar_mass>(25.) {
                    Mass::new::<solar_mass>(1.4) // Typical Neutron Star
                } else {
                    Mass::new::<solar_mass>(7.0) // Typical Black Hole
                }
            }
        }
    }

    pub(crate) fn apply_to_radius(&self) -> Length {
        match self {
            StarFate::WhiteDwarf => Length::new::<solar_radii>(0.0084), // Sirius B
            StarFate::TypeIISupernova => Length::new::<kilometer>(20.), // Neutron star or black hole
        }
    }

    pub(crate) fn apply_to_luminous_intensity(
        &self,
        luminous_intensity: LuminousIntensity,
        time_since_death: Time,
    ) -> LuminousIntensity {
        match self {
            StarFate::WhiteDwarf => absolute_magnitude_to_luminous_intensity(11.18), // Sirius B
            StarFate::TypeIISupernova => {
                type_2_supernova_luminous_intensity(luminous_intensity, time_since_death)
            }
        }
    }

    pub(crate) fn apply_to_temperature(
        &self,
        temperature: ThermodynamicTemperature,
        time_since_death: Time,
    ) -> ThermodynamicTemperature {
        match self {
            StarFate::WhiteDwarf => ThermodynamicTemperature::new::<kelvin>(25_000.), // Sirius B
            StarFate::TypeIISupernova => {
                type_2_supernova_temperature(temperature, time_since_death)
            }
        }
    }
}

const SN_PHASE_1_INCREASE: Range<f64> = 0.0..10.0;
const SN_PHASE_2_DECREASE: Range<f64> = 10.0..20.0;
const SN_PHASE_3_PLATEAU: Range<f64> = 20.0..110.0;
pub(crate) const TYPE_II_SUPERNOVA_PEAK_MAGNITUDE: f64 = -16.8;

fn type_2_supernova_luminous_intensity(
    initial: LuminousIntensity,
    time_since_death: Time,
) -> LuminousIntensity {
    const PLATEAU_MAGNITUDE: f64 = -16.3;

    let days = time_since_death.get::<day>();
    if days < 0. {
        initial
    } else if SN_PHASE_1_INCREASE.contains(&days) {
        let offset = luminous_intensity_to_absolute_magnitude(initial);
        let slope = (TYPE_II_SUPERNOVA_PEAK_MAGNITUDE - offset)
            / (SN_PHASE_1_INCREASE.end - SN_PHASE_1_INCREASE.start);
        let mag = offset + slope * (days - SN_PHASE_1_INCREASE.start);
        absolute_magnitude_to_luminous_intensity(mag)
    } else if SN_PHASE_2_DECREASE.contains(&days) {
        let offset = TYPE_II_SUPERNOVA_PEAK_MAGNITUDE;
        let slope =
            (PLATEAU_MAGNITUDE - offset) / (SN_PHASE_2_DECREASE.end - SN_PHASE_2_DECREASE.start);
        let mag = offset + slope * (days - SN_PHASE_2_DECREASE.start);
        absolute_magnitude_to_luminous_intensity(mag)
    } else if SN_PHASE_3_PLATEAU.contains(&days) {
        absolute_magnitude_to_luminous_intensity(PLATEAU_MAGNITUDE)
    } else {
        let offset = PLATEAU_MAGNITUDE;
        let slope = 1. / 60.;
        let mag = offset + slope * (days - SN_PHASE_3_PLATEAU.end);
        absolute_magnitude_to_luminous_intensity(mag)
    }
}

fn type_2_supernova_temperature(
    initial: ThermodynamicTemperature,
    time_since_death: Time,
) -> ThermodynamicTemperature {
    let peak_temperature = 100_000.;
    let plateau_temperature = 4_500.;

    let days = time_since_death.get::<day>();
    let t = if days < 0. {
        initial.get::<kelvin>()
    } else if SN_PHASE_1_INCREASE.contains(&days) {
        let offset = initial.get::<kelvin>();
        let slope =
            (peak_temperature - offset) / (SN_PHASE_1_INCREASE.end - SN_PHASE_1_INCREASE.start);
        offset + slope * (days - SN_PHASE_1_INCREASE.start)
    } else if SN_PHASE_2_DECREASE.contains(&days) {
        let offset = peak_temperature;
        let slope =
            (plateau_temperature - offset) / (SN_PHASE_2_DECREASE.end - SN_PHASE_2_DECREASE.start);
        offset + slope * (days - SN_PHASE_2_DECREASE.start)
    } else if SN_PHASE_3_PLATEAU.contains(&days) {
        plateau_temperature
    } else {
        let offset = plateau_temperature;
        let slope = -plateau_temperature / Time::new::<kiloyear>(10.).get::<day>();
        let t = offset + slope * (days - SN_PHASE_3_PLATEAU.end);
        if t > 0. {
            t
        } else {
            0.
        }
    };
    ThermodynamicTemperature::new::<kelvin>(t)
}

impl AstroDisplay for StarFate {
    fn astro_display(&self) -> String {
        match self {
            StarFate::WhiteDwarf => "White Dwarf".to_string(),
            StarFate::TypeIISupernova => "Type II Supernova".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use uom::si::luminous_intensity::candela;

    use crate::units::luminous_intensity::solar_luminous_intensity;

    use super::*;

    #[test]
    fn type_2_supernova_luminous_intensity_is_smooth() {
        let initial = absolute_magnitude_to_luminous_intensity(-16.);
        let mut last = initial;
        for count in -3..20_000 {
            let time_since_death = Time::new::<day>(count as f64 / 25.);
            let current = type_2_supernova_luminous_intensity(initial, time_since_death);
            let diff = current - last;
            assert!(
                diff.get::<candela>().abs() < 1e-1 * current.get::<candela>().abs(),
                "days: {} current: {} last: {}",
                time_since_death.astro_display(),
                current.astro_display(),
                last.astro_display()
            );
            last = current;
        }
    }

    #[test]
    fn type_2_supernova_luminous_intensity_increases_during_phase_1() {
        let initial = 10. * solar_luminous_intensity();
        let mut last = initial;
        for days in (SN_PHASE_1_INCREASE.start as i32 + 1)..=SN_PHASE_1_INCREASE.end as i32 {
            let time_since_death = Time::new::<day>(days as f64);
            let current = type_2_supernova_luminous_intensity(initial, time_since_death);
            assert!(
                current > last,
                "days: {} current: {} last: {}",
                days,
                current.astro_display(),
                last.astro_display()
            );
            last = current;
        }
    }

    #[test]
    fn type_2_supernova_luminous_intensity_decreases_during_phase_2() {
        let initial = 10. * solar_luminous_intensity();
        let last_time = Time::new::<day>(SN_PHASE_2_DECREASE.start);
        let mut last = type_2_supernova_luminous_intensity(initial, last_time);
        for days in (SN_PHASE_2_DECREASE.start as i32 + 1)..=SN_PHASE_2_DECREASE.end as i32 {
            let time_since_death = Time::new::<day>(days as f64);
            let current = type_2_supernova_luminous_intensity(initial, time_since_death);
            assert!(
                current < last,
                "days: {} current: {} last: {}",
                days,
                current.astro_display(),
                last.astro_display()
            );
            last = current;
        }
    }

    #[test]
    fn type_2_supernova_luminous_intensity_plateaus_during_phase_3() {
        let initial = 10. * solar_luminous_intensity();
        let last_time = Time::new::<day>(SN_PHASE_3_PLATEAU.start);
        let mut last = type_2_supernova_luminous_intensity(initial, last_time);
        for days in (SN_PHASE_3_PLATEAU.start as i32 + 1)..=SN_PHASE_3_PLATEAU.end as i32 {
            let time_since_death = Time::new::<day>(days as f64);
            let current = type_2_supernova_luminous_intensity(initial, time_since_death);
            let diff = current - last;
            assert!(
                diff.get::<candela>() < 1.,
                "days: {} current: {} last: {}",
                days,
                current.astro_display(),
                last.astro_display()
            );
            last = current;
        }
    }

    #[test]
    fn type_2_supernova_luminous_intensity_decreases_after_phase_3() {
        let initial = 10. * solar_luminous_intensity();
        let last_time = Time::new::<day>(SN_PHASE_2_DECREASE.end);
        let mut last = type_2_supernova_luminous_intensity(initial, last_time);
        for days in (SN_PHASE_3_PLATEAU.end as i32 + 1)..(SN_PHASE_3_PLATEAU.end as i32 + 100) {
            let time_since_death = Time::new::<day>(days as f64);
            let current = type_2_supernova_luminous_intensity(initial, time_since_death);
            assert!(
                current < last,
                "days: {} current: {} last: {}",
                days,
                current.astro_display(),
                last.astro_display()
            );
            last = current;
        }
    }

    #[test]
    fn type_2_supernova_temperature_is_smooth() {
        let initial = ThermodynamicTemperature::new::<kelvin>(50_000.);
        let mut last = initial;
        for count in -3..20_000 {
            let time_since_death = Time::new::<day>(count as f64 / 25.);
            let current = type_2_supernova_temperature(initial, time_since_death);
            let diff = current.get::<kelvin>() - last.get::<kelvin>();
            assert!(
                diff.abs() < 1e-1 * current.get::<kelvin>().abs(),
                "days: {} current: {} last: {}",
                time_since_death.astro_display(),
                current.astro_display(),
                last.astro_display()
            );
            last = current;
        }
    }

    #[test]
    fn type_2_supernova_temperature_increases_during_phase_1() {
        let initial = ThermodynamicTemperature::new::<kelvin>(5_000.);
        let mut last = initial;
        for days in (SN_PHASE_1_INCREASE.start as i32 + 1)..=SN_PHASE_1_INCREASE.end as i32 {
            let time_since_death = Time::new::<day>(days as f64);
            let current = type_2_supernova_temperature(initial, time_since_death);
            assert!(
                current > last,
                "days: {} current: {} last: {}",
                days,
                current.astro_display(),
                last.astro_display()
            );
            last = current;
        }
    }

    #[test]
    fn type_2_supernova_temperature_decreases_during_phase_2() {
        let initial = ThermodynamicTemperature::new::<kelvin>(5_000.);
        let last_time = Time::new::<day>(SN_PHASE_2_DECREASE.start);
        let mut last = type_2_supernova_temperature(initial, last_time);
        for days in (SN_PHASE_2_DECREASE.start as i32 + 1)..=SN_PHASE_2_DECREASE.end as i32 {
            let time_since_death = Time::new::<day>(days as f64);
            let current = type_2_supernova_temperature(initial, time_since_death);
            assert!(
                current < last,
                "days: {} current: {} last: {}",
                days,
                current.astro_display(),
                last.astro_display()
            );
            last = current;
        }
    }

    #[test]
    fn type_2_supernova_temperature_plateaus_during_phase_3() {
        let initial = ThermodynamicTemperature::new::<kelvin>(5_000.);
        let last_time = Time::new::<day>(SN_PHASE_3_PLATEAU.start);
        let mut last = type_2_supernova_temperature(initial, last_time);
        for days in (SN_PHASE_3_PLATEAU.start as i32 + 1)..=SN_PHASE_3_PLATEAU.end as i32 {
            let time_since_death = Time::new::<day>(days as f64);
            let current = type_2_supernova_temperature(initial, time_since_death);
            let diff = current.get::<kelvin>() - last.get::<kelvin>();
            assert!(
                diff < 1.,
                "days: {} current: {} last: {}",
                days,
                current.astro_display(),
                last.astro_display()
            );
            last = current;
        }
    }

    #[test]
    fn type_2_supernova_temperature_decreases_after_phase_3() {
        let initial = ThermodynamicTemperature::new::<kelvin>(5_000.);
        let last_time = Time::new::<day>(SN_PHASE_3_PLATEAU.end);
        let mut last = type_2_supernova_temperature(initial, last_time);
        for days in (SN_PHASE_3_PLATEAU.end as i32 + 1)..(SN_PHASE_3_PLATEAU.end as i32 + 100) {
            let time_since_death = Time::new::<day>(days as f64);
            let current = type_2_supernova_temperature(initial, time_since_death);
            assert!(
                current < last,
                "days: {} current: {} last: {}",
                days,
                current.astro_display(),
                last.astro_display()
            );
            last = current;
        }
    }
}
