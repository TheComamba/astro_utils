use crate::Float;
use serde::{Deserialize, Serialize};

pub(crate) const SECONDS_PER_MINUTE: Float = 60.;
const MINUTES_PER_SECOND: Float = 1. / SECONDS_PER_MINUTE;
pub(crate) const SECONDS_PER_HOUR: Float = 3600.;
const HOURS_PER_SECOND: Float = 1. / SECONDS_PER_HOUR;
pub(crate) const SECONDS_PER_DAY: Float = 86400.;
const DAYS_PER_SECOND: Float = 1. / SECONDS_PER_DAY;
pub(crate) const SECONDS_PER_YEAR: Float = 31_557_600.;
const YEARS_PER_SECOND: Float = 1. / SECONDS_PER_YEAR;
pub(crate) const SECONDS_PER_MILLENIUM: Float = 3_155_760e3;
const KILOYEARS_PER_SECOND: Float = 1. / SECONDS_PER_MILLENIUM;
pub(crate) const SECONDS_PER_MILLION_YEARS: Float = 3_155_760e6;
const MILLION_YEARS_PER_SECOND: Float = 1. / SECONDS_PER_MILLION_YEARS;
pub(crate) const SECONDS_PER_BILLION_YEARS: Float = 3_155_760e9;
const BILLION_YEARS_PER_SECOND: Float = 1. / SECONDS_PER_BILLION_YEARS;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Time {
    pub(crate) seconds: Float,
}

impl Time {
    pub const ZERO: Time = Time { seconds: 0. };

    pub const fn from_seconds(seconds: Float) -> Time {
        Time { seconds }
    }

    pub fn from_minutes(minutes: Float) -> Time {
        Time {
            seconds: minutes * SECONDS_PER_MINUTE,
        }
    }

    pub fn from_hours(hours: Float) -> Time {
        Time {
            seconds: hours * SECONDS_PER_HOUR,
        }
    }

    pub fn from_days(days: Float) -> Time {
        Time {
            seconds: days * SECONDS_PER_DAY,
        }
    }

    pub fn from_years(years: Float) -> Time {
        Time {
            seconds: years * SECONDS_PER_YEAR,
        }
    }

    pub fn from_thousand_years(kiloyears: Float) -> Time {
        Time {
            seconds: kiloyears * SECONDS_PER_MILLENIUM,
        }
    }

    pub fn from_million_years(million_years: Float) -> Time {
        Time {
            seconds: million_years * SECONDS_PER_MILLION_YEARS,
        }
    }

    pub fn from_billion_years(billion_years: Float) -> Time {
        Time {
            seconds: billion_years * SECONDS_PER_BILLION_YEARS,
        }
    }

    pub const fn as_seconds(&self) -> Float {
        self.seconds
    }

    pub fn as_minutes(&self) -> Float {
        self.seconds * MINUTES_PER_SECOND
    }

    pub fn as_hours(&self) -> Float {
        self.seconds * HOURS_PER_SECOND
    }

    pub fn as_days(&self) -> Float {
        self.seconds * DAYS_PER_SECOND
    }

    pub fn as_years(&self) -> Float {
        self.seconds * YEARS_PER_SECOND
    }

    pub fn as_thousand_years(&self) -> Float {
        self.seconds * KILOYEARS_PER_SECOND
    }

    pub fn as_million_years(&self) -> Float {
        self.seconds * MILLION_YEARS_PER_SECOND
    }

    pub fn as_billion_years(&self) -> Float {
        self.seconds * BILLION_YEARS_PER_SECOND
    }

    #[cfg(test)]
    pub(crate) fn eq_within(&self, other: Time, accuracy: Time) -> bool {
        let diff = self.seconds - other.seconds;
        diff.abs() <= accuracy.seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TEST_ACCURACY, TEST_TIME_ACCURACY};

    #[test]
    fn test_seconds() {
        let time = Time::from_seconds(1.);
        assert!((time.as_seconds() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_minutes() {
        let expected = Time::from_seconds(SECONDS_PER_MINUTE);
        let time = Time::from_minutes(1.);
        assert!(time.eq_within(expected, TEST_TIME_ACCURACY));
        assert!((time.as_minutes() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_hours() {
        let expected = Time::from_seconds(SECONDS_PER_HOUR);
        let time = Time::from_hours(1.);
        assert!(time.eq_within(expected, TEST_TIME_ACCURACY));
        assert!((time.as_hours() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_days() {
        let expected = Time::from_seconds(SECONDS_PER_DAY);
        let time = Time::from_days(1.);
        assert!(time.eq_within(expected, TEST_TIME_ACCURACY));
        assert!((time.as_days() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_years() {
        let expected = Time::from_seconds(SECONDS_PER_YEAR);
        let time = Time::from_years(1.);
        assert!(time.eq_within(expected, TEST_TIME_ACCURACY));
        assert!((time.as_years() - 1.).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_add() {
        let time1 = Time::from_seconds(1.);
        let time2 = Time::from_seconds(2.);
        let expected = Time::from_seconds(3.);
        assert!(expected.eq_within(time1 + time2, TEST_TIME_ACCURACY));
    }

    #[test]
    fn test_sub() {
        let time1 = Time::from_seconds(1.);
        let time2 = Time::from_seconds(2.);
        let expected = Time::from_seconds(-1.);
        assert!(expected.eq_within(time1 - time2, TEST_TIME_ACCURACY));
    }

    #[test]
    fn test_div_by_time() {
        let time1 = Time::from_seconds(1.);
        let time2 = Time::from_seconds(2.);
        let expected = 0.5 as Float;
        let actual = time1 / time2;
        assert!((expected - actual).abs() < TEST_ACCURACY);
    }
}
