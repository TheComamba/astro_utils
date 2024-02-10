use crate::astro_display::AstroDisplay;

use super::DISPLAY_THRESHOLD;
use simple_si_units::base::Mass;

pub const MASS_ZERO: Mass<f64> = Mass { kg: 0. };
pub const LUNAR_MASS: Mass<f64> = Mass { kg: 7.346e22 };
pub const EARTH_MASS: Mass<f64> = Mass { kg: 5.972e24 };
pub const SOLAR_MASS: Mass<f64> = Mass { kg: 1.988e30 };

pub enum MassUnit {
    Kilograms,
    Tonne,
    Kilotonne,
    Megatonne,
    Gigatonne,
    LunarMasses,
    EarthMasses,
    SolarMasses,
}

pub fn mass_to_kilotonnes(mass: &Mass<f64>) -> f64 {
    mass.to_tons() * 1e-3
}

pub fn mass_to_megatonnes(mass: &Mass<f64>) -> f64 {
    mass.to_tons() * 1e-6
}

pub fn mass_to_gigatonnes(mass: &Mass<f64>) -> f64 {
    mass.to_tons() * 1e-9
}

pub fn mass_to_lunar_masses(mass: &Mass<f64>) -> f64 {
    mass / &LUNAR_MASS
}

pub fn display_mass_in_units(mass: &Mass<f64>, units: MassUnit) -> String {
    match units {
        MassUnit::Kilograms => format!("{:.2} kg", mass.to_kilograms()),
        MassUnit::Tonne => format!("{:.2} t", mass.to_tons()),
        MassUnit::Kilotonne => format!("{:.2} kt", mass_to_kilotonnes(mass)),
        MassUnit::Megatonne => format!("{:.2} Mt", mass_to_megatonnes(mass)),
        MassUnit::Gigatonne => format!("{:.2} Gt", mass_to_gigatonnes(mass)),
        MassUnit::LunarMasses => format!("{:.2} M☽", mass / &LUNAR_MASS),
        MassUnit::EarthMasses => format!("{:.2} M🜨", mass.to_earth_mass()),
        MassUnit::SolarMasses => format!("{:.2} M☉", mass.to_solar_mass()),
    }
}

impl AstroDisplay for Mass<f64> {
    fn astro_display(&self) -> String {
        let units = if self.to_solar_mass().abs() > DISPLAY_THRESHOLD {
            MassUnit::SolarMasses
        } else if self.to_earth_mass().abs() > DISPLAY_THRESHOLD {
            MassUnit::EarthMasses
        } else if mass_to_lunar_masses(self).abs() > DISPLAY_THRESHOLD {
            MassUnit::LunarMasses
        } else if mass_to_gigatonnes(self).abs() > DISPLAY_THRESHOLD {
            MassUnit::Gigatonne
        } else if mass_to_megatonnes(self).abs() > DISPLAY_THRESHOLD {
            MassUnit::Megatonne
        } else if mass_to_kilotonnes(self).abs() > DISPLAY_THRESHOLD {
            MassUnit::Kilotonne
        } else if self.to_tons().abs() > DISPLAY_THRESHOLD {
            MassUnit::Tonne
        } else {
            MassUnit::Kilograms
        };
        display_mass_in_units(self, units)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_display() {
        let mass = Mass::from_kilograms(1.23);
        assert_eq!(mass.astro_display(), "1.23 kg");
        let mass = Mass::from_tons(1.23);
        assert_eq!(mass.astro_display(), "1.23 t");
        let mass = Mass::from_tons(1.23e3);
        assert_eq!(mass.astro_display(), "1.23 kt");
        let mass = Mass::from_tons(1.23e6);
        assert_eq!(mass.astro_display(), "1.23 Mt");
        let mass = Mass::from_tons(1.23e9);
        assert_eq!(mass.astro_display(), "1.23 Gt");
        let mass = 1.23 as f64 * LUNAR_MASS;
        assert_eq!(mass.astro_display(), "1.23 M☽");
        let mass = 1.23 as f64 * EARTH_MASS;
        assert_eq!(mass.astro_display(), "1.23 M🜨");
        let mass = 1.23 as f64 * SOLAR_MASS;
        assert_eq!(mass.astro_display(), "1.23 M☉");
    }

    #[test]
    fn test_mass_negative_display() {
        let mass = Mass::from_kilograms(-1.23);
        assert_eq!(mass.astro_display(), "-1.23 kg");
        let mass = Mass::from_tons(-1.23);
        assert_eq!(mass.astro_display(), "-1.23 t");
        let mass = Mass::from_tons(-1.23e3);
        assert_eq!(mass.astro_display(), "-1.23 kt");
        let mass = Mass::from_tons(-1.23e6);
        assert_eq!(mass.astro_display(), "-1.23 Mt");
        let mass = Mass::from_tons(-1.23e9);
        assert_eq!(mass.astro_display(), "-1.23 Gt");
        let mass = -1.23 as f64 * LUNAR_MASS;
        assert_eq!(mass.astro_display(), "-1.23 M☽");
        let mass = -1.23 as f64 * EARTH_MASS;
        assert_eq!(mass.astro_display(), "-1.23 M🜨");
        let mass = -1.23 as f64 * SOLAR_MASS;
        assert_eq!(mass.astro_display(), "-1.23 M☉");
    }

    #[test]
    fn test_mass_display_thresholds() {
        let mass = Mass::from_kilograms(0.1);
        assert_eq!(mass.astro_display(), "0.10 kg");
        let mass = Mass::from_tons(0.1);
        assert_eq!(mass.astro_display(), "0.10 t");
        let mass = Mass::from_tons(0.1e3);
        assert_eq!(mass.astro_display(), "0.10 kt");
        let mass = Mass::from_tons(0.1e6);
        assert_eq!(mass.astro_display(), "0.10 Mt");
        let mass = Mass::from_tons(0.1e9);
        assert_eq!(mass.astro_display(), "0.10 Gt");
        let mass = 0.1 as f64 * LUNAR_MASS;
        assert_eq!(mass.astro_display(), "0.10 M☽");
        let mass = 0.1 as f64 * EARTH_MASS;
        assert_eq!(mass.astro_display(), "0.10 M🜨");
        let mass = 0.1 as f64 * SOLAR_MASS;
        assert_eq!(mass.astro_display(), "0.10 M☉");
    }
}
