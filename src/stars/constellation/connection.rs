use crate::stars::star_appearance::StarAppearance;
use simple_si_units::geometry::Angle;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
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

fn get_max_allowed_steps(i: usize, j: usize, all_nearest_neighbours: &Vec<Vec<usize>>) -> usize {
    let steps1 = all_nearest_neighbours[i]
        .iter()
        .position(|&ind| ind == j)
        .unwrap_or(0)
        + 1;
    let steps2 = all_nearest_neighbours[j]
        .iter()
        .position(|&ind| ind == i)
        .unwrap_or(0)
        + 1;
    steps1.min(steps2)
}

pub(super) fn collect_connections(stars: &[StarAppearance]) -> Vec<Connection> {
    let all_nearest_neighbours = all_nearest_neighbours(stars);
    let all_connections = sorted_connections(stars);
    let mut connections: Vec<Connection> = Vec::new();
    for connection in all_connections {
        if connections.contains(&connection) {
            continue;
        }

        let start = connection.get_indices().0;
        let end = connection.get_indices().1;

        let max_steps = get_max_allowed_steps(start, end, &all_nearest_neighbours);
        if !is_reachable_within(start, end, max_steps, &connections) {
            connections.push(connection);
        }
    }
    connections
}

#[cfg(test)]
fn find_nearest_neighbour(
    index: usize,
    stars: &[StarAppearance],
    excluding: &Vec<usize>,
) -> Option<usize> {
    let mut nearest_neighbour = None;
    let pos = stars[index].get_pos();
    for j in 0..stars.len() {
        if index != j && !excluding.contains(&j) {
            let distance = stars[j].get_pos().angle_to(pos);
            if let Some(nn) = nearest_neighbour {
                let nn_distance = stars[nn as usize].get_pos().angle_to(pos);
                if distance < nn_distance {
                    nearest_neighbour = Some(j);
                }
            } else {
                nearest_neighbour = Some(j);
            }
        }
    }
    nearest_neighbour
}

#[cfg(test)]
fn minimum_spanning_tree(stars: &[StarAppearance]) -> Vec<Connection> {
    use crate::units::angle::FULL_CIRC;

    // This is Prim's algorithm
    let mut connections = Vec::new();
    if stars.len() < 2 {
        return connections;
    }
    let mut visited = vec![0];
    while visited.len() < stars.len() {
        let mut current_best = Connection {
            to: 0,
            from: 0,
            distance: FULL_CIRC,
        };
        for i in &visited {
            let nn = find_nearest_neighbour(*i, stars, &visited);
            if let Some(nn) = nn {
                let connection = Connection::new(*i, nn, stars);
                if connection.distance < current_best.distance {
                    current_best = connection;
                }
            }
        }
        visited.push(current_best.to);
        connections.push(current_best);
    }
    connections
}

#[cfg(test)]
mod tests {
    use simple_si_units::electromagnetic::Illuminance;

    use super::*;
    use crate::{
        color::sRGBColor,
        coordinates::spherical::SphericalCoordinates,
        real_data::stars::BRIGHTEST_STARS,
        stars::constellation::constellation::collect_constellations,
        units::{angle::ANGLE_ZERO, tests::ANGLE_TEST_ACCURACY},
    };

    fn stars_in_line(size: usize) -> Vec<StarAppearance> {
        let mut stars = Vec::new();
        for i in 0..size {
            // Making distances distinct
            let longitude = Angle::from_degrees(10. * i as f64 + (i as f64).powi(2) / 100.);
            assert!(longitude.to_degrees() < 179.0);
            let pos = SphericalCoordinates::new(longitude, ANGLE_ZERO).to_ecliptic();
            stars.push(StarAppearance::new(
                format!("Star {}", i),
                Illuminance::from_lux(1.0),
                sRGBColor::DEFAULT,
                pos,
            ));
        }
        stars
    }

    fn connections_in_line(size: usize) -> Vec<Connection> {
        let mut connections = Vec::new();
        for i in 0..size {
            connections.push(Connection {
                from: i,
                to: i + 1,
                distance: ANGLE_ZERO,
            });
        }
        connections
    }

    #[test]
    fn nearest_neighbours_of_line_are_sorted() {
        let size = 10;
        let stars = stars_in_line(size);
        let neighbours = nearest_neighbours(0, &stars);
        assert!(neighbours.len() == size - 1);
        for i in 1..size {
            assert!(neighbours[i - 1] == i);
        }

        let nearest_neighbours = nearest_neighbours(size - 1, &stars);
        assert!(nearest_neighbours.len() == size - 1);
        for i in 1..size {
            assert!(nearest_neighbours[i - 1] == size - 1 - i);
        }
    }

    #[test]
    fn all_nearest_neighbours_for_short_line() {
        let stars = stars_in_line(3);
        let all_neighbours = all_nearest_neighbours(&stars);
        let expected = vec![vec![1, 2], vec![0, 2], vec![1, 0]];
        assert!(all_neighbours == expected);
    }

    #[test]
    fn is_reachable() {
        let size = 10;
        let connections = connections_in_line(size);
        for i in 0..size {
            assert!(is_reachable_within(0, i, i, &connections));
            assert!(!is_reachable_within(0, i + 1, i, &connections));
        }
    }

    #[test]
    fn collect_connections_for_line() {
        for size in 1..10 {
            let stars = stars_in_line(size);
            let connections = collect_connections(&stars);
            assert!(connections.len() == size - 1);
            for i in 0..size - 1 {
                assert!(connections[i].get_indices() == (i, i + 1));
            }
        }
    }

    #[test]
    fn minimum_spanning_tree_has_length_n_minus_1() {
        let all_stars = BRIGHTEST_STARS
            .iter()
            .map(|star| star.to_star_data())
            .collect::<Vec<_>>();
        let all_consteallations = collect_constellations(&all_stars[..]);
        for constellation in all_consteallations {
            let mst = minimum_spanning_tree(&constellation.get_stars());
            assert!(mst.len() == constellation.get_stars().len() - 1);
        }
    }

    #[test]
    fn minimum_spanning_tree_is_contained_in_constellation_connections() {
        let all_stars = BRIGHTEST_STARS
            .iter()
            .map(|star| star.to_star_data())
            .collect::<Vec<_>>();
        let all_consteallations = collect_constellations(&all_stars[..]);
        for constellation in all_consteallations {
            println!("\nChecking {}", constellation.get_name());
            let connections = constellation.get_connections();
            let mst = minimum_spanning_tree(&constellation.get_stars());
            for mst_connection in mst {
                assert!(connections.contains(&mst_connection));
            }
        }
    }

    fn index_independent_cmp(
        con1: &Connection,
        stars1: &[StarAppearance],
        con2: &Connection,
        stars2: &[StarAppearance],
    ) -> bool {
        let pos_1_1 = stars1[con1.from].get_pos();
        let pos_1_2 = stars1[con1.to].get_pos();
        let pos_2_1 = stars2[con2.from].get_pos();
        let pos_2_2 = stars2[con2.to].get_pos();
        let case1 = pos_1_1.eq_within(pos_2_1, ANGLE_TEST_ACCURACY)
            && pos_1_2.eq_within(pos_2_2, ANGLE_TEST_ACCURACY);
        let case2 = pos_1_1.eq_within(pos_2_2, ANGLE_TEST_ACCURACY)
            && pos_1_2.eq_within(pos_2_1, ANGLE_TEST_ACCURACY);
        case1 || case2
    }

    fn nice_print(connections: &[Connection], stars: &[StarAppearance]) {
        println!("");
        for connection in connections {
            let mut names = vec![
                stars[connection.from].get_name(),
                stars[connection.to].get_name(),
            ];
            names.sort();
            println!("{} - {} : {}", names[0], names[1], connection.distance);
        }
    }

    #[test]
    fn constellation_connection_is_independent_of_order() {
        let all_stars = BRIGHTEST_STARS
            .iter()
            .map(|star| star.to_star_data())
            .collect::<Vec<_>>();
        let all_consteallations = collect_constellations(&all_stars[..]);
        for constellation in all_consteallations {
            println!("\nChecking {}", constellation.get_name());
            let connections = constellation.get_connections();
            let mut stars_rev = constellation.get_stars().clone();
            stars_rev.reverse();
            let connections_rev = collect_connections(&stars_rev);

            nice_print(connections, constellation.get_stars());
            nice_print(&connections_rev, &stars_rev);
            assert!(connections.len() == connections_rev.len());
            for connection in connections {
                assert!(connections_rev.iter().any(|con_rev| index_independent_cmp(
                    connection,
                    &constellation.get_stars(),
                    con_rev,
                    &stars_rev
                )));
            }
        }
    }
}
