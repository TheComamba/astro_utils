use super::connection::{collect_connections, Connection};
use crate::stars::{appearance::StarAppearance, data::StarData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constellation {
    name: String,
    stars: Vec<StarAppearance>,
    connections: Vec<Connection>,
}

impl Constellation {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_stars(&self) -> &Vec<StarAppearance> {
        &self.stars
    }

    pub fn get_connections(&self) -> &Vec<Connection> {
        &self.connections
    }
}

fn collect_constellation_names(all_stars: &[StarData]) -> Vec<String> {
    let mut constellation_names: Vec<String> = Vec::new();
    for star in all_stars {
        if let Some(constellation) = star.get_constellation() {
            if !constellation_names.contains(constellation) {
                constellation_names.push(constellation.clone());
            }
        }
    }
    constellation_names
}

fn collect_stars_in_constellation<'a>(
    constellation_name: &str,
    all_stars: &'a [StarData],
) -> Vec<&'a StarData> {
    let mut stars_in_constellation: Vec<&StarData> = Vec::new();
    for star in all_stars {
        if let Some(constellation) = star.get_constellation() {
            if constellation == constellation_name {
                stars_in_constellation.push(star);
            }
        }
    }
    stars_in_constellation
}

pub fn collect_constellations(all_stars: &[StarData]) -> Vec<Constellation> {
    let constellation_names = collect_constellation_names(all_stars);
    let mut constellations: Vec<Constellation> = Vec::new();
    for constellation_name in constellation_names {
        let stars_in_constellation = collect_stars_in_constellation(&constellation_name, all_stars);
        let mut stars: Vec<StarAppearance> = Vec::new();
        for star in stars_in_constellation {
            stars.push(star.to_star_appearance());
        }
        let connections = collect_connections(&stars[..]);
        constellations.push(Constellation {
            name: constellation_name,
            stars,
            connections,
        });
    }
    constellations
}

#[cfg(test)]
mod tests {
    use crate::real_data::stars::all::get_many_stars;

    use super::*;

    #[test]
    fn there_are_88_constellations() {
        const EXPECTED_CONSTELLATIONS: [&str; 88] = [
            "Andromeda",
            "Antlia",
            "Apus",
            "Aquarius",
            "Aquila",
            "Ara",
            "Aries",
            "Auriga",
            "Boötes",
            "Caelum",
            "Camelopardalis",
            "Cancer",
            "Canes Venatici",
            "Canis Major",
            "Canis Minor",
            "Capricornus",
            "Carina",
            "Cassiopeia",
            "Centaurus",
            "Cepheus",
            "Cetus",
            "Chamaeleon",
            "Circinus",
            "Columba",
            "Coma Berenices",
            "Corona Australis",
            "Corona Borealis",
            "Corvus",
            "Crater",
            "Crux",
            "Cygnus",
            "Delphinus",
            "Dorado",
            "Draco",
            "Equuleus",
            "Eridanus",
            "Fornax",
            "Gemini",
            "Grus",
            "Hercules",
            "Horologium",
            "Hydra",
            "Hydrus",
            "Indus",
            "Lacerta",
            "Leo",
            "Leo Minor",
            "Lepus",
            "Libra",
            "Lupus",
            "Lynx",
            "Lyra",
            "Mensa",
            "Microscopium",
            "Monoceros",
            "Musca",
            "Norma",
            "Octans",
            "Ophiuchus",
            "Orion",
            "Pavo",
            "Pegasus",
            "Perseus",
            "Phoenix",
            "Pictor",
            "Pisces",
            "Piscis Austrinus",
            "Puppis",
            "Pyxis",
            "Reticulum",
            "Sagitta",
            "Sagittarius",
            "Scorpius",
            "Sculptor",
            "Scutum",
            "Serpens",
            "Sextans",
            "Taurus",
            "Telescopium",
            "Triangulum",
            "Triangulum Australe",
            "Tucana",
            "Ursa Major",
            "Ursa Minor",
            "Vela",
            "Virgo",
            "Volans",
            "Vulpecula",
        ];
        assert!(EXPECTED_CONSTELLATIONS.len() == 88);

        let all_stars = get_many_stars()
            .iter()
            .map(|star| star.to_star_data())
            .collect::<Vec<_>>();
        let constellations = collect_constellations(&all_stars);
        let mut constellation_names = constellations
            .iter()
            .map(|constellation| constellation.get_name())
            .collect::<Vec<_>>();
        constellation_names.sort();
        let mut something_is_wrong = false;
        for name in EXPECTED_CONSTELLATIONS {
            if !constellation_names.contains(&name) {
                something_is_wrong = true;
                println!("{} is missing", name);
            }
        }
        for name in constellation_names {
            if !EXPECTED_CONSTELLATIONS.contains(&name) {
                something_is_wrong = true;
                println!("{} is not expected", name);
            }
        }
        assert_eq!(constellations.len(), 88);
        assert!(!something_is_wrong);
    }

    #[test]
    fn all_constellations_have_at_least_three_stars() {
        let all_stars = get_many_stars()
            .iter()
            .map(|star| star.to_star_data())
            .collect::<Vec<_>>();
        let constellations = collect_constellations(&all_stars);
        for constellation in constellations {
            assert!(
                constellation.get_stars().len() >= 3,
                "{}: {}",
                constellation.get_name(),
                constellation.get_stars().len()
            );
        }
    }
}
