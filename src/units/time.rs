use simple_si_units::base::Time;

pub const TIME_ZERO: Time<f64> = Time { s: 0. };
pub(crate) const SECONDS_PER_HOUR: f64 = 60. * 60.;
pub(crate) const SECONDS_PER_DAY: f64 = 24. * SECONDS_PER_HOUR;
pub(crate) const SECONDS_PER_YEAR: f64 = 365.25 * SECONDS_PER_DAY;
pub(crate) const SECONDS_PER_MILLION_YEARS: f64 = 1e6 * SECONDS_PER_YEAR;
pub(crate) const SECONDS_PER_BILLION_YEARS: f64 = 1e9 * SECONDS_PER_YEAR;

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

pub fn display_time_in_units(time: Time<f64>, units: TimeUnit) -> String {
    match units {
        TimeUnit::Seconds => format!("{:2} sec", time.to_seconds()),
        TimeUnit::Minutes => format!("{:2} min", time.to_min()),
        TimeUnit::Hours => format!("{:2} hrs", time.to_hr()),
        TimeUnit::Days => format!("{:2} days", time.to_days()),
        TimeUnit::Years => format!("{:2} yrs", time.to_yr()),
        TimeUnit::ThousandYears => format!("{:2} kyr", time.to_kyr()),
        TimeUnit::MillionYears => format!("{:2} Myrs", time.to_Myr()),
        TimeUnit::BillionYears => format!("{:2} Gyrs", time.to_Gyr()),
    }
}

pub fn display_time(time: Time<f64>) -> String {
    let units = if time.to_Gyr().abs() > 0.099 {
        TimeUnit::BillionYears
    } else if time.to_Myr().abs() > 0.099 {
        TimeUnit::MillionYears
    } else if time.to_kyr().abs() > 0.099 {
        TimeUnit::ThousandYears
    } else if time.to_yr().abs() > 0.099 {
        TimeUnit::Years
    } else if time.to_days().abs() > 0.099 {
        TimeUnit::Days
    } else if time.to_hr().abs() > 0.099 {
        TimeUnit::Hours
    } else if time.to_min().abs() > 0.099 {
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
        let time = Time::from_min(1.);
        assert_eq!(format!("{}", time), "1.00 min");
        let time = Time::from_hr(1.);
        assert_eq!(format!("{}", time), "1.00 hrs");
        let time = Time::from_days(1.);
        assert_eq!(format!("{}", time), "1.00 days");
        let time = Time::from_yr(1.);
        assert_eq!(format!("{}", time), "1.00 yrs");
        let time = Time::from_kyr(1.);
        assert_eq!(format!("{}", time), "1.00 kyr");
        let time = Time::from_Myr(1.);
        assert_eq!(format!("{}", time), "1.00 Myrs");
        let time = Time::from_Gyr(1.);
        assert_eq!(format!("{}", time), "1.00 Gyrs");
    }

    #[test]
    fn test_time_negative_display() {
        let time = Time::from_seconds(-1.);
        assert_eq!(format!("{}", time), "-1.00 sec");
        let time = Time::from_min(-1.);
        assert_eq!(format!("{}", time), "-1.00 min");
        let time = Time::from_hr(-1.);
        assert_eq!(format!("{}", time), "-1.00 hrs");
        let time = Time::from_days(-1.);
        assert_eq!(format!("{}", time), "-1.00 days");
        let time = Time::from_yr(-1.);
        assert_eq!(format!("{}", time), "-1.00 yrs");
        let time = Time::from_kyr(-1.);
        assert_eq!(format!("{}", time), "-1.00 kyr");
        let time = Time::from_Myr(-1.);
        assert_eq!(format!("{}", time), "-1.00 Myrs");
        let time = Time::from_Gyr(-1.);
        assert_eq!(format!("{}", time), "-1.00 Gyrs");
    }
}
