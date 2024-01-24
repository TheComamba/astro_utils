use super::star_appearance::StarAppearance;
use crate::{
    color::sRGBColor,
    coordinates::spherical::SphericalCoordinates,
    error::AstroUtilError,
    units::{angle::Angle, illuminance::Illuminance, temperature},
    Float,
};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum GaiaCellData {
    String(String),
    Float(Float),
    Null,
}

fn get_string(data: &GaiaCellData) -> Option<String> {
    match data {
        GaiaCellData::String(string) => Some(string.clone()),
        _ => None,
    }
}

fn get_float(data: &GaiaCellData) -> Option<Float> {
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
    fn to_star_appearances(&self) -> Result<Vec<StarAppearance>, AstroUtilError> {
        const DESIGNATION_INDEX: usize = 0;
        const ECL_LON_INDEX: usize = 1;
        const ECL_LAT_INDEX: usize = 2;
        const MAG_INDEX: usize = 3;
        const TEMPERATURE_INDEX: usize = 4;
        let mut stars = Vec::new();
        for row in self.data.iter() {
            let designation =
                get_string(&row[DESIGNATION_INDEX]).ok_or(AstroUtilError::DataNotAvailable)?;
            let ecl_lon = get_float(&row[ECL_LON_INDEX]);
            let ecl_lat = get_float(&row[ECL_LAT_INDEX]);
            let mag = get_float(&row[MAG_INDEX]);
            let temperature = get_float(&row[TEMPERATURE_INDEX]);

            let mag = mag.ok_or(AstroUtilError::DataNotAvailable)?;
            let illuminance = Illuminance::from_apparent_magnitude(mag);
            let ecl_lon = Angle::from_degrees(ecl_lon.ok_or(AstroUtilError::DataNotAvailable)?);
            let ecl_lat = Angle::from_degrees(ecl_lat.ok_or(AstroUtilError::DataNotAvailable)?);
            let direction_in_ecliptic = SphericalCoordinates::new(ecl_lon, ecl_lat).to_direction();

            let temperature =
                temperature.map(|temperature| temperature::Temperature::from_kelvin(temperature));
            let color = match temperature {
                Some(temperature) => sRGBColor::from_temperature(temperature),
                None => sRGBColor::DEFAULT,
            };

            let star = StarAppearance {
                name: designation,
                illuminance,
                color,
                direction_in_ecliptic,
            };
            stars.push(star);
        }
        Ok(stars)
    }
}

fn query_brightest_stars(brightest: Illuminance) -> Result<GaiaResponse, AstroUtilError> {
    let mut url = "https://gea.esac.esa.int/tap-server/tap/sync".to_string();
    url += "?REQUEST=doQuery";
    url += "&LANG=ADQL";
    url += "&FORMAT=json";
    url += "&QUERY=SELECT+designation,ecl_lon,ecl_lat,phot_g_mean_mag,teff_gspphot";
    url += "+FROM+gaiadr3.gaia_source";
    url += "+WHERE+phot_g_mean_mag+<+";
    url += &format!("{:.1}", brightest.as_apparent_magnitude());
    let resp = reqwest::blocking::get(&url)
        .map_err(AstroUtilError::Connection)?
        .text()
        .map_err(AstroUtilError::Connection)?;
    serde_json::from_str(&resp).map_err(AstroUtilError::Json)
}

pub fn star_is_already_known(
    new_star: &StarAppearance,
    known_stars: &Vec<&StarAppearance>,
) -> bool {
    known_stars
        .iter()
        .any(|known_star| known_star.apparently_the_same(new_star))
}

pub fn fetch_brightest_stars() -> Result<Vec<StarAppearance>, AstroUtilError> {
    let brightest = Illuminance::from_apparent_magnitude(6.5);
    let resp = query_brightest_stars(brightest)?;
    let gaia_stars = resp.to_star_appearances()?;
    Ok(gaia_stars)
}

#[cfg(test)]
mod tests {
    use crate::data::stars::BRIGHTEST_STARS;

    use super::*;

    fn find_closest_star(
        gaia_star: &StarAppearance,
        known_stars: &Vec<&StarAppearance>,
    ) -> Option<StarAppearance> {
        let mut closest_star = None;
        let mut closest_distance = Angle::from_degrees(90.);
        for known_star in known_stars.iter() {
            let distance = gaia_star
                .direction_in_ecliptic
                .angle_to(&known_star.direction_in_ecliptic);
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
{"name": "teff_gspphot", "datatype": "float", "xtype": null, "arraysize": null, "description": "Effective temperature from GSP-Phot Aeneas best library using BP\/RP spectra", "unit": "K", "ucd": "phys.temperature.effective", "utype": null}
],
"data":
[
["Gaia DR3 1576683529448755328",158.93417999773797,54.319104622247664,1.731607,null],
["Gaia DR3 6560604777055249536",315.907271103737,-32.914074316251266,1.7732803,null]
]
}"#;
        println!("Input:\n{}", input);
        let deserialized: GaiaResponse = serde_json::from_str(&input).unwrap();
        let serialized = serde_json::to_string(&deserialized).unwrap();
        println!("After roundtrip:\n{}", serialized);
        assert_eq!(input, serialized);
    }

    #[test]
    fn all_bright_stars_are_already_known() {
        let mut known_stars = vec![];
        for star_data in BRIGHTEST_STARS {
            known_stars.push(star_data.to_star_appearance());
        }

        let gaia_response =
            query_brightest_stars(Illuminance::from_apparent_magnitude(2.5)).unwrap();
        let gaia_stars = gaia_response.to_star_appearances().unwrap();

        println!("gaia_stars.len(): {}", gaia_stars.len());
        assert!(gaia_stars.len() > 10);
        let mut failure_count = 0;
        for gaia_star in gaia_stars.iter() {
            let is_known = star_is_already_known(gaia_star, &known_stars.iter().collect());
            if !is_known {
                println!("gaia_star is not known");
                let closest = find_closest_star(gaia_star, &known_stars.iter().collect()).unwrap();
                println!("gaia_star: \n{:?}", gaia_star);
                println!("closest_star: \n{:?}", closest);
                let (gaia_ra, gaia_dec) = gaia_star
                    .direction_in_ecliptic
                    .to_earth_equatorial()
                    .to_spherical()
                    .to_ra_and_dec();
                let (closest_ra, closest_dec) = closest
                    .direction_in_ecliptic
                    .to_earth_equatorial()
                    .to_spherical()
                    .to_ra_and_dec();
                println!("gaia_star position: \n{}, {}", gaia_ra, gaia_dec);
                println!("closest_star position: \n{}, {}", closest_ra, closest_dec);
                println!(
                    "Angle difference: \n{} arcsecs",
                    gaia_star
                        .direction_in_ecliptic
                        .angle_to(&closest.direction_in_ecliptic)
                        .as_arcsecs()
                );
                println!("gaia_star_illuminance: \n{}", gaia_star.illuminance);
                println!("closest_star_illuminance: \n{}", closest.illuminance);
                println!(
                    "Illuminance difference: \n{} lx",
                    (gaia_star.illuminance - closest.illuminance).as_lux()
                );
                failure_count += 1;
            }
        }
        println!("failure_count: {} of {}", failure_count, gaia_stars.len());
        assert!(failure_count == 0);
    }

    #[test]
    fn known_stars_brightness_is_the_same() {
        let mut known_stars = vec![];
        for star_data in BRIGHTEST_STARS {
            known_stars.push(star_data.to_star_appearance());
        }

        let gaia_response =
            query_brightest_stars(Illuminance::from_apparent_magnitude(3.5)).unwrap();
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
        let mut mean_brightness_difference = Illuminance::ZERO;
        for (gaia_star, known_star) in star_pairs.iter() {
            let brightness_difference = known_star.illuminance - gaia_star.illuminance;
            mean_brightness_difference += brightness_difference;
        }
        mean_brightness_difference /= star_pairs.len() as Float;
        let acceptable_difference = 0.1 * Illuminance::from_apparent_magnitude(2.5);
        println!(
            "mean_brightness_difference: {} lx",
            mean_brightness_difference.as_lux()
        );
        println!(
            "acceptable_difference: {} lx",
            acceptable_difference.as_lux()
        );
        println!(
            "ratio: {}",
            mean_brightness_difference / acceptable_difference
        );
        assert!(mean_brightness_difference.as_lux().abs() < acceptable_difference.as_lux());
    }
}
