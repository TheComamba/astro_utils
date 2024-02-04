use super::DISPLAY_THRESHOLD;
use simple_si_units::base::Mass;

pub const MASS_ZERO: Mass<f64> = Mass { kg: 0. };
pub const EARTH_MASS: Mass<f64> = Mass { kg: 5.972e24 };
pub const SOLAR_MASS: Mass<f64> = Mass { kg: 1.988e30 };

pub enum MassUnit {
    Kilograms,
    EarthMasses,
    SolarMasses,
}

pub(crate) fn mass_to_earth_masses(mass: Mass<f64>) -> f64 {
    mass / EARTH_MASS
}

pub(crate) fn mass_to_solar_masses(mass: Mass<f64>) -> f64 {
    mass / SOLAR_MASS
}

pub fn display_mass_in_units(mass: Mass<f64>, units: MassUnit) -> String {
    match units {
        MassUnit::Kilograms => format!("{:.2} kg", mass.to_kilograms()),
        MassUnit::EarthMasses => format!("{:.2} M🜨", mass / EARTH_MASS),
        MassUnit::SolarMasses => format!("{:.2} M☉", mass / SOLAR_MASS),
    }
}

pub fn display_mass(mass: Mass<f64>) -> String {
    let units = if mass_to_solar_masses(mass).abs() > DISPLAY_THRESHOLD {
        MassUnit::SolarMasses
    } else if mass_to_earth_masses(mass).abs() > DISPLAY_THRESHOLD {
        MassUnit::EarthMasses
    } else {
        MassUnit::Kilograms
    };
    display_mass_in_units(mass, units)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_display() {
        let mass = Mass::from_kilograms(1.23);
        assert_eq!(display_mass(mass), "1.23 kg");
        let mass = 1.23 * EARTH_MASS;
        assert_eq!(display_mass(mass), "1.23 M🜨");
        let mass = 1.23 * SOLAR_MASS;
        assert_eq!(display_mass(mass), "1.23 M☉");
    }

    #[test]
    fn test_mass_negative_display() {
        let mass = Mass::from_kilograms(-1.23);
        assert_eq!(display_mass(mass), "-1.23 kg");
        let mass = -1.23 * EARTH_MASS;
        assert_eq!(display_mass(mass), "-1.23 M🜨");
        let mass = -1.23 * SOLAR_MASS;
        assert_eq!(display_mass(mass), "-1.23 M☉");
    }

    #[test]
    fn test_mass_display_thresholds() {
        let mass = Mass::from_kilograms(1.);
        assert_eq!(display_mass(mass), "1.00 kg");
        let mass = 0.1 * EARTH_MASS;
        assert_eq!(display_mass(mass), "0.10 M🜨");
        let mass = 0.1 * SOLAR_MASS;
        assert_eq!(display_mass(mass), "0.10 M☉");
    }
}
