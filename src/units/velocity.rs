use super::DISPLAY_THRESHOLD;
use crate::astro_display::AstroDisplay;

pub enum VelocityUnit {
    MetresPerSecond,
    KilometresPerSecond,
    SpeedOfLight,
}

pub fn display_velocity_in_units(velocity: &Velocity, units: VelocityUnit) -> String {
    match units {
        VelocityUnit::MetresPerSecond => format!("{:.2} m/s", velocity.to_mps()),
        VelocityUnit::KilometresPerSecond => format!("{:.2} km/s", velocity.to_kmps()),
        VelocityUnit::SpeedOfLight => format!("{:.2} c", velocity.to_c()),
    }
}

impl AstroDisplay for Velocity {
    fn astro_display(&self) -> String {
        let units = if self.to_c().abs() > DISPLAY_THRESHOLD {
            VelocityUnit::SpeedOfLight
        } else if self.to_kmps().abs() > DISPLAY_THRESHOLD {
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
        let velocity = Velocity::from_mps(1.23);
        assert_eq!(velocity.astro_display(), "1.23 m/s");
        let velocity = Velocity::from_kmps(1.23);
        assert_eq!(velocity.astro_display(), "1.23 km/s");
        let velocity = Velocity::from_c(1.23);
        assert_eq!(velocity.astro_display(), "1.23 c");
    }

    #[test]
    fn test_velocity_negative_display() {
        let velocity = Velocity::from_mps(-1.23);
        assert_eq!(velocity.astro_display(), "-1.23 m/s");
        let velocity = Velocity::from_kmps(-1.23);
        assert_eq!(velocity.astro_display(), "-1.23 km/s");
        let velocity = Velocity::from_c(-1.23);
        assert_eq!(velocity.astro_display(), "-1.23 c");
    }

    #[test]
    fn test_velocity_display_thresholds() {
        let velocity = Velocity::from_mps(0.1);
        assert_eq!(velocity.astro_display(), "0.10 m/s");
        let velocity = Velocity::from_kmps(0.1);
        assert_eq!(velocity.astro_display(), "0.10 km/s");
        let velocity = Velocity::from_c(0.1);
        assert_eq!(velocity.astro_display(), "0.10 c");
    }
}
