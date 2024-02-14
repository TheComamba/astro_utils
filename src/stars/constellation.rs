use super::{star_appearance::StarAppearance, star_data::StarData};
use simple_si_units::geometry::Angle;
use std::cmp::Ordering;

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

pub struct Connection {
    from: usize,
    to: usize,
    distance: Angle<f64>,
}

impl Connection {
    fn new(from: usize, to: usize, stars: &[StarAppearance]) -> Self {
        let distance = stars[from].get_pos().angle_to(stars[to].get_pos());
        Connection { from, to, distance }
    }

    pub fn get_indices(&self) -> (usize, usize) {
        (self.from, self.to)
    }

    fn connects_to(&self, i: usize) -> bool {
        self.from == i || self.to == i
    }

    fn other_end(&self, i: usize) -> usize {
        if self.from == i {
            self.to
        } else {
            self.from
        }
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to)
            || (self.from == other.to && self.to == other.from)
    }
}

fn shortest_path(start: usize, end: usize, connections: &[Connection]) -> Option<Vec<&Connection>> {
    let mut paths: Vec<Vec<&Connection>> = Vec::new();
    for connection in connections {
        if connection.connects_to(start) {
            let mut path = vec![connection];
            if connection.connects_to(end) {
                return Some(path);
            } else if let Some(mut sub_path) =
                shortest_path(connection.other_end(start), end, connections)
            {
                path.append(&mut sub_path);
                paths.push(path);
            }
        }
    }
    if paths.is_empty() {
        return None;
    }
    paths.sort_by(|a, b| a.len().cmp(&b.len()));
    Some(paths[0].clone())
}

fn is_reachable_within(
    start: usize,
    end: usize,
    max_steps: usize,
    connections: &[Connection],
) -> bool {
    if max_steps == 0 && start != end {
        return false;
    }
    for connection in connections {
        if connection.connects_to(start) {
            if connection.connects_to(end) {
                return true;
            } else if is_reachable_within(
                connection.other_end(start),
                end,
                max_steps - 1,
                connections,
            ) {
                return true;
            }
        }
    }
    false
}

fn connections_sorted_by_distance(stars: &[StarAppearance]) -> Vec<Vec<Connection>> {
    let mut connections: Vec<Vec<Connection>> = Vec::new();
    for i in 0..stars.len() {
        let mut star_connections: Vec<Connection> = Vec::new();
        for j in 0..stars.len() {
            if i != j {
                star_connections.push(Connection::new(i, j, stars));
            }
        }
        star_connections.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(Ordering::Equal)
        });
        connections.push(star_connections);
    }
    connections
}

fn sorted_connections(stars: &[StarAppearance]) -> Vec<Connection> {
    let mut connections: Vec<Connection> = Vec::new();
    for i in 0..stars.len() {
        for j in i + 1..stars.len() {
            connections.push(Connection::new(i, j, stars));
        }
    }
    connections.sort_by(|a, b| {
        a.distance
            .partial_cmp(&b.distance)
            .unwrap_or(Ordering::Equal)
    });
    connections
}

fn nearest_neighbours(i: usize, stars: &[StarAppearance]) -> Vec<usize> {
    let mut neighbours: Vec<usize> = Vec::new();
    for j in 0..stars.len() {
        if i != j {
            neighbours.push(j);
        }
    }
    neighbours.sort_by(|a, b| {
        stars[i]
            .get_pos()
            .angle_to(stars[*a].get_pos())
            .partial_cmp(&stars[i].get_pos().angle_to(stars[*b].get_pos()))
            .unwrap_or(Ordering::Equal)
    });
    neighbours
}

fn all_nearest_neighbours(stars: &[StarAppearance]) -> Vec<Vec<usize>> {
    let mut all_neighbours: Vec<Vec<usize>> = Vec::new();
    for i in 0..stars.len() {
        all_neighbours.push(nearest_neighbours(i, stars));
    }
    all_neighbours
}

fn get_max_allowed_steps(end: usize, nearest_neighbours: &Vec<usize>) -> usize {
    nearest_neighbours
        .iter()
        .position(|&i| i == end)
        .unwrap_or(0)
        + 1
}

fn collect_connections(stars: &[StarAppearance]) -> Vec<Connection> {
    let all_nearest_neighbours = all_nearest_neighbours(stars);
    let all_connections = sorted_connections(stars);
    let mut connections: Vec<Connection> = Vec::new();
    for connection in all_connections {
        if connections.contains(&connection) {
            continue;
        }

        let start = connection.get_indices().0;
        let end = connection.get_indices().1;

        let max_steps =
            get_max_allowed_steps(end, &all_nearest_neighbours[connection.get_indices().0]);
        if is_reachable_within(start, end, max_steps, &connections) {
            connections.push(connection);
        }
    }
    connections
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
    use crate::real_data::stars::BRIGHTEST_STARS;

    #[test]
    fn there_are_88_constellations() {
        let all_stars = BRIGHTEST_STARS
            .iter()
            .map(|star| star.to_star_data())
            .collect::<Vec<_>>();
        let constellations = crate::stars::constellation::collect_constellations(&all_stars);
        let mut constellation_names = constellations
            .iter()
            .map(|constellation| constellation.get_name())
            .collect::<Vec<_>>();
        constellation_names.sort();
        for name in constellation_names {
            println!("{}", name);
        }
        assert_eq!(constellations.len(), 88);
    }

    #[test]
    fn all_constellations_have_at_least_three_stars() {
        let all_stars = BRIGHTEST_STARS
            .iter()
            .map(|star| star.to_star_data())
            .collect::<Vec<_>>();
        let constellations = crate::stars::constellation::collect_constellations(&all_stars);
        for constellation in constellations {
            println!(
                "{}: {}",
                constellation.get_name(),
                constellation.get_stars().len()
            );
            assert!(constellation.get_stars().len() >= 3);
        }
    }
}
