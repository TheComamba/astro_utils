use astro_coords::earth_equatorial::EarthEquatorial;
use uom::si::angle::degree;
use uom::si::f64::Angle;
use uom::si::f64::Time;
use uom::si::time::hour;

use crate::color::srgb::sRGBColor;
use crate::planets::orbit_parameters::OrbitParameters;
use crate::planets::real_data::RealData;
use crate::units::angle::*;
use crate::units::time::*;

pub fn MERCURY() -> RealData {
    RealData {
        name: "Mercury",
        orbit: OrbitParameters {
            semi_major_axis: Length { m: 57_909_050_000. },
            eccentricity: 0.205_630_69,
            inclination: Angle::new::<degree>(7.00487),
            longitude_of_ascending_node: Angle::new::<degree>(48.33167),
            argument_of_periapsis: Angle::new::<degree>(29.124279),
        },
        geometric_albedo: 0.142,
        bond_albedo: Some(0.088),
        color: sRGBColor::from_sRGB(0.6, 0.58, 0.58),
        radius: Length { m: 2_439_700. },
        mass: Mass { kg: 3.3011e23 },
        siderial_rotation_period: Time::new::<hour>(58.646_2 * 24.),
        axis_tilt: Angle::new::<degree>(0.034),
        rotation_axis: EarthEquatorial::new(
            Angle::new::<degree>(281.01),
            Angle::new::<degree>(61.45),
        ),
    }
}

pub fn VENUS() -> RealData {
    RealData {
        name: "Venus",
        orbit: OrbitParameters {
            semi_major_axis: Length {
                m: 108_208_000_000.,
            },
            eccentricity: 0.006_773_23,
            inclination: Angle::new::<degree>(3.39471),
            longitude_of_ascending_node: Angle::new::<degree>(76.68069),
            argument_of_periapsis: Angle::new::<degree>(54.85229),
        },
        geometric_albedo: 0.689,
        bond_albedo: Some(0.76),
        color: sRGBColor::from_sRGB(0.75, 0.74, 0.71),
        radius: Length { m: 6_051_800. },
        mass: Mass { kg: 4.8675e24 },
        siderial_rotation_period: Time::new::<hour>(-243.022_6 * 24.),
        axis_tilt: Angle::new::<degree>(2.64),
        rotation_axis: EarthEquatorial::new(
            Angle::new::<degree>(272.76),
            Angle::new::<degree>(67.16),
        ),
    }
}

pub fn EARTH() -> RealData {
    RealData {
        name: "Earth",
        orbit: OrbitParameters {
            semi_major_axis: Length {
                m: 149_598_023_000.,
            },
            eccentricity: 0.016_708_6,
            inclination: Angle::new::<degree>(0.00005),
            longitude_of_ascending_node: Angle::new::<degree>(-11.26064),
            argument_of_periapsis: Angle::new::<degree>(114.20783),
        },
        geometric_albedo: 0.367,
        bond_albedo: Some(0.306),
        color: sRGBColor::from_sRGB(0.38, 0.39, 0.48),
        radius: Length { m: 6_371_009. },
        mass: Mass { kg: 5.97e24 },
        siderial_rotation_period: Time::new::<hour>(0.997_269_68 * 24.),
        axis_tilt: Angle::new::<degree>(23.439_281),
        rotation_axis: EarthEquatorial::new(angle_zero(), quarter_circ()),
    }
}

pub fn MARS() -> RealData {
    RealData {
        name: "Mars",
        orbit: OrbitParameters {
            semi_major_axis: Length {
                m: 227_939_200_000.,
            },
            eccentricity: 0.093_394_1,
            inclination: Angle::new::<degree>(1.85061),
            longitude_of_ascending_node: Angle::new::<degree>(49.57854),
            argument_of_periapsis: Angle::new::<degree>(286.4623),
        },
        geometric_albedo: 0.17,
        bond_albedo: Some(0.25),
        color: sRGBColor::from_sRGB(0.59, 0.38, 0.19),
        radius: Length { m: 3_389_500. },
        mass: Mass { kg: 6.4171e23 },
        siderial_rotation_period: Time::new::<hour>(1.025_956_75 * 24.),
        axis_tilt: Angle::new::<degree>(25.19),
        rotation_axis: EarthEquatorial::new(
            Angle::new::<degree>(317.681),
            Angle::new::<degree>(52.88650),
        ),
    }
}

pub fn CERES() -> RealData {
    RealData {
        name: "Ceres",
        orbit: OrbitParameters {
            semi_major_axis: Length {
                m: 413_690_250_000.,
            },
            eccentricity: 0.075_823_9,
            inclination: Angle::new::<degree>(10.593),
            longitude_of_ascending_node: Angle::new::<degree>(80.393),
            argument_of_periapsis: Angle::new::<degree>(73.597),
        },
        geometric_albedo: 0.09,
        bond_albedo: None,
        color: sRGBColor::from_sRGB(1., 1., 1.), //Color unknown, taking grey
        radius: Length { m: 476_200. },
        mass: Mass { kg: 9.393e20 },
        siderial_rotation_period: Time::new::<hour>(9.074_170),
        axis_tilt: Angle::new::<degree>(4.),
        rotation_axis: EarthEquatorial::new(
            Angle::new::<degree>(291.42744),
            Angle::new::<degree>(66.76033),
        ),
    }
}

pub fn JUPITER() -> RealData {
    RealData {
        name: "Jupiter",
        orbit: OrbitParameters {
            semi_major_axis: Length {
                m: 778_547_200_000.,
            },
            eccentricity: 0.048_386_24,
            inclination: Angle::new::<degree>(1.30530),
            longitude_of_ascending_node: Angle::new::<degree>(100.55615),
            argument_of_periapsis: Angle::new::<degree>(273.865),
        },
        geometric_albedo: 0.538,
        bond_albedo: Some(0.503),
        color: sRGBColor::from_sRGB(0.76, 0.7, 0.67),
        radius: Length { m: 69_950_000. },
        mass: Mass { kg: 1.898e27 },
        siderial_rotation_period: Time::new::<hour>(9.925_8),
        axis_tilt: Angle::new::<degree>(3.13),
        rotation_axis: EarthEquatorial::new(
            Angle::new::<degree>(268.057),
            Angle::new::<degree>(64.495),
        ),
    }
}

pub fn SATURN() -> RealData {
    RealData {
        name: "Saturn",
        orbit: OrbitParameters {
            semi_major_axis: Length {
                m: 1_433_449_370_000.,
            },
            eccentricity: 0.054_150_60,
            inclination: Angle::new::<degree>(2.48446),
            longitude_of_ascending_node: Angle::new::<degree>(113.71504),
            argument_of_periapsis: Angle::new::<degree>(339.39153),
        },
        geometric_albedo: 0.499,
        bond_albedo: Some(0.342),
        color: sRGBColor::from_sRGB(0.77, 0.7, 0.56),
        radius: Length { m: 58_232_000. },
        mass: Mass { kg: 5.6834e26 },
        siderial_rotation_period: Time::new::<hour>(10.656),
        axis_tilt: Angle::new::<degree>(26.73),
        rotation_axis: EarthEquatorial::new(
            Angle::new::<degree>(40.589),
            Angle::new::<degree>(83.537),
        ),
    }
}

pub fn URANUS() -> RealData {
    RealData {
        name: "Uranus",
        orbit: OrbitParameters {
            semi_major_axis: Length {
                m: 2_872_463_710_000.,
            },
            eccentricity: 0.047_167_71,
            inclination: Angle::new::<degree>(0.76986),
            longitude_of_ascending_node: Angle::new::<degree>(74.22988),
            argument_of_periapsis: Angle::new::<degree>(96.734),
        },
        geometric_albedo: 0.488,
        bond_albedo: Some(0.300),
        color: sRGBColor::from_sRGB(0.57, 0.75, 0.83),
        radius: Length { m: 25_362_000. },
        mass: Mass { kg: 8.6810e25 },
        siderial_rotation_period: Time::new::<hour>(-17.24),
        axis_tilt: Angle::new::<degree>(82.23),
        rotation_axis: EarthEquatorial::new(
            Angle::new::<degree>(257.311),
            Angle::new::<degree>(-15.175),
        ),
    }
}

pub fn NEPTUNE() -> RealData {
    RealData {
        name: "Neptune",
        orbit: OrbitParameters {
            semi_major_axis: Length {
                m: 4_495_060_000_000.,
            },
            eccentricity: 0.008_585_87,
            inclination: Angle::new::<degree>(1.76917),
            longitude_of_ascending_node: Angle::new::<degree>(131.72169),
            argument_of_periapsis: Angle::new::<degree>(265.64685),
        },
        geometric_albedo: 0.442,
        bond_albedo: Some(0.290),
        color: sRGBColor::from_sRGB(0.56, 0.75, 0.88),
        radius: Length { m: 24_622_000. },
        mass: Mass { kg: 1.0243e26 },
        siderial_rotation_period: Time::new::<hour>(16.11),
        axis_tilt: Angle::new::<degree>(28.32),
        rotation_axis: EarthEquatorial::new(
            Angle::new::<degree>(299.3),
            Angle::new::<degree>(42.950),
        ),
    }
}

pub fn PLUTO() -> RealData {
    RealData {
        name: "Pluto",
        orbit: OrbitParameters {
            semi_major_axis: Length {
                m: 5_906_380_000_000.,
            },
            eccentricity: 0.248_807_66,
            inclination: Angle::new::<degree>(17.14175),
            longitude_of_ascending_node: Angle::new::<degree>(110.30347),
            argument_of_periapsis: Angle::new::<degree>(113.76329),
        },
        geometric_albedo: 0.52,
        bond_albedo: Some(0.72),
        color: sRGBColor::from_sRGB(0.63, 0.48, 0.37),
        radius: Length { m: 1_188_300. },
        mass: Mass { kg: 1.303e22 },
        siderial_rotation_period: Time::new::<hour>(-6.387_230 * 24.),
        axis_tilt: Angle::new::<degree>(119.51),
        rotation_axis: EarthEquatorial::new(
            Angle::new::<degree>(132.99),
            Angle::new::<degree>(-6.16),
        ),
    }
}

pub fn luna() -> RealData {
    RealData {
        name: "Moon",
        orbit: OrbitParameters {
            semi_major_axis: Length { m: 384_399_000. },
            eccentricity: 0.054_9,
            inclination: Angle::new::<degree>(5.145),
            longitude_of_ascending_node: Angle::new::<degree>(125.08),
            argument_of_periapsis: Angle::new::<degree>(318.15),
        },
        geometric_albedo: 0.120,
        bond_albedo: Some(0.110),
        color: sRGBColor::from_sRGB(0.59, 0.53, 0.52),
        radius: Length { m: 1_737_400. },
        mass: Mass { kg: 7.3459e22 },
        siderial_rotation_period: Time::new::<hour>(27.321_661 * 24.),
        axis_tilt: Angle::new::<degree>(6.68),
        rotation_axis: EarthEquatorial::new(
            Angle::new::<degree>(266.86),
            Angle::new::<degree>(65.64),
        ),
    }
}
