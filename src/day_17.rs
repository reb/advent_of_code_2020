/// --- Day 17: Conway Cubes ---
///
/// As your flight slowly drifts through the sky, the Elves at the Mythical
/// Information Bureau at the North Pole contact you. They'd like some help
/// debugging a malfunctioning experimental energy source aboard one of their
/// super-secret imaging satellites.
///
/// The experimental energy source is based on cutting-edge technology: a set of
/// Conway Cubes contained in a pocket dimension! When you hear it's having
/// problems, you can't help but agree to take a look.
///
/// The pocket dimension contains an infinite 3-dimensional grid. At every
/// integer 3-dimensional coordinate (x,y,z), there exists a single cube which
/// is either active or inactive.
///
/// In the initial state of the pocket dimension, almost all cubes start
/// inactive. The only exception to this is a small flat region of cubes (your
/// puzzle input); the cubes in this region start in the specified active (#) or
/// inactive (.) state.
///
/// The energy source then proceeds to boot up by executing six cycles.
///
/// Each cube only ever considers its neighbors: any of the 26 other cubes where
/// any of their coordinates differ by at most 1. For example, given the cube at
/// x=1,y=2,z=3, its neighbors include the cube at x=2,y=2,z=2, the cube at
/// x=0,y=2,z=3, and so on.
///
/// During a cycle, all cubes simultaneously change their state according to the
/// following rules:
///
///   - If a cube is active and exactly 2 or 3 of its neighbors are also active,
///     the cube remains active. Otherwise, the cube becomes inactive.
///   - If a cube is inactive but exactly 3 of its neighbors are active, the
///     cube becomes active. Otherwise, the cube remains inactive.
///
/// The engineers responsible for this experimental energy source would like you
/// to simulate the pocket dimension and determine what the configuration of
/// cubes should be at the end of the six-cycle boot process.
///
/// For example, consider the following initial state:
///
/// .#.
/// ..#
/// ###
///
/// Even though the pocket dimension is 3-dimensional, this initial state
/// represents a small 2-dimensional slice of it. (In particular, this initial
/// state defines a 3x3x1 region of the 3-dimensional space.)
///
/// Simulating a few cycles from this initial state produces the following
/// configurations, where the result of each cycle is shown layer-by-layer at
/// each given z coordinate (and the frame of view follows the active cells in
/// each cycle):
///
/// Before any cycles:
///
/// z=0
/// .#.
/// ..#
/// ###
///
///
/// After 1 cycle:
///
/// z=-1
/// #..
/// ..#
/// .#.
///
/// z=0
/// #.#
/// .##
/// .#.
///
/// z=1
/// #..
/// ..#
/// .#.
///
///
/// After 2 cycles:
///
/// z=-2
/// .....
/// .....
/// ..#..
/// .....
/// .....
///
/// z=-1
/// ..#..
/// .#..#
/// ....#
/// .#...
/// .....
///
/// z=0
/// ##...
/// ##...
/// #....
/// ....#
/// .###.
///
/// z=1
/// ..#..
/// .#..#
/// ....#
/// .#...
/// .....
///
/// z=2
/// .....
/// .....
/// ..#..
/// .....
/// .....
///
///
/// After 3 cycles:
///
/// z=-2
/// .......
/// .......
/// ..##...
/// ..###..
/// .......
/// .......
/// .......
///
/// z=-1
/// ..#....
/// ...#...
/// #......
/// .....##
/// .#...#.
/// ..#.#..
/// ...#...
///
/// z=0
/// ...#...
/// .......
/// #......
/// .......
/// .....##
/// .##.#..
/// ...#...
///
/// z=1
/// ..#....
/// ...#...
/// #......
/// .....##
/// .#...#.
/// ..#.#..
/// ...#...
///
/// z=2
/// .......
/// .......
/// ..##...
/// ..###..
/// .......
/// .......
/// .......
///
/// After the full six-cycle boot process completes, 112 cubes are left in the
/// active state.
///
/// Starting with your given initial configuration, simulate six cycles. How
/// many cubes are left in the active state after the sixth cycle?
///
/// --- Part Two ---
///
/// For some reason, your simulated results don't match what the experimental
/// energy source engineers expected. Apparently, the pocket dimension actually
/// has four spatial dimensions, not three.
///
/// The pocket dimension contains an infinite 4-dimensional grid. At every
/// integer 4-dimensional coordinate (x,y,z,w), there exists a single cube
/// (really, a hypercube) which is still either active or inactive.
///
/// Each cube only ever considers its neighbors: any of the 80 other cubes where
/// any of their coordinates differ by at most 1. For example, given the cube at
/// x=1,y=2,z=3,w=4, its neighbors include the cube at x=2,y=2,z=3,w=3, the cube
/// at x=0,y=2,z=3,w=4, and so on.
///
/// The initial state of the pocket dimension still consists of a small flat
/// region of cubes. Furthermore, the same rules for cycle updating still apply:
/// during each cycle, consider the number of active neighbors of each cube.
///
/// For example, consider the same initial state as in the example above. Even
/// though the pocket dimension is 4-dimensional, this initial state represents
/// a small 2-dimensional slice of it. (In particular, this initial state
/// defines a 3x3x1x1 region of the 4-dimensional space.)
///
/// Simulating a few cycles from this initial state produces the following
/// configurations, where the result of each cycle is shown layer-by-layer at
/// each given z and w coordinate:
///
/// Before any cycles:
///
/// z=0, w=0
/// .#.
/// ..#
/// ###
///
///
/// After 1 cycle:
///
/// z=-1, w=-1
/// #..
/// ..#
/// .#.
///
/// z=0, w=-1
/// #..
/// ..#
/// .#.
///
/// z=1, w=-1
/// #..
/// ..#
/// .#.
///
/// z=-1, w=0
/// #..
/// ..#
/// .#.
///
/// z=0, w=0
/// #.#
/// .##
/// .#.
///
/// z=1, w=0
/// #..
/// ..#
/// .#.
///
/// z=-1, w=1
/// #..
/// ..#
/// .#.
///
/// z=0, w=1
/// #..
/// ..#
/// .#.
///
/// z=1, w=1
/// #..
/// ..#
/// .#.
///
///
/// After 2 cycles:
///
/// z=-2, w=-2
/// .....
/// .....
/// ..#..
/// .....
/// .....
///
/// z=-1, w=-2
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=0, w=-2
/// ###..
/// ##.##
/// #...#
/// .#..#
/// .###.
///
/// z=1, w=-2
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=2, w=-2
/// .....
/// .....
/// ..#..
/// .....
/// .....
///
/// z=-2, w=-1
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=-1, w=-1
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=0, w=-1
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=1, w=-1
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=2, w=-1
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=-2, w=0
/// ###..
/// ##.##
/// #...#
/// .#..#
/// .###.
///
/// z=-1, w=0
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=0, w=0
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=1, w=0
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=2, w=0
/// ###..
/// ##.##
/// #...#
/// .#..#
/// .###.
///
/// z=-2, w=1
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=-1, w=1
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=0, w=1
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=1, w=1
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=2, w=1
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=-2, w=2
/// .....
/// .....
/// ..#..
/// .....
/// .....
///
/// z=-1, w=2
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=0, w=2
/// ###..
/// ##.##
/// #...#
/// .#..#
/// .###.
///
/// z=1, w=2
/// .....
/// .....
/// .....
/// .....
/// .....
///
/// z=2, w=2
/// .....
/// .....
/// ..#..
/// .....
/// .....
///
/// After the full six-cycle boot process completes, 848 cubes are left in the
/// active state.
///
/// Starting with your given initial configuration, simulate six cycles in a
/// 4-dimensional space. How many cubes are left in the active state after the
/// sixth cycle?
use num::iter::range_inclusive;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

const INPUT: &str = include_str!("../input/day_17.txt");

pub fn run() {
    let mut cubes_3d = parse_cubes::<Point3D>(INPUT);
    for _ in 0..6 {
        cubes_3d = iterate(cubes_3d);
    }
    println!(
        "After simulating six cycles the amount of cubes left in the activated state is: {}",
        cubes_3d.len()
    );

    let mut cubes_4d = parse_cubes::<Point4D>(INPUT);
    for _ in 0..6 {
        cubes_4d = iterate(cubes_4d);
    }
    println!(
        "After six cycles in 4-dimensional space the amount of activated cubes is: {}",
        cubes_4d.len()
    );
}

fn get_neighbour_counts<D: Point>(cubes: &Cubes<D>) -> HashMap<D, usize> {
    cubes
        .iter()
        .flat_map(|point| point.neighbours())
        .fold(HashMap::new(), |mut map, neighbour| {
            *map.entry(neighbour).or_insert(0) += 1;
            map
        })
}

fn iterate<D: Point>(cubes: Cubes<D>) -> Cubes<D> {
    let neighbour_counts = get_neighbour_counts(&cubes);

    neighbour_counts
        .iter()
        .filter_map(|(point, count)| {
            match count {
                2 => cubes.get(point), // only active if cube already is active
                3 => Some(point),      // always activate
                _ => None,
            }
        })
        .cloned()
        .collect()
}

// marker trait for Points
trait Point: Sized + Eq + Clone + Hash {
    fn neighbours(&self) -> Vec<Self>;

    fn new(x: i32, y: i32) -> Self;
}

type Point3D = (i32, i32, i32);
impl Point for Point3D {
    fn neighbours(&self) -> Vec<Self> {
        let (x, y, z) = *self;
        range_inclusive(-1, 1)
            .map(move |dz| {
                range_inclusive(-1, 1)
                    .map(move |dy| range_inclusive(-1, 1).map(move |dx| (x + dx, y + dy, z + dz)))
                    .flatten()
            })
            .flatten()
            .filter(|point| *point != (x, y, z))
            .collect()
    }

    fn new(x: i32, y: i32) -> Self {
        (x, y, 0)
    }
}

type Point4D = (i32, i32, i32, i32);
impl Point for Point4D {
    fn neighbours(&self) -> Vec<Self> {
        let (x, y, z, w) = *self;
        range_inclusive(-1, 1)
            .map(move |dw| {
                range_inclusive(-1, 1)
                    .map(move |dz| {
                        range_inclusive(-1, 1)
                            .map(move |dy| {
                                range_inclusive(-1, 1)
                                    .map(move |dx| (x + dx, y + dy, z + dz, w + dw))
                            })
                            .flatten()
                    })
                    .flatten()
            })
            .flatten()
            .filter(|point| *point != (x, y, z, w))
            .collect()
    }

    fn new(x: i32, y: i32) -> Self {
        (x, y, 0, 0)
    }
}
type Cubes<D> = HashSet<D>;

fn parse_cubes<D: Point>(input: &str) -> Cubes<D> {
    input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars().enumerate().filter_map(move |(y, c)| match c {
                '#' => Some(D::new(x as i32, y as i32)),
                _ => None,
            })
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cubes() {
        let input = ".#.\n..#\n###";

        let mut expected_cubes = Cubes::new();
        // .#.
        expected_cubes.insert((0, 1, 0));
        // ..#
        expected_cubes.insert((1, 2, 0));
        // ###
        expected_cubes.insert((2, 0, 0));
        expected_cubes.insert((2, 1, 0));
        expected_cubes.insert((2, 2, 0));

        assert_eq!(parse_cubes(input), expected_cubes);
    }

    #[test]
    fn test_neighbours() {
        let point = (0, 0, 0);
        let expected_neighbours = vec![
            (-1, -1, -1),
            (0, -1, -1),
            (1, -1, -1),
            (-1, 0, -1),
            (0, 0, -1),
            (1, 0, -1),
            (-1, 1, -1),
            (0, 1, -1),
            (1, 1, -1),
            (-1, -1, 0),
            (0, -1, 0),
            (1, -1, 0),
            (-1, 0, 0),
            (1, 0, 0),
            (-1, 1, 0),
            (0, 1, 0),
            (1, 1, 0),
            (-1, -1, 1),
            (0, -1, 1),
            (1, -1, 1),
            (-1, 0, 1),
            (0, 0, 1),
            (1, 0, 1),
            (-1, 1, 1),
            (0, 1, 1),
            (1, 1, 1),
        ];

        assert_eq!(point.neighbours(), expected_neighbours);
    }

    #[test]
    fn test_iterate() {
        let mut cubes = Cubes::new();
        // .#.
        cubes.insert((0, 1, 0));
        // ..#
        cubes.insert((1, 2, 0));
        // ###
        cubes.insert((2, 0, 0));
        cubes.insert((2, 1, 0));
        cubes.insert((2, 2, 0));

        let mut expected_cubes = Cubes::new();
        // view shifted down one (x + 1)
        // z=-1
        // #..
        expected_cubes.insert((1, 0, -1));
        // ..#
        expected_cubes.insert((2, 2, -1));
        // .#.
        expected_cubes.insert((3, 1, -1));

        // z=0
        // #.#
        expected_cubes.insert((1, 0, 0));
        expected_cubes.insert((1, 2, 0));
        // .##
        expected_cubes.insert((2, 1, 0));
        expected_cubes.insert((2, 2, 0));
        // .#.
        expected_cubes.insert((3, 1, 0));

        // z=1
        // #..
        expected_cubes.insert((1, 0, 1));
        // ..#
        expected_cubes.insert((2, 2, 1));
        // .#.
        expected_cubes.insert((3, 1, 1));

        assert_eq!(iterate(cubes), expected_cubes);
    }

    #[test]
    fn test_iterate_4d() {
        let mut cubes = Cubes::new();
        // .#.
        cubes.insert((0, 1, 0, 0));
        // ..#
        cubes.insert((1, 2, 0, 0));
        // ###
        cubes.insert((2, 0, 0, 0));
        cubes.insert((2, 1, 0, 0));
        cubes.insert((2, 2, 0, 0));

        for _ in 0..6 {
            cubes = iterate(cubes);
        }

        assert_eq!(cubes.len(), 848);
    }
}
