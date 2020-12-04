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
use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day_03.txt");

pub fn run() {
    println!("Not implemented yet");
    unimplemented!();
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
}
