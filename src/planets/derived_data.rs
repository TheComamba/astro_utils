use super::{kepler_orbit::orbital_period, planet_data::PlanetData};
use crate::{astro_display::AstroDisplay, stars::star_data::StarData};
use fraction::Fraction;
use simple_si_units::{
    base::{Distance, Luminosity, Mass, Temperature, Time},
    geometry::Angle,
    mechanical::{Acceleration, Density, Velocity},
};
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct DerivedPlanetData {
    density: Density<f64>,
    surface_gravity: Acceleration<f64>,
    escape_velocity: Velocity<f64>,
    orbital_period: Time<f64>,
    orbital_resonance: Option<Fraction>,
    mean_synodic_day: Time<f64>,
    axial_tilt: Angle<f64>,
    black_body_temperature: Temperature<f64>,
}

impl DerivedPlanetData {
    pub fn new(
        data: &PlanetData,
        central_body: StarData,
        previous: Option<&DerivedPlanetData>,
    ) -> Self {
        let central_body_mass = central_body.get_mass().unwrap();
        let central_body_luminosity = central_body.get_luminous_intensity().unwrap();
        let radius = data.get_radius();

        let surface_gravity = surface_gravity(data.get_mass(), radius);

        let escape_velocity = escape_velocity(data.get_mass(), radius);

        let volume = 4. / 3. * PI * radius * radius * radius;
        let density = data.get_mass() / volume;

        let orbital_period = orbital_period(
            data.get_orbital_parameters().semi_major_axis,
            data.get_mass(),
            central_body_mass,
        );

        let orbital_resonance = match previous {
            Some(previous) => orbital_resonance(orbital_period, previous.orbital_period),
            None => None,
        };

        let mean_synodic_day = mean_synodic_day(data.get_sideral_rotation_period(), orbital_period);

        let axial_tilt = axis_tilt(data);

        let black_body_temperature = black_body_temperature(
            central_body_luminosity,
            data.get_orbital_parameters().semi_major_axis,
        );

        Self {
            density,
            surface_gravity,
            escape_velocity,
            orbital_period,
            orbital_resonance,
            mean_synodic_day,
            axial_tilt,
            black_body_temperature,
        }
    }

    pub fn get_density(&self) -> Density<f64> {
        self.density
    }

    pub fn get_surface_gravity(&self) -> Acceleration<f64> {
        self.surface_gravity
    }

    pub fn get_escape_velocity(&self) -> Velocity<f64> {
        self.escape_velocity
    }

    pub fn get_orbital_period(&self) -> Time<f64> {
        self.orbital_period
    }

    pub fn get_orbital_resonance(&self) -> Option<Fraction> {
        self.orbital_resonance
    }

    pub fn get_mean_synodic_day(&self) -> Time<f64> {
        self.mean_synodic_day
    }

    pub fn get_axial_tilt(&self) -> Angle<f64> {
        self.axial_tilt
    }

    pub fn get_black_body_temperature(&self) -> Temperature<f64> {
        self.black_body_temperature
    }
}

fn surface_gravity(mass: Mass<f64>, radius: Distance<f64>) -> Acceleration<f64> {
    todo!()
}

fn escape_velocity(mass: Mass<f64>, radius: Distance<f64>) -> Velocity<f64> {
    todo!()
}

impl AstroDisplay for Fraction {
    fn astro_display(&self) -> String {
        format!("{}", self)
    }
}

const RESONANCE_TOLERANCE: f64 = 0.01;
const RESONANCE_MAX_INT: u8 = 6;

fn orbital_resonance(period1: Time<f64>, period2: Time<f64>) -> Option<Fraction> {
    let large = period1.s.max(period2.s);
    let small = period1.s.min(period2.s);
    let ratio = small / large;
    for denominator in 1..=RESONANCE_MAX_INT {
        let numerator_float = ratio * denominator as f64;
        let numerator_int = numerator_float.round() as u8;
        let error = (numerator_float - numerator_int as f64).abs();
        if error < RESONANCE_TOLERANCE && numerator_int <= denominator {
            return Some(Fraction::new(numerator_int, denominator));
        }
    }
    None
}

fn mean_synodic_day(siderial_day: Time<f64>, orbital_period: Time<f64>) -> Time<f64> {
    1. / (1. / siderial_day - 1. / orbital_period)
}

fn black_body_temperature(
    central_body_luminosity: Luminosity<f64>,
    distance: Distance<f64>,
) -> Temperature<f64> {
    todo!()
}

fn axis_tilt(data: &PlanetData) -> Angle<f64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        real_data::{planets::EARTH, stars::SUN_DATA},
        tests::eq,
    };

    #[test]
    fn surface_gravity_of_earth() {
        let mass = EARTH.to_planet_data().mass;
        let radius = EARTH.to_planet_data().get_radius();
        let gravity = surface_gravity(mass, radius);
        assert!(eq(gravity.to_mps2(), 9.81));
    }

    #[test]
    fn escape_velocity_of_earth() {
        let mass = EARTH.to_planet_data().mass;
        let radius = EARTH.to_planet_data().get_radius();
        let velocity = escape_velocity(mass, radius);
        assert!(eq(velocity.to_kmps(), 11.2));
    }

    #[test]
    fn black_body_temperature_of_earth() {
        let luminosity = SUN_DATA.to_star_data().get_luminous_intensity().unwrap();
        let distance = EARTH
            .to_planet_data()
            .get_orbital_parameters()
            .semi_major_axis;
        let temperature = black_body_temperature(luminosity, distance);
        assert!(eq(temperature.to_celsius(), 255.));
    }

    #[test]
    fn orbital_resonance_is_never_larger_than_1() {
        for i in 1..=RESONANCE_MAX_INT {
            for j in 1..=RESONANCE_MAX_INT {
                let period_1 = Time::from_seconds(i as f64);
                let period_2 = Time::from_seconds(j as f64);
                let resonance = orbital_resonance(period_1, period_2).unwrap();
                assert!(resonance <= Fraction::new(1u64, 1u64));
            }
        }
    }

    #[test]
    fn test_finding_orbital_resonance() {
        for small in 1..=RESONANCE_MAX_INT {
            for large in small..=RESONANCE_MAX_INT {
                let expected = Fraction::new(small, large);
                println!("{}/{} = {}", large, small, expected);
                let small = Time::from_seconds(small as f64);
                let large = Time::from_seconds(large as f64);
                let resonance = orbital_resonance(small, large);
                assert!(resonance.is_some());
                let resonance = resonance.unwrap();
                assert_eq!(resonance, expected);
            }
        }
    }

    #[test]
    fn tetst_not_finding_orbital_resonance() {
        let factor = 1.0 + RESONANCE_TOLERANCE * 2.;
        for small in 1..=RESONANCE_MAX_INT {
            for large in small..=RESONANCE_MAX_INT {
                let small = Time::from_seconds(small as f64);
                let large = Time::from_seconds(large as f64 * factor);
                let ratio = large.s / small.s;
                println!("{:.2}/{:.2} = {:.4}", large.s, small.s, ratio);
                let resonance = orbital_resonance(small, large);
                if let Some(resonance) = resonance {
                    println!("Found resonance: {}", resonance);
                }
                assert!(resonance.is_none());
            }
        }
    }

    #[test]
    fn earth_has_synodic_period_of_1_day() {
        let synodic_day = mean_synodic_day(EARTH.siderial_rotation_period, Time::from_yr(1.));
        assert!(eq(synodic_day.to_days(), 1.));
    }

    #[test]
    fn tidally_locked_planet_has_synodic_period_of_infinity() {
        let synodic_day = mean_synodic_day(Time::from_days(1.), Time::from_days(1.));
        assert!(synodic_day.s.is_infinite() || synodic_day.s.is_nan() || synodic_day.s > 1e20);
    }

    #[test]
    fn axis_tilt_of_earth() {
        let tilt = axis_tilt(&EARTH.to_planet_data());
        assert!(eq(tilt.to_degrees(), 23.44));
    }
}
