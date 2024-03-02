use astro_utils::{
    real_data::stars::all::get_many_stars,
    stars::{
        appearance::StarAppearance,
        data::StarData,
        gaia_data::{fetch_brightest_stars, fetch_brightest_stars_data, star_is_already_known},
        random::random_stars::generate_random_stars,
    },
    units::{
        illuminance::illuminance_to_apparent_magnitude, temperature::TEMPERATURE_ZERO,
        time::TIME_ZERO,
    },
};
use simple_si_units::base::Distance;

#[test]
#[ignore]
fn parsec_generates_data_similar_to_gaia() {
    let max_distance: Distance<f64> = Distance::from_lyr(10_000.);

    let parsec_data: Vec<StarData> = generate_random_stars(max_distance)
        .unwrap()
        .into_iter()
        .filter(|s| s.get_age_at_epoch().unwrap() < s.get_lifetime())
        .collect();
    let parsec_stars = parsec_data
        .iter()
        .map(|s| s.to_star_appearance(TIME_ZERO))
        .collect::<Vec<_>>();
    let mut gaia_data = fetch_brightest_stars_data().unwrap();
    let gaia_stars = fetch_brightest_stars().unwrap();
    let manual_data: Vec<StarData> = get_many_stars().iter().map(|s| s.to_star_data()).collect();
    let manual_stars: Vec<StarAppearance> = manual_data
        .iter()
        .map(|s| s.to_star_appearance(TIME_ZERO))
        .collect();
    let mut expected_data = manual_data.clone();
    expected_data.append(&mut gaia_data); //The few duplicates are ok for average temperature
    let mut expected_stars = manual_stars.clone();
    gaia_stars.into_iter().for_each(|s| {
        if !star_is_already_known(&s, &manual_stars) {
            expected_stars.push(s);
        }
    });

    let total_num_is_similar = total_number_is_similar(&parsec_stars, &expected_stars);
    let parsec_to_gaia_ratio = parsec_stars.len() as f64 / expected_stars.len() as f64;
    let mag_1_stars_is_similar = number_of_stars_in_apparent_magnitude_range_is_similar(
        &parsec_stars,
        &expected_stars,
        1.,
        parsec_to_gaia_ratio,
    );
    let mag_2_stars_is_similar = number_of_stars_in_apparent_magnitude_range_is_similar(
        &parsec_stars,
        &expected_stars,
        2.,
        parsec_to_gaia_ratio,
    );
    let mag_3_stars_is_similar = number_of_stars_in_apparent_magnitude_range_is_similar(
        &parsec_stars,
        &expected_stars,
        3.,
        parsec_to_gaia_ratio,
    );
    let mag_4_stars_is_similar = number_of_stars_in_apparent_magnitude_range_is_similar(
        &parsec_stars,
        &expected_stars,
        4.,
        parsec_to_gaia_ratio,
    );
    let mag_5_stars_is_similar = number_of_stars_in_apparent_magnitude_range_is_similar(
        &parsec_stars,
        &expected_stars,
        5.,
        parsec_to_gaia_ratio,
    );
    let mag_6_stars_is_similar = number_of_stars_in_apparent_magnitude_range_is_similar(
        &parsec_stars,
        &expected_stars,
        6.,
        parsec_to_gaia_ratio,
    );
    let temperature_is_similar = mean_temperature_is_similar(&parsec_data, &expected_data);
    assert!(total_num_is_similar);
    assert!(mag_1_stars_is_similar);
    assert!(mag_2_stars_is_similar);
    assert!(mag_3_stars_is_similar);
    assert!(mag_4_stars_is_similar);
    assert!(mag_5_stars_is_similar);
    assert!(mag_6_stars_is_similar);
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
