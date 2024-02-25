use serde::{Deserialize, Serialize};
use simple_si_units::base::Mass;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum StarFate {
    WhiteDwarf,
    TypeIISupernova,
}

impl StarFate {
    pub(crate) fn new(initial_mass: Mass<f64>) -> Self {
        if initial_mass < Mass::from_solar_mass(8.) {
            StarFate::WhiteDwarf
        } else {
            StarFate::TypeIISupernova
        }
    }
}
