use super::{star_appearance::StarAppearance, star_data::StarData};
use crate::{
    color::sRGBColor,
    coordinates::{ecliptic::EclipticCoordinates, spherical::SphericalCoordinates},
    error::AstroUtilError,
    units::illuminance::{apparent_magnitude_to_illuminance, illuminance_to_apparent_magnitude},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use simple_si_units::{base::Temperature, electromagnetic::Illuminance, geometry::Angle};

#[derive(Serialize, Deserialize)]
struct GaiaMetadataLine {
    name: String,
    datatype: String,
    xtype: Option<String>,
    arraysize: Option<String>,
    description: String,
    unit: Option<String>,
    ucd: String,
    utype: Option<String>,
}

struct ParsedGaiaCellData {
    pub designation: String,
    pub pos: EclipticCoordinates,
    pub mag: f64,
    pub temperature: Option<Temperature<f64>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum GaiaCellData {
    String(String),
    Float(f64),
    Null,
}

fn get_string(data: &GaiaCellData) -> Option<String> {
    match data {
        GaiaCellData::String(string) => Some(string.clone()),
        _ => None,
    }
}

fn get_float(data: &GaiaCellData) -> Option<f64> {
    match data {
        GaiaCellData::Float(float) => Some(*float),
        _ => None,
    }
}

#[derive(Serialize, Deserialize)]
struct GaiaResponse {
    metadata: Vec<GaiaMetadataLine>,
    data: Vec<Vec<GaiaCellData>>,
}

impl GaiaResponse {
    fn get_parsed_data(row: &[GaiaCellData]) -> Result<ParsedGaiaCellData, AstroUtilError> {
        const DESIGNATION_INDEX: usize = 0;
        const ECL_LON_INDEX: usize = 1;
        const ECL_LAT_INDEX: usize = 2;
        const MAG_INDEX: usize = 3;
        const TEMPERATURE_INDEX: usize = 4;

        let designation =
            get_string(&row[DESIGNATION_INDEX]).ok_or(AstroUtilError::DataNotAvailable)?;
        let ecl_lon = get_float(&row[ECL_LON_INDEX]);
        let ecl_lat = get_float(&row[ECL_LAT_INDEX]);
        let temperature = get_float(&row[TEMPERATURE_INDEX]);

        let ecl_lon = Angle::from_degrees(ecl_lon.ok_or(AstroUtilError::DataNotAvailable)?);
        let ecl_lat = Angle::from_degrees(ecl_lat.ok_or(AstroUtilError::DataNotAvailable)?);
        let pos = SphericalCoordinates::new(ecl_lon, ecl_lat).to_ecliptic();
        let mag = get_float(&row[MAG_INDEX]);
        let mag = mag.ok_or(AstroUtilError::DataNotAvailable)?;
        let temperature = temperature.map(Temperature::from_K);
        Ok(ParsedGaiaCellData {
            designation,
            pos,
            mag,
            temperature,
        })
    }

    fn to_star_data(&self) -> Result<Vec<StarData>, AstroUtilError> {
        let stars = self
            .data
            .par_iter()
            .map(|row| {
                let parsed_data = Self::get_parsed_data(row)?;
                let star = StarData {
                    name: parsed_data.designation,
                    mass: None,
                    radius: None,
                    luminous_intensity: None,
                    temperature: parsed_data.temperature,
                    age: None,
                    distance: None,
                    pos: parsed_data.pos,
                    constellation: None,
                };
                Ok(star)
            })
            .collect::<Result<Vec<StarData>, AstroUtilError>>();
        stars
    }

    fn to_star_appearances(&self) -> Result<Vec<StarAppearance>, AstroUtilError> {
        let stars = self
            .data
            .par_iter()
            .map(|row| {
                let parsed_data = Self::get_parsed_data(row)?;

                let illuminance = apparent_magnitude_to_illuminance(parsed_data.mag);
                let color = match parsed_data.temperature {
                    Some(temperature) => sRGBColor::from_temperature(temperature),
                    None => sRGBColor::DEFAULT,
                };

                let star = StarAppearance {
                    name: parsed_data.designation,
                    illuminance,
                    color,
                    pos: parsed_data.pos,
                };
                Ok(star)
            })
            .collect::<Result<Vec<StarAppearance>, AstroUtilError>>();
        stars
    }
}

fn query_brightest_stars(brightest: Illuminance<f64>) -> Result<GaiaResponse, AstroUtilError> {
    let mut url = "https://gea.esac.esa.int/tap-server/tap/sync".to_string();
    url += "?REQUEST=doQuery";
    url += "&LANG=ADQL";
    url += "&FORMAT=json";
    url += "&QUERY=SELECT+designation,ecl_lon,ecl_lat,phot_g_mean_mag,teff_gspphot";
    url += "+FROM+gaiadr3.gaia_source";
    url += "+WHERE+phot_g_mean_mag+<+";
    url += &format!("{:.1}", illuminance_to_apparent_magnitude(&brightest));
    let resp = reqwest::blocking::get(&url)
        .map_err(AstroUtilError::Connection)?
        .text()
        .map_err(AstroUtilError::Connection)?;
    serde_json::from_str(&resp).map_err(AstroUtilError::Json)
}

pub fn star_is_already_known(new_star: &StarAppearance, known_stars: &[&StarAppearance]) -> bool {
    known_stars
        .iter()
        .any(|known_star| known_star.apparently_the_same(new_star))
}

pub fn fetch_brightest_stars() -> Result<Vec<StarAppearance>, AstroUtilError> {
    let brightest = apparent_magnitude_to_illuminance(6.5);
    let resp = query_brightest_stars(brightest)?;
    let gaia_stars = resp.to_star_appearances()?;
    Ok(gaia_stars)
}

pub fn fetch_brightest_stars_data() -> Result<Vec<StarData>, AstroUtilError> {
    let brightest = apparent_magnitude_to_illuminance(6.5);
    let resp = query_brightest_stars(brightest)?;
    let gaia_stars = resp.to_star_data()?;
    Ok(gaia_stars)
}

#[cfg(test)]
mod tests {
    use crate::{
        astro_display::AstroDisplay,
        real_data::stars::BRIGHTEST_STARS,
        units::{angle::angle_to_arcsecs, illuminance::IRRADIANCE_ZERO},
    };

    use super::*;

    fn find_closest_star(
        gaia_star: &StarAppearance,
        known_stars: &[&StarAppearance],
    ) -> Option<StarAppearance> {
        let mut closest_star = None;
        let mut closest_distance = Angle::from_degrees(90.);
        for known_star in known_stars.iter() {
            let distance = gaia_star
                .pos
                .get_spherical()
                .to_direction()
                .angle_to(&known_star.pos.get_spherical().to_direction());
            if distance < closest_distance {
                closest_star = Some(known_star);
                closest_distance = distance;
            }
        }
        closest_star.cloned().cloned()
    }

    #[test]
    fn gaia_serialization_roundtrip() {
        let input = r#"
{"metadata":
[
{"name": "designation", "datatype": "char", "xtype": null, "arraysize": "*", "description": "Unique source designation (unique across all Data Releases)", "unit": null, "ucd": "meta.id;meta.main", "utype": null},
{"name": "ecl_lon", "datatype": "double", "xtype": null, "arraysize": null, "description": "Ecliptic longitude", "unit": "deg", "ucd": "pos.ecliptic.lon", "utype": "stc:AstroCoords.Position2D.Value2.C1"},
{"name": "ecl_lat", "datatype": "double", "xtype": null, "arraysize": null, "description": "Ecliptic latitude", "unit": "deg", "ucd": "pos.ecliptic.lat", "utype": "stc:AstroCoords.Position2D.Value2.C2"},
{"name": "phot_g_mean_mag", "datatype": "float", "xtype": null, "arraysize": null, "description": "G-band mean magnitude", "unit": "mag", "ucd": "phot.mag;em.opt", "utype": null},
{"name": "teff_gspphot", "datatype": "float", "xtype": null, "arraysize": null, "description": "Effective temperature from GSP-Phot Aeneas best library using BP/RP spectra", "unit": "K", "ucd": "phys.temperature.effective", "utype": null}
],
"data":
[
["Gaia DR3 1576683529448755328",158.93417,54.319103,1.731607,null],
["Gaia DR3 6560604777055249536",315.90726,-32.914074,1.7732803,null]
]
}"#;
        println!("Input:\n{}", input);
        let deserialized: GaiaResponse = serde_json::from_str(&input).unwrap();
        let serialized = serde_json::to_string(&deserialized).unwrap();
        println!("After roundtrip:\n{}", serialized);
        let input = input
            .replace("\n", "")
            .replace(": ", ":")
            .replace(", ", ",");
        assert_eq!(input, serialized);
    }

    #[test]
    fn all_bright_gaia_stars_are_already_known() {
        // Gaia finds R Doradus to be much brighter than all other literature.
        const PROBLEMATIC_STAR: &str = "Gaia DR3 4677205714465503104";

        let mut known_stars = vec![];
        for star_data in BRIGHTEST_STARS {
            known_stars.push(star_data.to_star_appearance());
        }
        let known_stars_ref: Vec<&StarAppearance> = known_stars.iter().collect();

        let gaia_response = query_brightest_stars(apparent_magnitude_to_illuminance(2.5)).unwrap();
        let gaia_stars = gaia_response.to_star_appearances().unwrap();

        println!("known_stars.len(): {}", known_stars.len());
        assert!(known_stars.len() > 30);
        println!("gaia_stars.len(): {}", gaia_stars.len());
        assert!(gaia_stars.len() > 30);
        let mut failure_count = 0;
        for gaia_star in gaia_stars.iter() {
            if gaia_star.name == PROBLEMATIC_STAR {
                continue;
            }
            let is_known = star_is_already_known(gaia_star, &known_stars_ref);
            if !is_known {
                println!("\ngaia_star is not known");
                let closest = find_closest_star(gaia_star, &known_stars_ref).unwrap();
                println!("gaia_star: {:?}", gaia_star.name);
                println!("closest_star: {:?}", closest.name);
                let (gaia_ra, gaia_dec) = gaia_star.pos.get_spherical().to_ra_and_dec();
                let (closest_ra, closest_dec) = closest.pos.get_spherical().to_ra_and_dec();
                let angle_difference = gaia_star
                    .pos
                    .get_spherical()
                    .to_direction()
                    .angle_to(&closest.pos.get_spherical().to_direction());
                if angle_difference > Angle::from_degrees(0.03) {
                    println!(
                        "gaia_star position: {}, {}",
                        gaia_ra,
                        gaia_dec.astro_display()
                    );
                    println!(
                        "closest_star position: {}, {}",
                        closest_ra,
                        closest_dec.astro_display()
                    );
                    println!(
                        "Angle difference: {} arcsecs",
                        angle_to_arcsecs(&angle_difference)
                    );
                } else {
                    println!("gaia_star_illuminance: {}", gaia_star.illuminance);
                    println!("closest_star_illuminance: {}", closest.illuminance);
                    println!(
                        "Illuminance difference: {} lx",
                        (gaia_star.illuminance - closest.illuminance).to_lux()
                    );
                }
                failure_count += 1;
            }
        }
        println!("failure_count: {} of {}", failure_count, gaia_stars.len());
        assert!(failure_count == 0);
    }

    #[test]
    fn all_not_too_bright_stars_are_in_gaia() {
        const PROBLEMATIC_STARS: [&str; 11] = [
            "R Doradus", // Gaia finds R Doradus to be much brighter than all other literature.
            "Scheat",
            "Suhail",
            "Eltanin",
            "Dschubba",
            "Epsilon Centauri",
            "Menkar",
            "Ghurab",
            "Zeta Centauri",
            "Eta Centauri", // Only in Gaia DR2
            "Enif",         // Only in Gaia DR2
        ];

        const BRIGHTNESS_THRESHOLD: f64 = 2.2;

        let mut known_stars = vec![];
        for star_data in BRIGHTEST_STARS {
            if star_data.apparent_magnitude > BRIGHTNESS_THRESHOLD
                && !PROBLEMATIC_STARS.contains(&star_data.common_name)
                && !PROBLEMATIC_STARS.contains(&star_data.astronomical_name)
            {
                known_stars.push(star_data.to_star_appearance());
            }
        }

        let gaia_response = query_brightest_stars(apparent_magnitude_to_illuminance(4.)).unwrap();
        let gaia_stars = gaia_response.to_star_appearances().unwrap();
        let gaia_stars_ref: Vec<&StarAppearance> = gaia_stars.iter().collect();

        println!("known_stars.len(): {}", known_stars.len());
        assert!(known_stars.len() > 30);
        println!("gaia_stars.len(): {}", gaia_stars.len());
        assert!(gaia_stars.len() > 30);

        let brightest_gaia_star = gaia_stars
            .iter()
            .min_by_key(|star| (star.illuminance.to_lux() * 1e5) as u32)
            .unwrap();
        println!(
            "Brightest gaia star illuminance: {} mag",
            illuminance_to_apparent_magnitude(&brightest_gaia_star.illuminance)
        );
        assert!(
            illuminance_to_apparent_magnitude(&brightest_gaia_star.illuminance)
                < BRIGHTNESS_THRESHOLD
        );

        let mut failure_count = 0;
        for known_star in known_stars.iter() {
            let is_known = star_is_already_known(known_star, &gaia_stars_ref);
            if !is_known {
                println!("\nknown_star is not in gaia:\n{}", known_star.name);
                println!(
                    "known_star_illuminance: {} mag",
                    illuminance_to_apparent_magnitude(&known_star.illuminance)
                );
                let closest_gaia_star = find_closest_star(known_star, &gaia_stars_ref).unwrap();
                println!("closest_gaia_star: {:?}", closest_gaia_star);
                failure_count += 1;
            }
        }
        println!("failure_count: {} of {}", failure_count, known_stars.len());
        assert!(failure_count == 0);
    }

    #[test]
    fn known_stars_brightness_is_the_same() {
        let mut known_stars = vec![];
        for star_data in BRIGHTEST_STARS {
            known_stars.push(star_data.to_star_appearance());
        }

        let gaia_response = query_brightest_stars(apparent_magnitude_to_illuminance(3.5)).unwrap();
        let gaia_stars = gaia_response.to_star_appearances().unwrap();
        let mut star_pairs = vec![];
        for gaia_star in gaia_stars.iter() {
            for known_star in known_stars.iter() {
                if gaia_star.apparently_the_same(known_star) {
                    star_pairs.push((gaia_star, known_star));
                }
            }
        }
        println!("star_pairs.len(): {}", star_pairs.len());
        assert!(star_pairs.len() > 15);
        let mut mean_brightness_difference = IRRADIANCE_ZERO;
        for (gaia_star, known_star) in star_pairs.iter() {
            let brightness_difference = known_star.illuminance - gaia_star.illuminance;
            mean_brightness_difference += brightness_difference;
        }
        mean_brightness_difference /= star_pairs.len() as f64;
        let acceptable_difference = apparent_magnitude_to_illuminance(4.);
        println!(
            "mean_brightness_difference: \n{} lx",
            mean_brightness_difference.to_lux()
        );
        println!(
            "acceptable_difference: \n{} lx",
            acceptable_difference.to_lux()
        );
        println!(
            "ratio: {}",
            mean_brightness_difference / acceptable_difference
        );
        assert!(mean_brightness_difference.to_lux().abs() < acceptable_difference.to_lux());
    }
}
