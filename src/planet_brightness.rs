use crate::{
    coordinates::cartesian::CartesianCoordinates,
    units::{length::Length, luminosity::Luminosity},
};

pub fn planet_brightness(
    star_luminosity: Luminosity,
    star_position: &CartesianCoordinates,
    planet_position: &CartesianCoordinates,
    observer_position: &CartesianCoordinates,
    planet_radius: Length,
) -> Luminosity {
    let distance_star_to_planet = (planet_position - star_position).length();
    let distance_planet_to_observer = (planet_position - observer_position).length();
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn apparent_magnitude_of_jupiter_at_opposition() {
        todo!()
    }

    #[test]
    fn apparent_magnitude_of_venus_at_greatest_elongation() {
        todo!()
    }
}
