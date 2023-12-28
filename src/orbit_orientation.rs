use crate::units::angle::Angle;

#[derive(Debug, Clone)]
pub struct OrbitOrientation {
    inclination: Angle, // The angle between the orbital plane and the reference plane
    longitude_of_ascending_node: Angle, // The angle between the reference plane and the ascending node
    argument_of_periapsis: Angle,       // The angle between the ascending node and the periapsis
}
