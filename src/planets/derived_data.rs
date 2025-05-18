use std::f64::consts::PI;

use fraction::Fraction;
use uom::si::{
    acceleration::standard_gravity,
    f64::{
        Acceleration, Angle, Length, LuminousIntensity, Mass, MassDensity,
        ThermodynamicTemperature, Time, Velocity,
    },
    length::meter,
    power::watt,
    thermodynamic_temperature::kelvin,
};

use crate::{
    astro_display::AstroDisplay,
    error::AstroUtilError,
    stars::data::StarData,
    units::{length::earth_radii, luminosity::luminous_intensity_to_luminosity, mass::earth_mass},
};

use super::{kepler_orbit::orbital_period, planet_data::PlanetData};

#[derive(Debug, Clone)]
pub struct DerivedPlanetData {
    density: MassDensity,
    surface_gravity: Acceleration,
    escape_velocity: Velocity,
    orbital_period: Time,
    orbital_resonance: Option<Fraction>,
    mean_synodic_day: Time,
    axial_tilt: Angle,
    black_body_temperature: ThermodynamicTemperature,
}

impl DerivedPlanetData {
    pub fn new(
        data: &PlanetData,
        central_body: &StarData,
        previous: Option<&DerivedPlanetData>,
    ) -> Result<Self, AstroUtilError> {
        let central_body_mass = central_body.get_mass_at_epoch().ok_or_else(|| {
            AstroUtilError::DataNotAvailable("Central body is missing mass.".to_string())
        })?;
        let central_body_luminosity = central_body.get_luminous_intensity_at_epoch();
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

        let black_body_temperature = black_body_temperature(central_body_luminosity, data);

        let derived_data = Self {
            density,
            surface_gravity,
            escape_velocity,
            orbital_period,
            orbital_resonance,
            mean_synodic_day,
            axial_tilt,
            black_body_temperature,
        };
        Ok(derived_data)
    }

    pub fn get_density(&self) -> MassDensity {
        self.density
    }

    pub fn get_surface_gravity(&self) -> Acceleration {
        self.surface_gravity
    }

    pub fn get_escape_velocity(&self) -> Velocity {
        self.escape_velocity
    }

    pub fn get_orbital_period(&self) -> Time {
        self.orbital_period
    }

    pub fn get_orbital_resonance(&self) -> Option<Fraction> {
        self.orbital_resonance
    }

    pub fn get_mean_synodic_day(&self) -> Time {
        self.mean_synodic_day
    }

    pub fn get_axial_tilt(&self) -> Angle {
        self.axial_tilt
    }

    pub fn get_black_body_temperature(&self) -> ThermodynamicTemperature {
        self.black_body_temperature
    }
}

fn surface_gravity(mass: Mass, radius: Length) -> Acceleration {
    mass.get::<earth_mass>() / radius.get::<earth_radii>().powi(2)
        * Acceleration::new::<standard_gravity>(1.)
}

fn escape_velocity(mass: Mass, radius: Length) -> Velocity {
    let gravity = surface_gravity(mass, radius);
    (2. * gravity * radius).sqrt()
}

impl AstroDisplay for Fraction {
    fn astro_display(&self) -> String {
        format!("{}", self)
    }
}

const RESONANCE_TOLERANCE: f64 = 0.01;
const RESONANCE_MAX_INT: u8 = 6;

fn orbital_resonance(period1: Time, period2: Time) -> Option<Fraction> {
    let large = period1.max(period2);
    let small = period1.min(period2);
    let ratio: f64 = (small / large).into();
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

fn mean_synodic_day(siderial_day: Time, orbital_period: Time) -> Time {
    1. / (1. / siderial_day - 1. / orbital_period)
}

/*
 * http://www.jeff-hester.com/wp-content/uploads/2015/10/Thermal-Equilibrium-of-Planets.pdf
 */
fn black_body_temperature(
    central_body_luminous_intensity: LuminousIntensity,
    data: &PlanetData,
) -> ThermodynamicTemperature {
    const STEFAN_BOLTZMANN: f64 = 5.67e-8;

    let luminosity = luminous_intensity_to_luminosity(central_body_luminous_intensity);
    let distance = data.get_orbital_parameters().semi_major_axis;
    let albedo = data.get_geometric_albedo();
    let t_to_the_4 = luminosity.get::<watt>() * (1. - albedo)
        / (16. * STEFAN_BOLTZMANN * PI * distance.get::<meter>().powi(2));
    ThermodynamicTemperature::new::<kelvin>(t_to_the_4.powf(1. / 4.))
}

fn axis_tilt(data: &PlanetData) -> Angle {
    data.orbital_parameters
        .normal()
        .angle_to(&data.params.rotation_axis)
}

#[cfg(test)]
mod tests {
    use uom::si::{
        acceleration::meter_per_second_squared,
        angle::degree,
        time::{day, second, year},
        velocity::kilometer_per_second,
    };

    use super::*;
    use crate::{
        real_data::{
            planets::{earth, mercury},
            stars::sun,
        },
        tests::{eq, eq_within},
    };

    const ACCURACY: f64 = 0.025;

    #[test]
    fn surface_gravity_of_earth() {
        let mass = earth().to_planet_data().params.mass;
        let radius = earth().to_planet_data().get_radius();
        let gravity = surface_gravity(mass, radius);
        assert!(eq_within(
            gravity.get::<meter_per_second_squared>(),
            9.81,
            ACCURACY
        ));
    }

    #[test]
    fn surface_gravity_of_mercury() {
        let mass = mercury().to_planet_data().params.mass;
        let radius = mercury().to_planet_data().get_radius();
        let gravity = surface_gravity(mass, radius);
        assert!(eq_within(
            gravity.get::<meter_per_second_squared>(),
            3.7,
            ACCURACY
        ));
    }

    #[test]
    fn escape_velocity_of_earth() {
        let mass = earth().to_planet_data().params.mass;
        let radius = earth().to_planet_data().get_radius();
        let velocity = escape_velocity(mass, radius);
        assert!(eq_within(
            velocity.get::<kilometer_per_second>(),
            11.2,
            10. * ACCURACY
        ));
    }

    #[test]
    fn escape_velocity_of_mercury() {
        let mass = mercury().to_planet_data().params.mass;
        let radius = mercury().to_planet_data().get_radius();
        let velocity = escape_velocity(mass, radius);
        assert!(eq_within(
            velocity.get::<kilometer_per_second>(),
            4.3,
            10. * ACCURACY
        ));
    }

    #[test]
    fn black_body_temperature_of_earth() {
        let luminosity = sun().to_star_data().get_luminous_intensity_at_epoch();
        let temperature = black_body_temperature(luminosity, &earth().to_planet_data());
        assert!(eq_within(temperature.get::<kelvin>(), 255., 20.));
    }

    #[test]
    fn black_body_temperature_of_mercury() {
        let luminosity = sun().to_star_data().get_luminous_intensity_at_epoch();
        let temperature = black_body_temperature(luminosity, &mercury().to_planet_data());
        assert!(eq_within(temperature.get::<kelvin>(), 442., 20.));
    }

    #[test]
    fn orbital_resonance_is_never_larger_than_1() {
        for i in 1..=RESONANCE_MAX_INT {
            for j in 1..=RESONANCE_MAX_INT {
                let period_1 = Time::new::<second>(i as f64);
                let period_2 = Time::new::<second>(j as f64);
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
                let small = Time::new::<second>(small as f64);
                let large = Time::new::<second>(large as f64);
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
                let small = Time::new::<second>(small as f64);
                let large = Time::new::<second>(large as f64 * factor);
                let ratio = large.value / small.value;
                println!("{:.2}/{:.2} = {:.4}", large.value, small.value, ratio);
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
        let synodic_day = mean_synodic_day(earth().siderial_rotation_period, Time::new::<year>(1.));
        assert!(eq(synodic_day.get::<day>(), 1.));
    }

    #[test]
    fn tidally_locked_planet_has_synodic_period_of_infinity() {
        let synodic_day = mean_synodic_day(Time::new::<day>(1.), Time::new::<day>(1.));
        assert!(
            synodic_day.value.is_infinite()
                || synodic_day.value.is_nan()
                || synodic_day.value > 1e20
        );
    }

    #[test]
    fn axis_tilt_of_earth() {
        let tilt = axis_tilt(&earth().to_planet_data());
        assert!(eq_within(tilt.get::<degree>(), 23.44, ACCURACY));
    }

    #[test]
    fn axis_tilt_of_mercury() {
        let tilt = axis_tilt(&mercury().to_planet_data());
        assert!(eq_within(tilt.get::<degree>(), 0.034, ACCURACY));
    }
}
