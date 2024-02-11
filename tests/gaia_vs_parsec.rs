use astro_utils::{
    stars::{
        gaia_data::fetch_brightest_stars, random_stars::generate_random_stars,
        star_appearance::StarAppearance,
    },
    units::illuminance::illuminance_to_apparent_magnitude,
};
use simple_si_units::base::Distance;

#[test]
#[ignore]
fn parsec_generates_data_similar_to_gaia() {
    let max_distance: Distance<f64> = Distance::from_lyr(100.0);

    let parsec = generate_random_stars(max_distance)
        .unwrap()
        .iter()
        .map(|s| s.to_star_appearance())
        .collect::<Vec<_>>();
    let gaia = fetch_brightest_stars().unwrap();

    let total_num_is_similar = total_number_is_similar(&parsec, &gaia);
    let mag_3_stars_is_similar =
        number_of_stars_in_apparent_magnitude_range_is_similar(&parsec, &gaia, 3.);
    let mag_4_stars_is_similar =
        number_of_stars_in_apparent_magnitude_range_is_similar(&parsec, &gaia, 4.);
    let mag_5_stars_is_similar =
        number_of_stars_in_apparent_magnitude_range_is_similar(&parsec, &gaia, 5.);
    let mag_6_stars_is_similar =
        number_of_stars_in_apparent_magnitude_range_is_similar(&parsec, &gaia, 6.);
    let temperature_is_similar = mean_temperature_is_similar(&parsec, &gaia);
    assert!(total_num_is_similar);
    assert!(mag_3_stars_is_similar);
    assert!(mag_4_stars_is_similar);
    assert!(mag_5_stars_is_similar);
    assert!(mag_6_stars_is_similar);
    assert!(temperature_is_similar);
}

fn eq_within_order_of_magnitude(a: f64, b: f64) -> bool {
    if a < 10.0 || b < 10.0 {
        // Should not be called on small numbers
        return false;
    }
    let max = a.max(b);
    let min = a.min(b);
    let ratio = max / min;
    println!("ratio: {}", ratio);
    ratio < 10.0
}

fn total_number_is_similar(parsec: &[StarAppearance], gaia: &[StarAppearance]) -> bool {
    let parsec = parsec.len() as f64;
    let gaia = gaia.len() as f64;
    println!("\nTotal number of stars");
    println!("parsec: {}, gaia: {}", parsec, gaia);
    eq_within_order_of_magnitude(parsec, gaia)
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
    eq_within_order_of_magnitude(parsec, gaia)
}

fn mean_temperature_is_similar(parsec: &[StarAppearance], gaia: &[StarAppearance]) -> bool {
    // let parsec = parsec.iter().map(|s| s.get_temperature()).sum::<f64>() / parsec.len() as f64;
    // let gaia = gaia.iter().map(|s| s.get_temperature()).sum::<f64>() / gaia.len() as f64;
    // println!("\nMean temperature");
    // println!("parsec: {}, gaia: {}", parsec, gaia);
    // eq_within_order_of_magnitude(parsec, gaia)
    todo!()
}
