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
    fn to_star_appearances(&self) -> Result<Vec<StarAppearance>, AstroUtilError> {
        const ECL_LON_INDEX: usize = 0;
        const ECL_LAT_INDEX: usize = 1;
        const MAG_INDEX: usize = 2;
        const TEMPERATURE_INDEX: usize = 3;
        let mut stars = Vec::new();
        for row in self.data.iter() {
            let ecl_lon = row[ECL_LON_INDEX];
            let ecl_lat = row[ECL_LAT_INDEX];
            let mag = row[MAG_INDEX];
            let temperature = row[TEMPERATURE_INDEX];

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
                name: "".to_string(),
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
    url += "&QUERY=SELECT+ecl_lon,ecl_lat,phot_g_mean_mag,teff_gspphot";
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
    use crate::data::stars::STARS_TO_TWO_POINT_FIVE_APPARENT_MAG;

    use super::*;

    #[test]
    fn some_bright_stars_are_already_known() {
        let mut known_stars = vec![];
        for star_data in STARS_TO_TWO_POINT_FIVE_APPARENT_MAG {
            known_stars.push(star_data.to_star_appearance());
        }

        let gaia_response =
            query_brightest_stars(Illuminance::from_apparent_magnitude(3.5)).unwrap();
        let gaia_stars = gaia_response.to_star_appearances().unwrap();
        println!("gaia_stars.len(): {}", gaia_stars.len());
        for gaia_star in gaia_stars.iter() {
            if star_is_already_known(gaia_star, &known_stars.iter().collect()) {
                println!("This star is already known: {:?}", gaia_star);
            }
        }
        assert!(gaia_stars.len() > 100);
        assert!(gaia_stars
            .iter()
            .any(|gaia_star| { star_is_already_known(gaia_star, &known_stars.iter().collect()) }));
    }
}
