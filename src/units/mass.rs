pub const KILOGRAMS_PER_SOLAR_MASS: f64 = 1.988e30;
pub const KG_PER_SOLAR_MASSES: f64 = 1.988e30;

pub enum MassUnit {
    Kilograms,
    EarthMasses,
    SolarMasses,
}

pub fn display_mass_in_units(mass: Mass<f64>, units: MassUnit) -> String {
    match units {
        MassUnit::Kilograms => format!("{:2} kg", mass.to_kilograms()),
        MassUnit::EarthMasses => format!("{:2} M🜨", mass.to_earth_masses()),
        MassUnit::SolarMasses => format!("{:2} M☉", mass.to_solar_masses()),
    }
}

pub fn display_mass(mass: Mass<f64>) -> String {
    let units = if mass.to_solar_masses().abs() > 0.099 {
        MassUnit::SolarMasses
    } else if mass.to_earth_masses().abs() > 0.099 {
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
        assert_eq!(mass.to_string(), "1.23 kg");
        let mass = Mass::from_earth_masses(1.23);
        assert_eq!(mass.to_string(), "1.23 M🜨");
        let mass = Mass::from_solar_masses(1.23);
        assert_eq!(mass.to_string(), "1.23 M☉");
    }

    #[test]
    fn test_mass_negative_display() {
        let mass = Mass::from_kilograms(-1.23);
        assert_eq!(mass.to_string(), "-1.23 kg");
        let mass = Mass::from_earth_masses(-1.23);
        assert_eq!(mass.to_string(), "-1.23 M🜨");
        let mass = Mass::from_solar_masses(-1.23);
        assert_eq!(mass.to_string(), "-1.23 M☉");
    }

    #[test]
    fn test_mass_display_thresholds() {
        let mass = Mass::from_kilograms(1.);
        assert_eq!(mass.to_string(), "1.00 kg");
        let mass = Mass::from_earth_masses(1e-5);
        assert_eq!(mass.to_string(), "0.00001 M🜨");
        let mass = Mass::from_earth_masses(0.01);
        assert_eq!(mass.to_string(), "0.01 M🜨");
        let mass = Mass::from_solar_masses(0.1);
        assert_eq!(mass.to_string(), "0.10 M☉");
    }
}
