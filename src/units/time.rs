use uom::si::f64::Time;

use crate::astro_display::AstroDisplay;

use super::DISPLAY_THRESHOLD;

pub const TIME_ZERO: Time = Time { s: 0. };
pub const HOUR: Time = Time { s: 60. * 60. };
pub const DAY: Time = Time { s: 24. * HOUR.s };
pub const YEAR: Time = Time { s: 365.25 * DAY.s };
pub(crate) const TEN_MILLENIA: Time = Time {
    s: 10_000. * 365.25 * 24. * 60. * 60.,
};
pub const BILLION_YEARS: Time = Time { s: 1e9 * YEAR.s };

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
        TimeUnit::Seconds => format!("{:.2} sec", time.to_seconds()),
        TimeUnit::Minutes => format!("{:.2} min", time.to_min()),
        TimeUnit::Hours => format!("{:.2} hrs", time.to_hr()),
        TimeUnit::Days => format!("{:.2} days", time.to_days()),
        TimeUnit::Years => format!("{:.2} yrs", time.to_yr()),
        TimeUnit::ThousandYears => format!("{:.2} kyr", time.to_kyr()),
        TimeUnit::MillionYears => format!("{:.2} Myrs", time.to_Myr()),
        TimeUnit::BillionYears => format!("{:.2} Gyrs", time.to_Gyr()),
    }
}

impl AstroDisplay for Time {
    fn astro_display(&self) -> String {
        let units = if self.to_Gyr().abs() > DISPLAY_THRESHOLD {
            TimeUnit::BillionYears
        } else if self.to_Myr().abs() > DISPLAY_THRESHOLD {
            TimeUnit::MillionYears
        } else if self.to_kyr().abs() > DISPLAY_THRESHOLD {
            TimeUnit::ThousandYears
        } else if self.to_yr().abs() > DISPLAY_THRESHOLD {
            TimeUnit::Years
        } else if self.to_days().abs() > DISPLAY_THRESHOLD {
            TimeUnit::Days
        } else if self.to_hr().abs() > DISPLAY_THRESHOLD {
            TimeUnit::Hours
        } else if self.to_min().abs() > DISPLAY_THRESHOLD {
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
        let time = Time::from_seconds(1.);
        assert_eq!(time.astro_display(), "1.00 sec");
        let time = Time::from_min(1.);
        assert_eq!(time.astro_display(), "1.00 min");
        let time = Time::from_hr(1.);
        assert_eq!(time.astro_display(), "1.00 hrs");
        let time = Time::from_days(1.);
        assert_eq!(time.astro_display(), "1.00 days");
        let time = Time::from_yr(1.);
        assert_eq!(time.astro_display(), "1.00 yrs");
        let time = Time::from_kyr(1.);
        assert_eq!(time.astro_display(), "1.00 kyr");
        let time = Time::from_Myr(1.);
        assert_eq!(time.astro_display(), "1.00 Myrs");
        let time = Time::from_Gyr(1.);
        assert_eq!(time.astro_display(), "1.00 Gyrs");
    }

    #[test]
    fn test_time_negative_display() {
        let time = Time::from_seconds(-1.);
        assert_eq!(time.astro_display(), "-1.00 sec");
        let time = Time::from_min(-1.);
        assert_eq!(time.astro_display(), "-1.00 min");
        let time = Time::from_hr(-1.);
        assert_eq!(time.astro_display(), "-1.00 hrs");
        let time = Time::from_days(-1.);
        assert_eq!(time.astro_display(), "-1.00 days");
        let time = Time::from_yr(-1.);
        assert_eq!(time.astro_display(), "-1.00 yrs");
        let time = Time::from_kyr(-1.);
        assert_eq!(time.astro_display(), "-1.00 kyr");
        let time = Time::from_Myr(-1.);
        assert_eq!(time.astro_display(), "-1.00 Myrs");
        let time = Time::from_Gyr(-1.);
        assert_eq!(time.astro_display(), "-1.00 Gyrs");
    }

    #[test]
    fn test_infinite_time_display() {
        let time = Time { s: INFINITY };
        assert_eq!(time.astro_display(), "inf Gyrs");
    }
}
