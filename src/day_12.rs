/// --- Day 12: Rain Risk ---
///
/// Your ferry made decent progress toward the island, but the storm came in
/// faster than anyone expected. The ferry needs to take evasive actions!
///
/// Unfortunately, the ship's navigation computer seems to be malfunctioning;
/// rather than giving a route directly to safety, it produced extremely
/// circuitous instructions. When the captain uses the PA system to ask if
/// anyone can help, you quickly volunteer.
///
/// The navigation instructions (your puzzle input) consists of a sequence of
/// single-character actions paired with integer input values. After staring at
/// them for a few minutes, you work out what they probably mean:
///
///   - Action N means to move north by the given value.
///   - Action S means to move south by the given value.
///   - Action E means to move east by the given value.
///   - Action W means to move west by the given value.
///   - Action L means to turn left the given number of degrees.
///   - Action R means to turn right the given number of degrees.
///   - Action F means to move forward by the given value in the direction the
///     ship is currently facing.
///
/// The ship starts by facing east. Only the L and R actions change the
/// direction the ship is facing. (That is, if the ship is facing east and the
/// next instruction is N10, the ship would move north 10 units, but would still
/// move east if the following action were F.)
///
/// For example:
///
/// F10
/// N3
/// F7
/// R90
/// F11
///
/// These instructions would be handled as follows:
///
///   - F10 would move the ship 10 units east (because the ship starts by facing
///     east) to east 10, north 0.
///   - N3 would move the ship 3 units north to east 10, north 3.
///   - F7 would move the ship another 7 units east (because the ship is still
///     facing east) to east 17, north 3.
///   - R90 would cause the ship to turn right by 90 degrees and face south; it
///     remains at east 17, north 3.
///   - F11 would move the ship 11 units south to east 17, south 8.
///
/// At the end of these instructions, the ship's Manhattan distance (sum of the
/// absolute values of its east/west position and its north/south position) from
/// its starting position is 17 + 8 = 25.
///
/// Figure out where the navigation instructions lead. What is the Manhattan
/// distance between that location and the ship's starting position?
///
/// --- Part Two ---
///
/// Before you can give the destination to the captain, you realize that the
/// actual action meanings were printed on the back of the instructions the
/// whole time.
///
/// Almost all of the actions indicate how to move a waypoint which is relative
/// to the ship's position:
///
///   - Action N means to move the waypoint north by the given value.
///   - Action S means to move the waypoint south by the given value.
///   - Action E means to move the waypoint east by the given value.
///   - Action W means to move the waypoint west by the given value.
///   - Action L means to rotate the waypoint around the ship left
///     (counter-clockwise) the given number of degrees.
///   - Action R means to rotate the waypoint around the ship right (clockwise)
///     the given number of degrees.
///   - Action F means to move forward to the waypoint a number of times equal
///     to the given value.
///
/// The waypoint starts 10 units east and 1 unit north relative to the ship. The
/// waypoint is relative to the ship; that is, if the ship moves, the waypoint
/// moves with it.
///
/// For example, using the same instructions as above:
///
///   - F10 moves the ship to the waypoint 10 times (a total of 100 units east
///     and 10 units north), leaving the ship at east 100, north 10. The
///     waypoint stays 10 units east and 1 unit north of the ship.
///   - N3 moves the waypoint 3 units north to 10 units east and 4 units north
///     of the ship. The ship remains at east 100, north 10.
///   - F7 moves the ship to the waypoint 7 times (a total of 70 units east and
///     28 units north), leaving the ship at east 170, north 38. The waypoint
///     stays 10 units east and 4 units north of the ship.
///   - R90 rotates the waypoint around the ship clockwise 90 degrees, moving it
///     to 4 units east and 10 units south of the ship. The ship remains at east
///     170, north 38.
///   - F11 moves the ship to the waypoint 11 times (a total of 44 units east
///     and 110 units south), leaving the ship at east 214, south 72. The
///     waypoint stays 4 units east and 10 units south of the ship.
///
/// After these operations, the ship's Manhattan distance from its starting
/// position is 214 + 72 = 286.
///
/// Figure out where the navigation instructions actually lead. What is the
/// Manhattan distance between that location and the ship's starting position?
use num;
use num_derive::{FromPrimitive, ToPrimitive};
use regex::Regex;

const INPUT: &str = include_str!("../input/day_12.txt");

pub fn run() {
    let instructions = parse_instructions(INPUT);

    // navigate the ship according to the instructions
    let mut ship = Ship::new();
    ship.execute_multiple(&instructions);

    println!(
        "The Manhattan distance between the ship and the starting position is: {}",
        ship.location.0 + ship.location.1
    );
}

#[derive(Debug, PartialEq)]
struct Ship {
    facing: Direction,
    location: Location,
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive, ToPrimitive)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Location = (i32, i32);

impl Ship {
    fn new() -> Ship {
        Ship {
            facing: Direction::East,
            location: (0, 0),
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction.action {
            Action::Left | Action::Right => self.turn(&instruction.action, instruction.value),
            Action::Forward => self.move_in_direction(self.facing, instruction.value),
            Action::North | Action::South | Action::East | Action::West => {
                self.move_in_direction(Ship::to_direction(&instruction.action), instruction.value)
            }
        };
    }

    fn to_direction(action: &Action) -> Direction {
        match action {
            Action::North => Direction::North,
            Action::South => Direction::South,
            Action::East => Direction::East,
            Action::West => Direction::West,
            _ => panic!("Action does not translate to a direction"),
        }
    }

    fn move_in_direction(&mut self, direction: Direction, value: i32) {
        self.location = Ship::move_location(self.location, direction, value);
    }

    fn move_location(location: Location, direction: Direction, value: i32) -> Location {
        let (x, y) = location;
        match direction {
            Direction::North => (x - value, y),
            Direction::South => (x + value, y),
            Direction::East => (x, y + value),
            Direction::West => (x, y - value),
        }
    }

    fn turn(&mut self, action: &Action, value: i32) {
        let old_facing = self.facing as i32;
        let turns = value / 90;
        self.facing = num::FromPrimitive::from_i32(
            match action {
                Action::Left => old_facing - turns,
                Action::Right => old_facing + turns,
                _ => panic!("Action is not a turn"),
            }
            .rem_euclid(4),
        )
        .unwrap();
    }

    fn execute_multiple(&mut self, instructions: &[Instruction]) {
        for instruction in instructions.iter() {
            self.execute(instruction);
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    action: Action,
    value: i32,
}

#[derive(Debug, PartialEq)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().filter_map(convert_to_instruction).collect()
}

fn convert_to_instruction(line: &str) -> Option<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([NSEWLRF])([0-9]+)").unwrap();
    }
    let groups = RE.captures(line).expect("No match found on line");
    let action = match groups.get(1).map(|a| a.as_str()) {
        Some("N") => Action::North,
        Some("S") => Action::South,
        Some("E") => Action::East,
        Some("W") => Action::West,
        Some("L") => Action::Left,
        Some("R") => Action::Right,
        Some("F") => Action::Forward,
        _ => return None,
    };
    let value = match groups.get(2).map(|m| m.as_str().parse()) {
        Some(Ok(value)) => value,
        _ => return None,
    };

    Some(Instruction { action, value })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        let input = "F10\nN3\nF7\nR90\nF11";

        let expected_instructions = vec![
            Instruction {
                action: Action::Forward,
                value: 10,
            },
            Instruction {
                action: Action::North,
                value: 3,
            },
            Instruction {
                action: Action::Forward,
                value: 7,
            },
            Instruction {
                action: Action::Right,
                value: 90,
            },
            Instruction {
                action: Action::Forward,
                value: 11,
            },
        ];

        assert_eq!(parse_instructions(input), expected_instructions);
    }

    #[test]
    fn test_ship_execute() {
        let instructions = vec![
            Instruction {
                action: Action::Forward,
                value: 10,
            },
            Instruction {
                action: Action::North,
                value: 3,
            },
            Instruction {
                action: Action::Forward,
                value: 7,
            },
            Instruction {
                action: Action::Right,
                value: 90,
            },
            Instruction {
                action: Action::Forward,
                value: 11,
            },
        ];

        let expected_ship = Ship {
            facing: Direction::South,
            location: (8, 17),
        };

        let mut ship = Ship::new();
        ship.execute_multiple(&instructions);
        assert_eq!(ship, expected_ship);
    }
}
