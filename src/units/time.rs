pub const SECONDS_PER_HOUR: Float = 60. * 60.;
pub const SECONDS_PER_DAY: Float = 24. * SECONDS_PER_HOUR;
pub const SECONDS_PER_YEAR: Float = 365.25 * SECONDS_PER_DAY;
pub const SECONDS_PER_MILLION_YEARS: Float = 1e6 * SECONDS_PER_YEAR;
pub const SECONDS_PER_BILLION_YEARS: Float = 1e9 * SECONDS_PER_YEAR;

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

pub fn display_time_in_units(time: Time<Float>, units: TimeUnit) -> String {
    match units {
        TimeUnit::Seconds => format!("{:2} sec", time.to_seconds()),
        TimeUnit::Minutes => format!("{:2} min", time.to_minutes()),
        TimeUnit::Hours => format!("{:2} hrs", time.to_hours()),
        TimeUnit::Days => format!("{:2} days", time.to_days()),
        TimeUnit::Years => format!("{:2} yrs", time.to_years()),
        TimeUnit::ThousandYears => format!("{:2} kyr", time.to_thousand_years()),
        TimeUnit::MillionYears => format!("{:2} Myrs", time.to_million_years()),
        TimeUnit::BillionYears => format!("{:2} Gyrs", time.to_billion_years()),
    }
}

pub fn display_time(time: Time<Float>) -> String {
    let units = if time.as_billion_years().abs() > 0.099 {
        TimeUnit::BillionYears
    } else if time.as_million_years().abs() > 0.099 {
        TimeUnit::MillionYears
    } else if time.as_thousand_years().abs() > 0.099 {
        TimeUnit::ThousandYears
    } else if time.as_years().abs() > 0.099 {
        TimeUnit::Years
    } else if time.as_days().abs() > 0.099 {
        TimeUnit::Days
    } else if time.as_hours().abs() > 0.099 {
        TimeUnit::Hours
    } else if time.as_minutes().abs() > 0.099 {
        TimeUnit::Minutes
    } else {
        TimeUnit::Seconds
    };
    display_time_in_units(time, units)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_display() {
        let time = Time::from_seconds(1.);
        assert_eq!(format!("{}", time), "1.00 sec");
        let time = Time::from_minutes(1.);
        assert_eq!(format!("{}", time), "1.00 min");
        let time = Time::from_hours(1.);
        assert_eq!(format!("{}", time), "1.00 hrs");
        let time = Time::from_days(1.);
        assert_eq!(format!("{}", time), "1.00 days");
        let time = Time::from_years(1.);
        assert_eq!(format!("{}", time), "1.00 yrs");
        let time = Time::from_thousand_years(1.);
        assert_eq!(format!("{}", time), "1.00 kyr");
        let time = Time::from_million_years(1.);
        assert_eq!(format!("{}", time), "1.00 Myrs");
        let time = Time::from_billion_years(1.);
        assert_eq!(format!("{}", time), "1.00 Gyrs");
    }

    #[test]
    fn test_time_negative_display() {
        let time = Time::from_seconds(-1.);
        assert_eq!(format!("{}", time), "-1.00 sec");
        let time = Time::from_minutes(-1.);
        assert_eq!(format!("{}", time), "-1.00 min");
        let time = Time::from_hours(-1.);
        assert_eq!(format!("{}", time), "-1.00 hrs");
        let time = Time::from_days(-1.);
        assert_eq!(format!("{}", time), "-1.00 days");
        let time = Time::from_years(-1.);
        assert_eq!(format!("{}", time), "-1.00 yrs");
        let time = Time::from_thousand_years(-1.);
        assert_eq!(format!("{}", time), "-1.00 kyr");
        let time = Time::from_million_years(-1.);
        assert_eq!(format!("{}", time), "-1.00 Myrs");
        let time = Time::from_billion_years(-1.);
        assert_eq!(format!("{}", time), "-1.00 Gyrs");
    }
}
