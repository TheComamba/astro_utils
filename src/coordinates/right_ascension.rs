use simple_si_units::geometry::Angle;
use std::fmt::Display;

pub struct RightAscension {
    pub(super) hours: i8,
    pub(super) minutes: i8,
    pub(super) seconds: i8,
}

impl RightAscension {
    pub const fn new(hours: i8, minutes: i8, seconds: i8) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }

    pub fn to_angle(&self) -> Angle<f64> {
        let hours = self.hours as f64;
        let minutes = self.minutes as f64;
        let seconds = self.seconds as f64;

        Angle::from_degrees((hours + minutes / 60. + seconds / 3600.) * 15.)
    }
}

impl Display for RightAscension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}h{:02}m{:02}s",
            self.hours, self.minutes, self.seconds
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::eq;

    #[test]
    fn one_second() {
        let dec = RightAscension::new(0, 0, 1);
        let expected = Angle::from_second_angle(1.);
        println!("{}", dec.to_angle().to_arcsecs());
        println!("{}", expected.to_arcsecs());
        assert!(eq(dec.to_angle(), expected));
    }
}
