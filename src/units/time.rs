use uom::si::{
    f64::Time,
    time::{day, hour, minute, second, year},
};

use crate::astro_display::AstroDisplay;

use super::DISPLAY_THRESHOLD;

unit! {
    system: uom::si;

    quantity: uom::si::time;
    @kiloyear: 3.1536_E10; "kyr", "kiloyear", "kiloyears";
    @megayear: 3.1536_E13; "Myr", "megayear", "megayears";
    @gigayear: 3.1536_E16; "Gyr", "gigayear", "gigayears";
}

pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Years,
    ThousandYears,
    MillionYears,
    BillionYears,
}

pub(crate) fn display_time_in_units(time: &Time, units: TimeUnit) -> String {
    match units {
        TimeUnit::Seconds => format!("{:.2} sec", time.get::<second>()),
        TimeUnit::Minutes => format!("{:.2} min", time.get::<minute>()),
        TimeUnit::Hours => format!("{:.2} hrs", time.get::<hour>()),
        TimeUnit::Days => format!("{:.2} days", time.get::<day>()),
        TimeUnit::Years => format!("{:.2} yrs", time.get::<year>()),
        TimeUnit::ThousandYears => format!("{:.2} kyr", time.get::<kiloyear>()),
        TimeUnit::MillionYears => format!("{:.2} Myrs", time.get::<megayear>()),
        TimeUnit::BillionYears => format!("{:.2} Gyrs", time.get::<gigayear>()),
    }
}

impl AstroDisplay for Time {
    fn astro_display(&self) -> String {
        let units = if self.get::<gigayear>().abs() > DISPLAY_THRESHOLD {
            TimeUnit::BillionYears
        } else if self.get::<megayear>().abs() > DISPLAY_THRESHOLD {
            TimeUnit::MillionYears
        } else if self.get::<kiloyear>().abs() > DISPLAY_THRESHOLD {
            TimeUnit::ThousandYears
        } else if self.get::<year>().abs() > DISPLAY_THRESHOLD {
            TimeUnit::Years
        } else if self.get::<day>().abs() > DISPLAY_THRESHOLD {
            TimeUnit::Days
        } else if self.get::<hour>().abs() > DISPLAY_THRESHOLD {
            TimeUnit::Hours
        } else if self.get::<minute>().abs() > DISPLAY_THRESHOLD {
            TimeUnit::Minutes
        } else {
            TimeUnit::Seconds
        };
        display_time_in_units(self, units)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::INFINITY;

    use super::*;

    #[test]
    fn test_time_display() {
        let time = Time::new::<second>(1.);
        assert_eq!(time.astro_display(), "1.00 sec");
        let time = Time::new::<minute>(1.);
        assert_eq!(time.astro_display(), "1.00 min");
        let time = Time::new::<hour>(1.);
        assert_eq!(time.astro_display(), "1.00 hrs");
        let time = Time::new::<day>(1.);
        assert_eq!(time.astro_display(), "1.00 days");
        let time = Time::new::<year>(1.);
        assert_eq!(time.astro_display(), "1.00 yrs");
        let time = Time::new::<kiloyear>(1.);
        assert_eq!(time.astro_display(), "1.00 kyr");
        let time = Time::new::<megayear>(1.);
        assert_eq!(time.astro_display(), "1.00 Myrs");
        let time = Time::new::<gigayear>(1.);
        assert_eq!(time.astro_display(), "1.00 Gyrs");
    }

    #[test]
    fn test_time_negative_display() {
        let time = Time::new::<second>(-1.);
        assert_eq!(time.astro_display(), "-1.00 sec");
        let time = Time::new::<minute>(1.);
        assert_eq!(time.astro_display(), "-1.00 min");
        let time = Time::new::<hour>(-1.);
        assert_eq!(time.astro_display(), "-1.00 hrs");
        let time = Time::new::<day>(-1.);
        assert_eq!(time.astro_display(), "-1.00 days");
        let time = Time::new::<year>(-1.);
        assert_eq!(time.astro_display(), "-1.00 yrs");
        let time = Time::new::<kiloyear>(-1.);
        assert_eq!(time.astro_display(), "-1.00 kyr");
        let time = Time::new::<megayear>(-1.);
        assert_eq!(time.astro_display(), "-1.00 Myrs");
        let time = Time::new::<gigayear>(-1.);
        assert_eq!(time.astro_display(), "-1.00 Gyrs");
    }

    #[test]
    fn test_infinite_time_display() {
        let time = Time::new::<second>(INFINITY);
        assert_eq!(time.astro_display(), "inf Gyrs");
    }
}
