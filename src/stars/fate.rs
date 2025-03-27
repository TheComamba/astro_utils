use std::ops::Range;

use serde::{Deserialize, Serialize};
use uom::si::{
    f64::{Length, Mass, ThermodynamicTemperature, Time},
    length::kilometer,
    time::day,
};

use crate::{
    astro_display::AstroDisplay,
    units::{
        distance::SOLAR_RADIUS,
        luminous_intensity::{
            absolute_magnitude_to_luminous_intensity, luminous_intensity_to_absolute_magnitude,
        },
        temperature::TEMPERATURE_ZERO,
    },
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum StarFate {
    WhiteDwarf,
    TypeIISupernova,
}

impl StarFate {
    pub(crate) fn new(initial_mass: Mass) -> Self {
        if initial_mass < Mass::from_solar_mass(8.) {
            StarFate::WhiteDwarf
        } else {
            StarFate::TypeIISupernova
        }
    }

    pub(crate) fn apply_to_mass(&self, mass: Mass) -> Mass {
        match self {
            StarFate::WhiteDwarf => 0.3 * mass, // rough estimate after shedding outer layers
            StarFate::TypeIISupernova => {
                if mass < Mass::from_solar_mass(25.) {
                    Mass::from_solar_mass(1.4) // Typical Neutron Star
                } else {
                    Mass::from_solar_mass(7.0) // Typical Black Hole
                }
            }
        }
    }

    pub(crate) fn apply_to_radius(&self) -> Length {
        match self {
            StarFate::WhiteDwarf => 0.0084 * SOLAR_RADIUS, // Sirius B
            StarFate::TypeIISupernova => Length::new::<kilometer>(20.), // Neutron star or black hole
        }
    }

    pub(crate) fn apply_to_luminous_intensity(
        &self,
        luminous_intensity: Luminosity<f64>,
        time_since_death: Time,
    ) -> Luminosity<f64> {
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
            StarFate::WhiteDwarf => ThermodynamicTemperature::from_celsius(25_000.), // Sirius B
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
    initial: Luminosity<f64>,
    time_since_death: Time,
) -> Luminosity<f64> {
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
    const PEAK_TEMPERATURE: ThermodynamicTemperature = Temperature { K: 100_000. };
    const PLATEAU_TEMPERATURE: ThermodynamicTemperature = Temperature { K: 4_500. };

    let days = time_since_death.get::<day>();
    if days < 0. {
        initial
    } else if SN_PHASE_1_INCREASE.contains(&days) {
        let offset = initial;
        let slope =
            (PEAK_TEMPERATURE - offset) / (SN_PHASE_1_INCREASE.end - SN_PHASE_1_INCREASE.start);
        offset + slope * (days - SN_PHASE_1_INCREASE.start)
    } else if SN_PHASE_2_DECREASE.contains(&days) {
        let offset = PEAK_TEMPERATURE;
        let slope =
            (PLATEAU_TEMPERATURE - offset) / (SN_PHASE_2_DECREASE.end - SN_PHASE_2_DECREASE.start);
        offset + slope * (days - SN_PHASE_2_DECREASE.start)
    } else if SN_PHASE_3_PLATEAU.contains(&days) {
        PLATEAU_TEMPERATURE
    } else {
        let offset = PLATEAU_TEMPERATURE;
        let slope = -PLATEAU_TEMPERATURE / Time::from_kyr(10.).get::<day>();
        let t = offset + slope * (days - SN_PHASE_3_PLATEAU.end);
        if t > TEMPERATURE_ZERO {
            t
        } else {
            TEMPERATURE_ZERO
        }
    }
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
    use crate::units::luminous_intensity::SOLAR_LUMINOUS_INTENSITY;

    use super::*;

    #[test]
    fn type_2_supernova_luminous_intensity_is_smooth() {
        let initial = absolute_magnitude_to_luminous_intensity(-16.);
        let mut last = initial;
        for count in -3..20_000 {
            let time_since_death = Time::from_days(count as f64 / 25.);
            let current = type_2_supernova_luminous_intensity(initial, time_since_death);
            let diff = current - last;
            assert!(
                diff.cd.abs() < 1e-1 * current.cd.abs(),
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
        let initial = 10. * SOLAR_LUMINOUS_INTENSITY;
        let mut last = initial;
        for days in (SN_PHASE_1_INCREASE.start as i32 + 1)..=SN_PHASE_1_INCREASE.end as i32 {
            let time_since_death = Time::from_days(days as f64);
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
        let initial = 10. * SOLAR_LUMINOUS_INTENSITY;
        let last_time = Time::from_days(SN_PHASE_2_DECREASE.start);
        let mut last = type_2_supernova_luminous_intensity(initial, last_time);
        for days in (SN_PHASE_2_DECREASE.start as i32 + 1)..=SN_PHASE_2_DECREASE.end as i32 {
            let time_since_death = Time::from_days(days as f64);
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
        let initial = 10. * SOLAR_LUMINOUS_INTENSITY;
        let last_time = Time::from_days(SN_PHASE_3_PLATEAU.start);
        let mut last = type_2_supernova_luminous_intensity(initial, last_time);
        for days in (SN_PHASE_3_PLATEAU.start as i32 + 1)..=SN_PHASE_3_PLATEAU.end as i32 {
            let time_since_death = Time::from_days(days as f64);
            let current = type_2_supernova_luminous_intensity(initial, time_since_death);
            let diff = current - last;
            assert!(
                diff.cd < 1.,
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
        let initial = 10. * SOLAR_LUMINOUS_INTENSITY;
        let last_time = Time::from_days(SN_PHASE_2_DECREASE.end);
        let mut last = type_2_supernova_luminous_intensity(initial, last_time);
        for days in (SN_PHASE_3_PLATEAU.end as i32 + 1)..(SN_PHASE_3_PLATEAU.end as i32 + 100) {
            let time_since_death = Time::from_days(days as f64);
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
        let initial = ThermodynamicTemperature::from_celsius(50_000.);
        let mut last = initial;
        for count in -3..20_000 {
            let time_since_death = Time::from_days(count as f64 / 25.);
            let current = type_2_supernova_temperature(initial, time_since_death);
            let diff = current - last;
            assert!(
                diff.value.abs() < 1e-1 * current.value.abs(),
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
        let initial = ThermodynamicTemperature::from_celsius(5_000.);
        let mut last = initial;
        for days in (SN_PHASE_1_INCREASE.start as i32 + 1)..=SN_PHASE_1_INCREASE.end as i32 {
            let time_since_death = Time::from_days(days as f64);
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
        let initial = ThermodynamicTemperature::from_celsius(5_000.);
        let last_time = Time::from_days(SN_PHASE_2_DECREASE.start);
        let mut last = type_2_supernova_temperature(initial, last_time);
        for days in (SN_PHASE_2_DECREASE.start as i32 + 1)..=SN_PHASE_2_DECREASE.end as i32 {
            let time_since_death = Time::from_days(days as f64);
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
        let initial = ThermodynamicTemperature::from_celsius(5_000.);
        let last_time = Time::from_days(SN_PHASE_3_PLATEAU.start);
        let mut last = type_2_supernova_temperature(initial, last_time);
        for days in (SN_PHASE_3_PLATEAU.start as i32 + 1)..=SN_PHASE_3_PLATEAU.end as i32 {
            let time_since_death = Time::from_days(days as f64);
            let current = type_2_supernova_temperature(initial, time_since_death);
            let diff = current - last;
            assert!(
                diff.get::<kelvin>() < 1.,
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
        let initial = ThermodynamicTemperature::from_celsius(5_000.);
        let last_time = Time::from_days(SN_PHASE_3_PLATEAU.end);
        let mut last = type_2_supernova_temperature(initial, last_time);
        for days in (SN_PHASE_3_PLATEAU.end as i32 + 1)..(SN_PHASE_3_PLATEAU.end as i32 + 100) {
            let time_since_death = Time::from_days(days as f64);
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
