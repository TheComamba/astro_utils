use astro_utils::{
    stars::{
        gaia_data::{fetch_brightest_stars, fetch_brightest_stars_data},
        random_stars::generate_random_stars,
        star_appearance::StarAppearance,
        star_data::StarData,
    },
    units::illuminance::illuminance_to_apparent_magnitude,
};
use simple_si_units::base::Distance;

#[test]
#[ignore]
fn parsec_generates_data_similar_to_gaia() {
    let max_distance: Distance<f64> = Distance::from_lyr(100.0);

    let parsec_data = generate_random_stars(max_distance).unwrap();
    let parsec_stars = parsec_data
        .iter()
        .map(|s| s.to_star_appearance())
        .collect::<Vec<_>>();
    let gaia_data = fetch_brightest_stars_data().unwrap();
    let gaia_stars = fetch_brightest_stars().unwrap();

    let total_num_is_similar = total_number_is_similar(&parsec_stars, &gaia_stars);
    let mag_3_stars_is_similar =
        number_of_stars_in_apparent_magnitude_range_is_similar(&parsec_stars, &gaia_stars, 3.);
    let mag_4_stars_is_similar =
        number_of_stars_in_apparent_magnitude_range_is_similar(&parsec_stars, &gaia_stars, 4.);
    let mag_5_stars_is_similar =
        number_of_stars_in_apparent_magnitude_range_is_similar(&parsec_stars, &gaia_stars, 5.);
    let mag_6_stars_is_similar =
        number_of_stars_in_apparent_magnitude_range_is_similar(&parsec_stars, &gaia_stars, 6.);
    let temperature_is_similar = mean_temperature_is_similar(&parsec_data, &gaia_data);
    assert!(total_num_is_similar);
    assert!(mag_3_stars_is_similar);
    assert!(mag_4_stars_is_similar);
    assert!(mag_5_stars_is_similar);
    assert!(mag_6_stars_is_similar);
    assert!(temperature_is_similar);
}

fn similar(a: f64, b: f64) -> bool {
    if a < 10.0 || b < 10.0 {
        // Should not be called on small numbers
        return false;
    }
    let max = a.max(b);
    let min = a.min(b);
    let ratio = max / min;
    println!("ratio: {}", ratio);
    ratio < 2.0
}

fn total_number_is_similar(parsec: &[StarAppearance], gaia: &[StarAppearance]) -> bool {
    let parsec = parsec.len() as f64;
    let gaia = gaia.len() as f64;
    println!("\nTotal number of stars");
    println!("parsec: {}, gaia: {}", parsec, gaia);
    similar(parsec, gaia)
}

fn apparent_magnitude_is_within(star: &StarAppearance, mag_min: f64, mag_max: f64) -> bool {
    let app_mag = illuminance_to_apparent_magnitude(star.get_illuminance());
    app_mag < mag_min && app_mag > mag_max
}

fn number_of_stars_in_apparent_magnitude_range_is_similar(
    parsec: &[StarAppearance],
    gaia: &[StarAppearance],
    mag: f64,
) -> bool {
    let mag_min = mag + 0.5;
    let mag_max = mag - 0.5;
    let parsec = parsec
        .iter()
        .filter(|s| apparent_magnitude_is_within(s, mag_min, mag_max))
        .count() as f64;
    let gaia = gaia
        .iter()
        .filter(|s| apparent_magnitude_is_within(s, mag_min, mag_max))
        .count() as f64;
    println!(
        "\nTotal number of stars with apparent magnitude between {} and {}:",
        mag_min, mag_max
    );
    println!("parsec: {}, gaia: {}", parsec, gaia);
    similar(parsec, gaia)
}

fn mean_temperature(data: &[StarData]) -> f64 {
    let temperatures = data
        .iter()
        .map(|s| s.get_temperature())
        .filter_map(|t| *t)
        .map(|t| t.to_K())
        .collect::<Vec<_>>();
    temperatures.iter().sum::<f64>() / temperatures.len() as f64
}

fn mean_temperature_is_similar(parsec: &[StarData], gaia: &[StarData]) -> bool {
    let parsec = mean_temperature(parsec);
    let gaia = mean_temperature(gaia);
    println!("\nMean temperature");
    println!("parsec: {}, gaia: {}", parsec, gaia);
    similar(parsec, gaia)
}
