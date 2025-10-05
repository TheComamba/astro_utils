use astro_units::mass::{earth_mass, gigaton, kiloton, lunar_mass, megaton, solar_mass};
use uom::si::{
    f64::Mass,
    mass::{kilogram, ton},
};

use crate::astro_display::AstroDisplay;

use super::DISPLAY_THRESHOLD;

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

pub fn display_mass_in_units(mass: &Mass, units: MassUnit) -> String {
    match units {
        MassUnit::Kilograms => format!("{:.2} kg", mass.get::<kilogram>()),
        MassUnit::Tonne => format!("{:.2} t", mass.get::<ton>()),
        MassUnit::Kilotonne => format!("{:.2} kt", mass.get::<kiloton>()),
        MassUnit::Megatonne => format!("{:.2} Mt", mass.get::<megaton>()),
        MassUnit::Gigatonne => format!("{:.2} Gt", mass.get::<gigaton>()),
        MassUnit::LunarMasses => format!("{:.2} M☽", mass.get::<lunar_mass>()),
        MassUnit::EarthMasses => format!("{:.2} M🜨", mass.get::<earth_mass>()),
        MassUnit::SolarMasses => format!("{:.2} M☉", mass.get::<solar_mass>()),
    }
}

impl AstroDisplay for Mass {
    fn astro_display(&self) -> String {
        let units = if self.abs().get::<solar_mass>() > DISPLAY_THRESHOLD {
            MassUnit::SolarMasses
        } else if self.abs().get::<earth_mass>() > DISPLAY_THRESHOLD {
            MassUnit::EarthMasses
        } else if self.abs().get::<lunar_mass>() > DISPLAY_THRESHOLD {
            MassUnit::LunarMasses
        } else if self.abs().get::<gigaton>() > DISPLAY_THRESHOLD {
            MassUnit::Gigatonne
        } else if self.abs().get::<megaton>() > DISPLAY_THRESHOLD {
            MassUnit::Megatonne
        } else if self.abs().get::<kiloton>() > DISPLAY_THRESHOLD {
            MassUnit::Kilotonne
        } else if self.abs().get::<ton>() > DISPLAY_THRESHOLD {
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
        let mass = Mass::new::<kilogram>(1.23);
        assert_eq!(mass.astro_display(), "1.23 kg");
        let mass = Mass::new::<ton>(1.23);
        assert_eq!(mass.astro_display(), "1.23 t");
        let mass = Mass::new::<ton>(1.23e3);
        assert_eq!(mass.astro_display(), "1.23 kt");
        let mass = Mass::new::<ton>(1.23e6);
        assert_eq!(mass.astro_display(), "1.23 Mt");
        let mass = Mass::new::<ton>(1.23e9);
        assert_eq!(mass.astro_display(), "1.23 Gt");
        let mass = Mass::new::<lunar_mass>(1.23);
        assert_eq!(mass.astro_display(), "1.23 M☽");
        let mass = Mass::new::<earth_mass>(1.23);
        assert_eq!(mass.astro_display(), "1.23 M🜨");
        let mass = Mass::new::<solar_mass>(1.23);
        assert_eq!(mass.astro_display(), "1.23 M☉");
    }

    #[test]
    fn test_mass_negative_display() {
        let mass = Mass::new::<kilogram>(-1.23);
        assert_eq!(mass.astro_display(), "-1.23 kg");
        let mass = Mass::new::<ton>(-1.23);
        assert_eq!(mass.astro_display(), "-1.23 t");
        let mass = Mass::new::<ton>(-1.23e3);
        assert_eq!(mass.astro_display(), "-1.23 kt");
        let mass = Mass::new::<ton>(-1.23e6);
        assert_eq!(mass.astro_display(), "-1.23 Mt");
        let mass = Mass::new::<ton>(-1.23e9);
        assert_eq!(mass.astro_display(), "-1.23 Gt");
        let mass = Mass::new::<lunar_mass>(-1.23);
        assert_eq!(mass.astro_display(), "-1.23 M☽");
        let mass = Mass::new::<earth_mass>(-1.23);
        assert_eq!(mass.astro_display(), "-1.23 M🜨");
        let mass = Mass::new::<solar_mass>(-1.23);
        assert_eq!(mass.astro_display(), "-1.23 M☉");
    }

    #[test]
    fn test_mass_display_thresholds() {
        let mass = Mass::new::<kilogram>(0.1);
        assert_eq!(mass.astro_display(), "0.10 kg");
        let mass = Mass::new::<ton>(0.1);
        assert_eq!(mass.astro_display(), "0.10 t");
        let mass = Mass::new::<ton>(0.1e3);
        assert_eq!(mass.astro_display(), "0.10 kt");
        let mass = Mass::new::<ton>(0.1e6);
        assert_eq!(mass.astro_display(), "0.10 Mt");
        let mass = Mass::new::<ton>(0.1e9);
        assert_eq!(mass.astro_display(), "0.10 Gt");
        let mass = Mass::new::<lunar_mass>(0.1);
        assert_eq!(mass.astro_display(), "0.10 M☽");
        let mass = Mass::new::<earth_mass>(0.1);
        assert_eq!(mass.astro_display(), "0.10 M🜨");
        let mass = Mass::new::<solar_mass>(0.1);
        assert_eq!(mass.astro_display(), "0.10 M☉");
    }
}
