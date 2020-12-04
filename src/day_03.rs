/// --- Day 3: Toboggan Trajectory ---
///
/// With the toboggan login problems resolved, you set off toward the airport.
/// While travel by toboggan might be easy, it's certainly not safe: there's
/// very minimal steering and the area is covered in trees. You'll need to see
/// which angles will take you near the fewest trees.
///
/// Due to the local geology, trees in this area only grow on exact integer
/// coordinates in a grid. You make a map (your puzzle input) of the open
/// squares (.) and trees (#) you can see. For example:
///
/// ..##.......
/// #...#...#..
/// .#....#..#.
/// ..#.#...#.#
/// .#...##..#.
/// ..#.##.....
/// .#.#.#....#
/// .#........#
/// #.##...#...
/// #...##....#
/// .#..#...#.#
///
/// These aren't the only trees, though; due to something you read about once
/// involving arboreal genetics and biome stability, the same pattern repeats to
/// the right many times:
///
/// ..##.........##.........##.........##.........##.........##.......  --->
/// #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
/// .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
/// ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
/// .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
/// ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
/// .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
/// .#........#.#........#.#........#.#........#.#........#.#........#
/// #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
/// #...##....##...##....##...##....##...##....##...##....##...##....#
/// .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
///
/// You start on the open square (.) in the top-left corner and need to reach
/// the bottom (below the bottom-most row on your map).
///
/// The toboggan can only follow a few specific slopes (you opted for a cheaper
/// model that prefers rational numbers); start by counting all the trees you
/// would encounter for the slope right 3, down 1:
///
/// From your starting position at the top-left, check the position that is
/// right 3 and down 1. Then, check the position that is right 3 and down 1 from
/// there, and so on until you go past the bottom of the map.
///
/// The locations you'd check in the above example are marked here with O where
/// there was an open square and X where there was a tree:
///
/// ..##.........##.........##.........##.........##.........##.......  --->
/// #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
/// .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
/// ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
/// .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
/// ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
/// .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
/// .#........#.#........X.#........#.#........#.#........#.#........#
/// #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
/// #...##....##...##....##...#X....##...##....##...##....##...##....#
/// .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
///
/// In this example, traversing the map using this slope would cause you to
/// encounter 7 trees.
///
/// Starting at the top-left corner of your map and following a slope of right 3
/// and down 1, how many trees would you encounter?
///
/// --- Part Two ---
///
/// Time to check the rest of the slopes - you need to minimize the probability
/// of a sudden arboreal stop, after all.
///
/// Determine the number of trees you would encounter if, for each of the
/// following slopes, you start at the top-left corner and traverse the map all
/// the way to the bottom:
///
///     Right 1, down 1.
///     Right 3, down 1. (This is the slope you already checked.)
///     Right 5, down 1.
///     Right 7, down 1.
///     Right 1, down 2.
///
/// In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s)
/// respectively; multiplied together, these produce the answer 336.
///
/// What do you get if you multiply together the number of trees encountered on
/// each of the listed slopes?
use std::collections::HashSet;
use std::ops::AddAssign;

const INPUT: &str = include_str!("../input/day_03.txt");

pub fn run() {
    let map = parse_map(INPUT);

    let trees_hit = traverse(&map, 3, 1);
    println!(
        "Following a slope of right 3 and down 1, the amount of trees hit is: {}",
        trees_hit
    );

    let answer = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(right, down)| traverse(&map, right, down))
        .map(|trees_hit| trees_hit as i64)
        .product::<i64>();
    println!(
        "Multiplying the number of trees encountered for all slops gives: {}",
        answer
    );
}

fn traverse(map: &Map, right: i32, down: i32) -> i32 {
    /// traverse the map, reporting on how many trees were hit along the slope
    // start at 0, 0
    let mut location = Point { x: 0, y: 0 };

    let mut trees_hit = 0;
    while location.x < map.height as i32 {
        // do this until the full map is traversed
        location += Point { x: down, y: right };
        location.wrap_horizontal(map.width as i32);
        if map.trees.contains(&location) {
            trees_hit += 1;
        }
    }
    trees_hit
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    width: usize,
    height: usize,
    trees: Trees,
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Point {
    fn wrap_horizontal(&mut self, max_y: i32) {
        self.y %= max_y;
    }
}
type Trees = HashSet<Point>;

fn parse_map(input: &str) -> Map {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let trees = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(y, _)| Point {
                    x: x as i32,
                    y: y as i32,
                })
        })
        .collect();
    Map {
        width,
        height,
        trees,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map() {
        // ..##.......
        // #...#...#..
        // .#....#..#.
        // ..#.#...#.#
        let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#";

        let mut expected_trees = Trees::new();
        expected_trees.insert(Point { x: 0, y: 2 });
        expected_trees.insert(Point { x: 0, y: 3 });
        expected_trees.insert(Point { x: 1, y: 0 });
        expected_trees.insert(Point { x: 1, y: 4 });
        expected_trees.insert(Point { x: 1, y: 8 });
        expected_trees.insert(Point { x: 2, y: 1 });
        expected_trees.insert(Point { x: 2, y: 6 });
        expected_trees.insert(Point { x: 2, y: 9 });
        expected_trees.insert(Point { x: 3, y: 2 });
        expected_trees.insert(Point { x: 3, y: 4 });
        expected_trees.insert(Point { x: 3, y: 8 });
        expected_trees.insert(Point { x: 3, y: 10 });

        let expected = Map {
            width: 11,
            height: 4,
            trees: expected_trees,
        };

        assert_eq!(parse_map(input), expected);
    }

    #[test]
    fn test_traverse() {
        // ..##.......
        // #...#...#..
        // .#....#..#.
        // ..#.#...#.#
        // .#...##..#.
        // ..#.##.....
        // .#.#.#....#
        // .#........#
        // #.##...#...
        // #...##....#
        // .#..#...#.#
        let input = "\
            ..##.......\n\
            #...#...#..\n\
            .#....#..#.\n\
            ..#.#...#.#\n\
            .#...##..#.\n\
            ..#.##.....\n\
            .#.#.#....#\n\
            .#........#\n\
            #.##...#...\n\
            #...##....#\n\
            .#..#...#.#";
        // cheat a bit by using parse_map to build the map
        let map = parse_map(input);

        // traverse the trees with a slope of right 3, down 1 (the example from the description)
        // this should encounter 7 trees
        assert_eq!(traverse(&map, 3, 1), 7)
    }
}
