use astro_utils::{
    astro_display::AstroDisplay,
    stars::{
        appearance::StarAppearance, data::StarData,
        gaia::gaia_universe_simulation::fetch_brightest_stars_simulated_data,
        random::random_stars::generate_random_stars,
    },
    units::{
        illuminance::illuminance_to_apparent_magnitude, temperature::TEMPERATURE_ZERO,
        time::TIME_ZERO,
    },
};

#[test]
#[ignore]
fn parsec_generates_data_similar_to_gaia() {
    let max_distance: Length = Length::from_lyr(15_000.);

    let randoms_stars = generate_random_stars(max_distance).unwrap();
    println!(
        "Generated {} random stars within {}.",
        randoms_stars.len(),
        max_distance.astro_display()
    );
    let parsec_data: Vec<StarData> = randoms_stars
        .into_iter()
        .filter(|s| s.get_age_at_epoch().unwrap() < s.get_lifetime())
        .collect();
    let parsec_stars = parsec_data
        .iter()
        .map(|s| s.to_star_appearance(TIME_ZERO))
        .collect::<Vec<_>>();
    let gaia_simulated_data = fetch_brightest_stars_simulated_data()
        .unwrap()
        .into_iter()
        .filter(|s| s.get_temperature_at_epoch() > TEMPERATURE_ZERO)
        .collect::<Vec<_>>();
    let gaia_stars = gaia_simulated_data
        .iter()
        .map(|s| s.to_star_appearance(TIME_ZERO))
        .collect::<Vec<_>>();

    let total_num_is_similar = total_number_is_similar(&parsec_stars, &gaia_stars);
    let parsec_to_gaia_ratio = parsec_stars.len() as f64 / gaia_stars.len() as f64;
    let mag_1_stars_is_similar = number_of_stars_in_apparent_magnitude_range_is_similar(
        &parsec_stars,
        &gaia_stars,
        1.,
        parsec_to_gaia_ratio,
    );
    let mag_2_stars_is_similar = number_of_stars_in_apparent_magnitude_range_is_similar(
        &parsec_stars,
        &gaia_stars,
        2.,
        parsec_to_gaia_ratio,
    );
    let mag_3_stars_is_similar = number_of_stars_in_apparent_magnitude_range_is_similar(
        &parsec_stars,
        &gaia_stars,
        3.,
        parsec_to_gaia_ratio,
    );
    let mag_4_stars_is_similar = number_of_stars_in_apparent_magnitude_range_is_similar(
        &parsec_stars,
        &gaia_stars,
        4.,
        parsec_to_gaia_ratio,
    );
    let mag_5_stars_is_similar = number_of_stars_in_apparent_magnitude_range_is_similar(
        &parsec_stars,
        &gaia_stars,
        5.,
        parsec_to_gaia_ratio,
    );
    let mag_6_stars_is_similar = number_of_stars_in_apparent_magnitude_range_is_similar(
        &parsec_stars,
        &gaia_stars,
        6.,
        parsec_to_gaia_ratio,
    );
    let temperatures_in_range_1_are_similar = number_of_stars_in_temperature_range_is_similar(
        &parsec_data,
        &gaia_simulated_data,
        ThermodynamicTemperature::new::<kelvin>(10.),
        ThermodynamicTemperature::new::<kelvin>(2_000.),
    );
    let temperatures_in_range_2_are_similar = number_of_stars_in_temperature_range_is_similar(
        &parsec_data,
        &gaia_simulated_data,
        ThermodynamicTemperature::new::<kelvin>(2_000.),
        ThermodynamicTemperature::new::<kelvin>(4_000.),
    );
    let temperatures_in_range_3_are_similar = number_of_stars_in_temperature_range_is_similar(
        &parsec_data,
        &gaia_simulated_data,
        ThermodynamicTemperature::new::<kelvin>(4_000.),
        ThermodynamicTemperature::new::<kelvin>(8_000.),
    );
    let temperatures_in_range_4_are_similar = number_of_stars_in_temperature_range_is_similar(
        &parsec_data,
        &gaia_simulated_data,
        ThermodynamicTemperature::new::<kelvin>(8_000.),
        ThermodynamicTemperature::new::<kelvin>(16_000.),
    );
    let temperatures_in_range_5_are_similar = number_of_stars_in_temperature_range_is_similar(
        &parsec_data,
        &gaia_simulated_data,
        ThermodynamicTemperature::new::<kelvin>(16_000.),
        ThermodynamicTemperature::new::<kelvin>(32_000.),
    );
    let temperatures_in_range_6_are_similar = number_of_stars_in_temperature_range_is_similar(
        &parsec_data,
        &gaia_simulated_data,
        ThermodynamicTemperature::new::<kelvin>(32_000.),
        ThermodynamicTemperature::new::<kelvin>(64_000.),
    );
    let temperature_is_similar = mean_temperature_is_similar(&parsec_data, &gaia_simulated_data);
    assert!(total_num_is_similar);
    assert!(mag_1_stars_is_similar);
    assert!(mag_2_stars_is_similar);
    assert!(mag_3_stars_is_similar);
    assert!(mag_4_stars_is_similar);
    assert!(mag_5_stars_is_similar);
    assert!(mag_6_stars_is_similar);
    assert!(temperatures_in_range_1_are_similar);
    assert!(temperatures_in_range_2_are_similar);
    assert!(temperatures_in_range_3_are_similar);
    assert!(temperatures_in_range_4_are_similar);
    assert!(temperatures_in_range_5_are_similar);
    assert!(temperatures_in_range_6_are_similar);
    assert!(temperature_is_similar);
}

fn similar_count(a: usize, b: usize) -> bool {
    if a < 1 || b < 1 {
        // Should not be called on small numbers
        return false;
    }
    let max = a.max(b) as f64;
    let min = a.min(b) as f64;
    let ratio = max / min;
    let accepted = 1. + 20. / min.sqrt();
    println!("ratio: {:2.2} (accepted: {:2.2})", ratio, accepted);
    ratio < accepted
}

fn is_similar(a: f64, b: f64) -> bool {
    let diff = (a - b).abs();
    let max = a.max(b);
    let accepted = 0.5 * max;
    println!("diff: {:2.2e} (accepted: {:2.2e})", diff, accepted);
    diff < accepted
}

fn total_number_is_similar(parsec: &[StarAppearance], gaia: &[StarAppearance]) -> bool {
    let parsec = parsec.len();
    let gaia = gaia.len();
    println!("\nTotal number of stars");
    println!("parsec: {:2.2e}, gaia: {:2.2e}", parsec, gaia);
    similar_count(parsec, gaia)
}

fn apparent_magnitude_is_within(star: &StarAppearance, mag_min: f64, mag_max: f64) -> bool {
    let app_mag = illuminance_to_apparent_magnitude(star.get_illuminance());
    app_mag < mag_min && app_mag > mag_max
}

fn number_of_stars_in_apparent_magnitude_range_is_similar(
    parsec: &[StarAppearance],
    gaia: &[StarAppearance],
    mag: f64,
    parsec_to_gaia_ratio: f64,
) -> bool {
    let mag_min = mag + 0.5;
    let mag_max = mag - 0.5;
    let parsec = parsec
        .iter()
        .filter(|s| apparent_magnitude_is_within(s, mag_min, mag_max))
        .count();
    let parsec = (parsec as f64 / parsec_to_gaia_ratio) as usize;
    let gaia = gaia
        .iter()
        .filter(|s| apparent_magnitude_is_within(s, mag_min, mag_max))
        .count();
    println!(
        "\nTotal number of stars with apparent magnitude between {} and {}:",
        mag_min, mag_max
    );
    println!("parsec (adjusted): {:2.2e}, gaia: {:2.2e}", parsec, gaia);
    similar_count(parsec, gaia)
}

fn number_of_stars_in_temperature_range_is_similar(
    parsec: &[StarData],
    gaia: &[StarData],
    lower: ThermodynamicTemperature,
    upper: ThermodynamicTemperature,
) -> bool {
    let parsec_in_range = parsec
        .iter()
        .filter(|s| {
            let t = s.get_temperature_at_epoch();
            t > lower && t < upper
        })
        .count();
    let gaia_in_range = gaia
        .iter()
        .filter(|s| {
            let t = s.get_temperature_at_epoch();
            t > lower && t < upper
        })
        .count();
    let parsec_fraction = parsec_in_range as f64 / parsec.len() as f64;
    let gaia_fraction = gaia_in_range as f64 / gaia.len() as f64;
    println!(
        "\nTotal number of stars with temperature between {} and {}:",
        lower.astro_display(),
        upper.astro_display()
    );
    println!(
        "parsec: {:2.2e} ({:.0}%), gaia: {:2.2e} ({:2.0}%)",
        parsec_in_range,
        parsec_fraction * 100.,
        gaia_in_range,
        gaia_fraction * 100.
    );
    is_similar(parsec_fraction, gaia_fraction)
}

fn mean_temperature(data: &[StarData]) -> f64 {
    let temperatures = data
        .iter()
        .map(|s| s.get_temperature_at_epoch())
        .filter_map(|t| if t > TEMPERATURE_ZERO { Some(t) } else { None })
        .map(|t| t.to_K())
        .collect::<Vec<_>>();
    temperatures.iter().sum::<f64>() / temperatures.len() as f64
}

fn mean_temperature_is_similar(parsec: &[StarData], gaia: &[StarData]) -> bool {
    let parsec = mean_temperature(parsec);
    let gaia = mean_temperature(gaia);
    println!("\nMean temperature");
    println!("parsec: {:2.2e}, gaia: {:2.2e}", parsec, gaia);
    is_similar(parsec, gaia)
}
