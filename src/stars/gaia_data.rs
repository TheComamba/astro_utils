use crate::{
    color::sRGBColor,
    coordinates::spherical::SphericalCoordinates,
    error::AstroUtilError,
    units::{
        angle::Angle, illuminance::Illuminance, length::Length, luminosity::Luminosity, temperature,
    },
    Float,
};
use serde::{Deserialize, Serialize};

use super::star::Star;

#[derive(Serialize, Deserialize)]
struct GaiaMetadataLine {
    name: String,
    datatype: String,
    xtype: Option<String>,
    arraysize: Option<u32>,
    description: String,
    unit: String,
    ucd: String,
    utype: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GaiaResponse {
    metadata: Vec<GaiaMetadataLine>,
    data: Vec<Vec<Option<Float>>>,
}

impl GaiaResponse {
    fn get_distance(distance: &Option<Float>, parallax: &Option<Float>) -> Option<Length> {
        match (distance, parallax) {
            (Some(distance), _) => Some(Length::from_parsecs(*distance)),
            (_, Some(parallax)) => {
                if parallax > &0. {
                    Some(Length::from_parsecs(1000.0 / parallax))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn get_luminosity(mag: Float, distance: &Length) -> Luminosity {
        let illuminance = Illuminance::from_apparent_magnitude(mag);
        illuminance.to_luminosity(distance)
    }

    fn to_stars(&self) -> Result<Vec<Star>, AstroUtilError> {
        const ECL_LON_INDEX: usize = 0;
        const ECL_LAT_INDEX: usize = 1;
        const MAG_INDEX: usize = 2;
        const DISTANCE_INDEX: usize = 3;
        const PARALLAX_INDEX: usize = 4;
        const TEMPERATURE_INDEX: usize = 5;
        let mut stars = Vec::new();
        for row in self.data.iter() {
            let ecl_lon = row[ECL_LON_INDEX];
            let ecl_lat = row[ECL_LAT_INDEX];
            let mag = row[MAG_INDEX];
            let distance = row[DISTANCE_INDEX];
            let parallax = row[PARALLAX_INDEX];
            let temperature = row[TEMPERATURE_INDEX];

            let distance = Self::get_distance(&distance, &parallax);
            let mag = mag.ok_or(AstroUtilError::DataNotAvailable)?;
            let (distance, luminosity) = match distance {
                Some(distance) => (distance, Self::get_luminosity(mag, &distance)),
                None => (
                    Length::from_parsecs(10.),
                    Luminosity::from_absolute_magnitude(mag),
                ),
            };
            let ecl_lon = Angle::from_degrees(ecl_lon.ok_or(AstroUtilError::DataNotAvailable)?);
            let ecl_lat = Angle::from_degrees(ecl_lat.ok_or(AstroUtilError::DataNotAvailable)?);
            let direction_in_ecliptic = SphericalCoordinates::new(ecl_lon, ecl_lat).to_direction();

            let temperature =
                temperature.map(|temperature| temperature::Temperature::from_kelvin(temperature));
            let color = match temperature {
                Some(temperature) => sRGBColor::from_temperature(temperature),
                None => sRGBColor::DEFAULT,
            };

            let star = Star {
                name: "".to_string(),
                mass: None,
                radius: None,
                luminosity,
                temperature,
                color,
                age: None,
                distance,
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
    url += "&QUERY=SELECT+ecl_lon,ecl_lat,phot_g_mean_mag,distance_gspphot,parallax,teff_gspphot";
    url += "+FROM+gaiadr3.gaia_source";
    url += "+WHERE+phot_g_mean_mag+<+";
    url += &format!("{:.1}", brightest.as_apparent_magnitude());
    let resp = reqwest::blocking::get(&url)
        .map_err(AstroUtilError::Connection)?
        .text()
        .map_err(AstroUtilError::Connection)?;
    serde_json::from_str(&resp).map_err(AstroUtilError::Json)
}

fn gaia_star_is_already_known(gaia_star: &Star, known_stars: &[Star]) -> bool {
    known_stars
        .iter()
        .any(|known_star| known_star.apparently_the_same(gaia_star))
}

pub fn complement_brightest_stars(mut known_stars: Vec<Star>) -> Result<Vec<Star>, AstroUtilError> {
    let brightest = Illuminance::from_apparent_magnitude(6.5);
    let resp = query_brightest_stars(brightest)?;
    let gaia_stars = resp.to_stars()?;
    let mut new_stars = vec![];
    for gaia_star in gaia_stars
        .into_iter()
        .filter(|star| !gaia_star_is_already_known(star, &known_stars))
    {
        new_stars.push(gaia_star);
    }
    known_stars.append(&mut new_stars);
    Ok(known_stars)
}

#[cfg(test)]
mod tests {
    use crate::data::stars::STARS_TO_TWO_POINT_FIVE_APPARENT_MAG;

    use super::*;

    #[test]
    fn some_bright_stars_are_already_known() {
        let mut known_stars = vec![];
        for star_data in STARS_TO_TWO_POINT_FIVE_APPARENT_MAG {
            known_stars.push(star_data.to_star());
        }

        let gaia_response =
            query_brightest_stars(Illuminance::from_apparent_magnitude(3.5)).unwrap();
        let gaia_stars = gaia_response.to_stars().unwrap();
        println!("gaia_stars.len(): {}", gaia_stars.len());
        for gaia_star in gaia_stars.iter() {
            if gaia_star_is_already_known(gaia_star, &known_stars) {
                println!("This star is already known: {:?}", gaia_star);
            }
        }
        assert!(gaia_stars.len() > 100);
        assert!(gaia_stars
            .iter()
            .any(|gaia_star| { gaia_star_is_already_known(gaia_star, &known_stars) }));
    }
}
