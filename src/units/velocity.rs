use uom::si::{
    f64::Velocity,
    velocity::{kilometer_per_second, meter_per_second, speed_of_light_in_vacuum},
};

use super::DISPLAY_THRESHOLD;
use crate::astro_display::AstroDisplay;

pub enum VelocityUnit {
    MetresPerSecond,
    KilometresPerSecond,
    SpeedOfLight,
}

pub fn display_velocity_in_units(velocity: &Velocity, units: VelocityUnit) -> String {
    match units {
        VelocityUnit::MetresPerSecond => format!("{:.2} m/s", velocity.get::<meter_per_second>()),
        VelocityUnit::KilometresPerSecond => {
            format!("{:.2} km/s", velocity.get::<kilometer_per_second>())
        }
        VelocityUnit::SpeedOfLight => {
            format!("{:.2} c", velocity.get::<speed_of_light_in_vacuum>())
        }
    }
}

impl AstroDisplay for Velocity {
    fn astro_display(&self) -> String {
        let units = if self.get::<speed_of_light_in_vacuum>().abs() > DISPLAY_THRESHOLD {
            VelocityUnit::SpeedOfLight
        } else if self.get::<kilometer_per_second>().abs() > DISPLAY_THRESHOLD {
            VelocityUnit::KilometresPerSecond
        } else {
            VelocityUnit::MetresPerSecond
        };
        display_velocity_in_units(self, units)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_display() {
        let velocity = Velocity::new::<meter_per_second>(1.23);
        assert_eq!(velocity.astro_display(), "1.23 m/s");
        let velocity = Velocity::new::<kilometer_per_second>(1.23);
        assert_eq!(velocity.astro_display(), "1.23 km/s");
        let velocity = Velocity::new::<speed_of_light_in_vacuum>(1.23);
        assert_eq!(velocity.astro_display(), "1.23 c");
    }

    #[test]
    fn test_velocity_negative_display() {
        let velocity = Velocity::new::<meter_per_second>(-1.23);
        assert_eq!(velocity.astro_display(), "-1.23 m/s");
        let velocity = Velocity::new::<kilometer_per_second>(-1.23);
        assert_eq!(velocity.astro_display(), "-1.23 km/s");
        let velocity = Velocity::new::<speed_of_light_in_vacuum>(-1.23);
        assert_eq!(velocity.astro_display(), "-1.23 c");
    }

    #[test]
    fn test_velocity_display_thresholds() {
        let velocity = Velocity::new::<meter_per_second>(0.1);
        assert_eq!(velocity.astro_display(), "0.10 m/s");
        let velocity = Velocity::new::<kilometer_per_second>(0.1);
        assert_eq!(velocity.astro_display(), "0.10 km/s");
        let velocity = Velocity::new::<speed_of_light_in_vacuum>(0.1);
        assert_eq!(velocity.astro_display(), "0.10 c");
    }
}
