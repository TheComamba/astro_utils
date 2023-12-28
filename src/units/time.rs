use crate::Float;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

static SECONDS_PER_MINUTE: Float = 60.0;
static MINUTES_PER_SECOND: Float = 1.0 / SECONDS_PER_MINUTE;
static SECONDS_PER_DAY: Float = 86400.0;
static DAYS_PER_SECOND: Float = 1.0 / SECONDS_PER_DAY;
static SECONDS_PER_HOUR: Float = 3600.0;
static HOURS_PER_SECOND: Float = 1.0 / SECONDS_PER_HOUR;
static SECONDS_PER_YEAR: Float = 31557600.0;
static YEARS_PER_SECOND: Float = 1.0 / SECONDS_PER_YEAR;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Time {
    seconds: Float,
}

impl Time {
    pub fn from_seconds(seconds: Float) -> Time {
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

    pub fn as_seconds(&self) -> Float {
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

    pub fn eq_within(&self, other: Time, relative_accuracy: Float) -> bool {
        let max = if self.seconds > other.seconds {
            self.seconds.abs()
        } else {
            other.seconds.abs()
        };
        (self.seconds - other.seconds).abs() <= relative_accuracy * max
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.seconds.abs() > 0.99 * SECONDS_PER_YEAR {
            write!(f, "{:.2} yrs", self.as_years())
        } else if self.seconds.abs() > 0.99 * SECONDS_PER_DAY {
            write!(f, "{:.2} days", self.as_days())
        } else if self.seconds.abs() > 0.99 * SECONDS_PER_HOUR {
            write!(f, "{:.2} hrs", self.as_hours())
        } else if self.seconds.abs() > 0.99 * SECONDS_PER_MINUTE {
            write!(f, "{:.2} min", self.as_minutes())
        } else {
            write!(f, "{:.2} sec", self.as_seconds())
        }
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        Time {
            seconds: self.seconds + other.seconds,
        }
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        Time {
            seconds: self.seconds - other.seconds,
        }
    }
}

impl Mul<Float> for Time {
    type Output = Time;

    fn mul(self, f: Float) -> Time {
        Time {
            seconds: self.seconds * f,
        }
    }
}

impl Mul<Time> for Float {
    type Output = Time;

    fn mul(self, time: Time) -> Time {
        Time {
            seconds: time.seconds * self,
        }
    }
}

impl Div<Float> for Time {
    type Output = Time;

    fn div(self, f: Float) -> Time {
        Time {
            seconds: self.seconds / f,
        }
    }
}

impl Neg for Time {
    type Output = Time;

    fn neg(self) -> Time {
        Time {
            seconds: -self.seconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_ACCURACY: Float = 1e-5;

    #[test]
    fn test_seconds() {
        let time = Time::from_seconds(1.0);
        assert!((time.as_seconds() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_minutes() {
        let expected = Time::from_seconds(SECONDS_PER_MINUTE);
        let time = Time::from_minutes(1.0);
        assert!(time.eq_within(expected, TEST_ACCURACY));
        assert!((time.as_minutes() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_hours() {
        let expected = Time::from_seconds(SECONDS_PER_HOUR);
        let time = Time::from_hours(1.0);
        assert!(time.eq_within(expected, TEST_ACCURACY));
        assert!((time.as_hours() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_days() {
        let expected = Time::from_seconds(SECONDS_PER_DAY);
        let time = Time::from_days(1.0);
        assert!(time.eq_within(expected, TEST_ACCURACY));
        assert!((time.as_days() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_years() {
        let expected = Time::from_seconds(SECONDS_PER_YEAR);
        let time = Time::from_years(1.0);
        assert!(time.eq_within(expected, TEST_ACCURACY));
        assert!((time.as_years() - 1.0).abs() < TEST_ACCURACY);
    }

    #[test]
    fn test_add() {
        let time1 = Time::from_seconds(1.0);
        let time2 = Time::from_seconds(2.0);
        let expected = Time::from_seconds(3.0);
        assert!(expected.eq_within(time1 + time2, TEST_ACCURACY));
    }

    #[test]
    fn test_sub() {
        let time1 = Time::from_seconds(1.0);
        let time2 = Time::from_seconds(2.0);
        let expected = Time::from_seconds(-1.0);
        assert!(expected.eq_within(time1 - time2, TEST_ACCURACY));
    }

    #[test]
    fn test_display() {
        let time = Time::from_seconds(1.0);
        assert_eq!(format!("{}", time), "1.00 sec");
        let time = Time::from_minutes(1.0);
        assert_eq!(format!("{}", time), "1.00 min");
        let time = Time::from_hours(1.0);
        assert_eq!(format!("{}", time), "1.00 hrs");
        let time = Time::from_days(1.0);
        assert_eq!(format!("{}", time), "1.00 days");
        let time = Time::from_years(1.0);
        assert_eq!(format!("{}", time), "1.00 yrs");
    }
}
