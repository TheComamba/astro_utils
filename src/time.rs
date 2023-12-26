use crate::Float;
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

pub static SECONDS_PER_MINUTE: Float = 60.0;
pub static SECONDS_PER_DAY: Float = 86400.0;
pub static SECONDS_PER_HOUR: Float = 3600.0;
pub static SECONDS_PER_YEAR: Float = 31557600.0;

#[derive(Debug, Copy, Clone)]
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
        self.seconds / SECONDS_PER_MINUTE
    }

    pub fn as_hours(&self) -> Float {
        self.seconds / SECONDS_PER_HOUR
    }

    pub fn as_days(&self) -> Float {
        self.seconds / SECONDS_PER_DAY
    }

    pub fn as_years(&self) -> Float {
        self.seconds / SECONDS_PER_YEAR
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.seconds.abs() > SECONDS_PER_YEAR {
            write!(f, "{:.2} yrs", self.as_years())
        } else if self.seconds.abs() > SECONDS_PER_DAY {
            write!(f, "{:.2} days", self.as_days())
        } else if self.seconds.abs() > SECONDS_PER_HOUR {
            write!(f, "{:.2} hrs", self.as_hours())
        } else if self.seconds.abs() > SECONDS_PER_MINUTE {
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
