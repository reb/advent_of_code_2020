/// --- Day 11: Seating System ---
///
/// Your plane lands with plenty of time to spare. The final leg of your journey
/// is a ferry that goes directly to the tropical island where you can finally
/// start your vacation. As you reach the waiting area to board the ferry, you
/// realize you're so early, nobody else has even arrived yet!
///
/// By modeling the process people use to choose (or abandon) their seat in the
/// waiting area, you're pretty sure you can predict the best place to sit. You
/// make a quick map of the seat layout (your puzzle input).
///
/// The seat layout fits neatly on a grid. Each position is either floor (.), an
/// empty seat (L), or an occupied seat (#). For example, the initial seat
/// layout might look like this:
///
/// L.LL.LL.LL
/// LLLLLLL.LL
/// L.L.L..L..
/// LLLL.LL.LL
/// L.LL.LL.LL
/// L.LLLLL.LL
/// ..L.L.....
/// LLLLLLLLLL
/// L.LLLLLL.L
/// L.LLLLL.LL
///
/// Now, you just need to model the people who will be arriving shortly.
/// Fortunately, people are entirely predictable and always follow a simple set
/// of rules. All decisions are based on the number of occupied seats adjacent
/// to a given seat (one of the eight positions immediately up, down, left,
/// right, or diagonal from the seat). The following rules are applied to every
/// seat simultaneously:
///
///   - If a seat is empty (L) and there are no occupied seats adjacent to it,
///     the seat becomes occupied.
///   - If a seat is occupied (#) and four or more seats adjacent to it are also
///     occupied, the seat becomes empty.
///   - Otherwise, the seat's state does not change.
///
/// Floor (.) never changes; seats don't move, and nobody sits on the floor.
///
/// After one round of these rules, every seat in the example layout becomes
/// occupied:
///
/// #.##.##.##
/// #######.##
/// #.#.#..#..
/// ####.##.##
/// #.##.##.##
/// #.#####.##
/// ..#.#.....
/// ##########
/// #.######.#
/// #.#####.##
///
/// After a second round, the seats with four or more occupied adjacent seats
/// become empty again:
///
/// #.LL.L#.##
/// #LLLLLL.L#
/// L.L.L..L..
/// #LLL.LL.L#
/// #.LL.LL.LL
/// #.LLLL#.##
/// ..L.L.....
/// #LLLLLLLL#
/// #.LLLLLL.L
/// #.#LLLL.##
///
/// This process continues for three more rounds:
///
/// #.##.L#.##
/// #L###LL.L#
/// L.#.#..#..
/// #L##.##.L#
/// #.##.LL.LL
/// #.###L#.##
/// ..#.#.....
/// #L######L#
/// #.LL###L.L
/// #.#L###.##
///
/// #.#L.L#.##
/// #LLL#LL.L#
/// L.L.L..#..
/// #LLL.##.L#
/// #.LL.LL.LL
/// #.LL#L#.##
/// ..L.L.....
/// #L#LLLL#L#
/// #.LLLLLL.L
/// #.#L#L#.##
///
/// #.#L.L#.##
/// #LLL#LL.L#
/// L.#.L..#..
/// #L##.##.L#
/// #.#L.LL.LL
/// #.#L#L#.##
/// ..L.L.....
/// #L#L##L#L#
/// #.LLLLLL.L
/// #.#L#L#.##
///
/// At this point, something interesting happens: the chaos stabilizes and
/// further applications of these rules cause no seats to change state! Once
/// people stop moving around, you count 37 occupied seats.
///
/// Simulate your seating area by applying the seating rules repeatedly until no
/// seats change state. How many seats end up occupied?
///
/// --- Part Two ---
///
/// As soon as people start to arrive, you realize your mistake. People don't
/// just care about adjacent seats - they care about the first seat they can see
/// in each of those eight directions!
///
/// Now, instead of considering just the eight immediately adjacent seats,
/// consider the first seat in each of those eight directions. For example, the
/// empty seat below would see eight occupied seats:
///
/// .......#.
/// ...#.....
/// .#.......
/// .........
/// ..#L....#
/// ....#....
/// .........
/// #........
/// ...#.....
///
/// The leftmost empty seat below would only see one empty seat, but cannot see
/// any of the occupied ones:
///
/// .............
/// .L.L.#.#.#.#.
/// .............
///
/// The empty seat below would see no occupied seats:
///
/// .##.##.
/// #.#.#.#
/// ##...##
/// ...L...
/// ##...##
/// #.#.#.#
/// .##.##.
///
/// Also, people seem to be more tolerant than you expected: it now takes five
/// or more visible occupied seats for an occupied seat to become empty (rather
/// than four or more from the previous rules). The other rules still apply:
/// empty seats that see no occupied seats become occupied, seats matching no
/// rule don't change, and floor never changes.
///
/// Given the same starting layout as above, these new rules cause the seating
/// area to shift around as follows:
///
/// L.LL.LL.LL
/// LLLLLLL.LL
/// L.L.L..L..
/// LLLL.LL.LL
/// L.LL.LL.LL
/// L.LLLLL.LL
/// ..L.L.....
/// LLLLLLLLLL
/// L.LLLLLL.L
/// L.LLLLL.LL
///
/// #.##.##.##
/// #######.##
/// #.#.#..#..
/// ####.##.##
/// #.##.##.##
/// #.#####.##
/// ..#.#.....
/// ##########
/// #.######.#
/// #.#####.##
///
/// #.LL.LL.L#
/// #LLLLLL.LL
/// L.L.L..L..
/// LLLL.LL.LL
/// L.LL.LL.LL
/// L.LLLLL.LL
/// ..L.L.....
/// LLLLLLLLL#
/// #.LLLLLL.L
/// #.LLLLL.L#
///
/// #.L#.##.L#
/// #L#####.LL
/// L.#.#..#..
/// ##L#.##.##
/// #.##.#L.##
/// #.#####.#L
/// ..#.#.....
/// LLL####LL#
/// #.L#####.L
/// #.L####.L#
///
/// #.L#.L#.L#
/// #LLLLLL.LL
/// L.L.L..#..
/// ##LL.LL.L#
/// L.LL.LL.L#
/// #.LLLLL.LL
/// ..L.L.....
/// LLLLLLLLL#
/// #.LLLLL#.L
/// #.L#LL#.L#
///
/// #.L#.L#.L#
/// #LLLLLL.LL
/// L.L.L..#..
/// ##L#.#L.L#
/// L.L#.#L.L#
/// #.L####.LL
/// ..#.#.....
/// LLL###LLL#
/// #.LLLLL#.L
/// #.L#LL#.L#
///
/// #.L#.L#.L#
/// #LLLLLL.LL
/// L.L.L..#..
/// ##L#.#L.L#
/// L.L#.LL.L#
/// #.LLLL#.LL
/// ..#.L.....
/// LLL###LLL#
/// #.LLLLL#.L
/// #.L#LL#.L#
///
/// Again, at this point, people stop shifting around and the seating area
/// reaches equilibrium. Once this occurs, you count 26 occupied seats.
///
/// Given the new visibility method and the rule change for occupied seats
/// becoming empty, once equilibrium is reached, how many seats end up occupied?
use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_11.txt");

pub fn run() {
    let seats = load_seats(INPUT);

    let mut seating = Seating::new(seats.clone(), NeighbourMode::Direct);

    // run rounds until a stable solution forms
    while seating.next_round() {
        seating.print();
    }

    // count the number of occupied seats
    let occupied_seats = seating
        .seats
        .values()
        .filter(|seat| seat == &&Seat::Occupied)
        .count();
    println!(
        "After no more seats change the amount of seats that are occupied is: {}",
        occupied_seats
    );
}

#[derive(Debug, PartialEq)]
struct Seating {
    seats: HashMap<Location, Seat>,
    bounds: Bounds,
    neighbour_mode: NeighbourMode,
}

type Location = (i32, i32);
#[derive(Debug, PartialEq, Clone)]
enum Seat {
    Empty,
    Occupied,
}
type Seats = HashMap<Location, Seat>;

#[derive(Debug, PartialEq)]
struct Bounds {
    x: MinMax,
    y: MinMax,
}

#[derive(Debug, PartialEq)]
struct MinMax {
    min: i32,
    max: i32,
}

#[derive(Debug, PartialEq)]
enum NeighbourMode {
    Direct,
    Visible,
}

impl Seating {
    fn new(seats: Seats, neighbour_mode: NeighbourMode) -> Seating {
        let bounds = Seating::get_bounds(&seats);
        Seating {
            seats,
            bounds,
            neighbour_mode,
        }
    }

    fn get_bounds(seats: &Seats) -> Bounds {
        let (&x_min, &x_max) = seats.keys().map(|(x, _)| x).minmax().into_option().unwrap();
        let (&y_min, &y_max) = seats.keys().map(|(_, y)| y).minmax().into_option().unwrap();
        Bounds {
            x: MinMax {
                min: x_min,
                max: x_max,
            },
            y: MinMax {
                min: y_min,
                max: y_max,
            },
        }
    }

    fn print(&self) {
        // clear the screen
        print!("\x1B[2J\x1B[1;1H");

        for x in self.bounds.x.min..=self.bounds.x.max {
            for y in self.bounds.y.min..=self.bounds.y.max {
                match self.seats.get(&(x, y)) {
                    Some(Seat::Empty) => print!("L"),
                    Some(Seat::Occupied) => print!("#"),
                    None => print!("."),
                }
            }
            print!("\n");
        }
        print!("\n\n");
    }

    fn direct_neighbours(&self, &(x, y): &Location) -> Vec<Location> {
        vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
    }

    fn visible_neighbours(&self, &(x, y): &Location) -> Vec<Location> {
        unimplemented!();
    }

    fn get_neighbour_counts(&self) -> HashMap<Location, usize> {
        // count all the neighbours for each seating location
        self.seats
            .iter()
            .filter_map(|(location, seat)| match seat {
                Seat::Occupied => Some(location),
                Seat::Empty => None,
            })
            .flat_map(|location| match &self.neighbour_mode {
                NeighbourMode::Direct => self.direct_neighbours(location),
                NeighbourMode::Visible => self.visible_neighbours(location),
            })
            .filter(|location| self.seats.contains_key(location))
            .fold(HashMap::new(), |mut map, neighbour| {
                *map.entry(neighbour).or_insert(0) += 1;
                map
            })
    }

    fn next_round(&mut self) -> bool {
        // For Direct NeighbourMode:
        // - If a seat is empty (L) and there are no occupied seats adjacent to it,
        //   the seat becomes occupied.
        // - If a seat is occupied (#) and four or more seats adjacent to it are also
        //   occupied, the seat becomes empty.
        // - Otherwise, the seat's state does not change.
        //
        // For Visible NeighbourMode:
        // Now, instead of considering just the eight immediately adjacent seats,
        // consider the first seat in each of those eight directions.
        // Also, people seem to be more tolerant than you expected: it now takes five
        // or more visible occupied seats for an occupied seat to become empty (rather
        // than four or more from the previous rules). The other rules still apply:
        // empty seats that see no occupied seats become occupied, seats matching no
        // rule don't change, and floor never changes.
        let neighbour_counts = self.get_neighbour_counts();
        let new_seats = self
            .seats
            .iter()
            .map(|(&location, seat)| {
                let new_seat = match seat {
                    &Seat::Empty => match neighbour_counts.get(&location) {
                        Some(&n) if n > 0 => Seat::Empty,
                        _ => Seat::Occupied,
                    },
                    &Seat::Occupied => {
                        match (neighbour_counts.get(&location), &self.neighbour_mode) {
                            (Some(&n), NeighbourMode::Direct) if n >= 4 => Seat::Empty,
                            (Some(&n), NeighbourMode::Visible) if n >= 5 => Seat::Empty,
                            _ => Seat::Occupied,
                        }
                    }
                };
                (location, new_seat)
            })
            .collect();

        if new_seats == self.seats {
            return false;
        }
        self.seats = new_seats;
        return true;
    }
}

fn load_seats(input: &str) -> Seats {
    input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars().enumerate().filter_map(move |(y, c)| match c {
                'L' => Some(((x as i32, y as i32), Seat::Empty)),
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
    fn test_parse_seats() {
        let input = "\
            L.LL.LL.LL\n\
            LLLLLLL.LL\n\
            L.L.L..L..\n\
            LLLL.LL.LL\n\
            L.LL.LL.LL\n\
            L.LLLLL.LL\n\
            ..L.L.....\n\
            LLLLLLLLLL\n\
            L.LLLLLL.L\n\
            L.LLLLL.LL";

        let mut expected_seats = HashMap::new();
        // L.LL.LL.LL
        expected_seats.insert((0, 0), Seat::Empty);
        expected_seats.insert((0, 2), Seat::Empty);
        expected_seats.insert((0, 3), Seat::Empty);
        expected_seats.insert((0, 5), Seat::Empty);
        expected_seats.insert((0, 6), Seat::Empty);
        expected_seats.insert((0, 8), Seat::Empty);
        expected_seats.insert((0, 9), Seat::Empty);
        // LLLLLLL.LL
        expected_seats.insert((1, 0), Seat::Empty);
        expected_seats.insert((1, 1), Seat::Empty);
        expected_seats.insert((1, 2), Seat::Empty);
        expected_seats.insert((1, 3), Seat::Empty);
        expected_seats.insert((1, 4), Seat::Empty);
        expected_seats.insert((1, 5), Seat::Empty);
        expected_seats.insert((1, 6), Seat::Empty);
        expected_seats.insert((1, 8), Seat::Empty);
        expected_seats.insert((1, 9), Seat::Empty);
        // L.L.L..L..
        expected_seats.insert((2, 0), Seat::Empty);
        expected_seats.insert((2, 2), Seat::Empty);
        expected_seats.insert((2, 4), Seat::Empty);
        expected_seats.insert((2, 7), Seat::Empty);
        // LLLL.LL.LL
        expected_seats.insert((3, 0), Seat::Empty);
        expected_seats.insert((3, 1), Seat::Empty);
        expected_seats.insert((3, 2), Seat::Empty);
        expected_seats.insert((3, 3), Seat::Empty);
        expected_seats.insert((3, 5), Seat::Empty);
        expected_seats.insert((3, 6), Seat::Empty);
        expected_seats.insert((3, 8), Seat::Empty);
        expected_seats.insert((3, 9), Seat::Empty);
        // L.LL.LL.LL
        expected_seats.insert((4, 0), Seat::Empty);
        expected_seats.insert((4, 2), Seat::Empty);
        expected_seats.insert((4, 3), Seat::Empty);
        expected_seats.insert((4, 5), Seat::Empty);
        expected_seats.insert((4, 6), Seat::Empty);
        expected_seats.insert((4, 8), Seat::Empty);
        expected_seats.insert((4, 9), Seat::Empty);
        // L.LLLLL.LL
        expected_seats.insert((5, 0), Seat::Empty);
        expected_seats.insert((5, 2), Seat::Empty);
        expected_seats.insert((5, 3), Seat::Empty);
        expected_seats.insert((5, 4), Seat::Empty);
        expected_seats.insert((5, 5), Seat::Empty);
        expected_seats.insert((5, 6), Seat::Empty);
        expected_seats.insert((5, 8), Seat::Empty);
        expected_seats.insert((5, 9), Seat::Empty);
        // ..L.L.....
        expected_seats.insert((6, 2), Seat::Empty);
        expected_seats.insert((6, 4), Seat::Empty);
        // LLLLLLLLLL
        expected_seats.insert((7, 0), Seat::Empty);
        expected_seats.insert((7, 1), Seat::Empty);
        expected_seats.insert((7, 2), Seat::Empty);
        expected_seats.insert((7, 3), Seat::Empty);
        expected_seats.insert((7, 4), Seat::Empty);
        expected_seats.insert((7, 5), Seat::Empty);
        expected_seats.insert((7, 6), Seat::Empty);
        expected_seats.insert((7, 7), Seat::Empty);
        expected_seats.insert((7, 8), Seat::Empty);
        expected_seats.insert((7, 9), Seat::Empty);
        // L.LLLLLL.L
        expected_seats.insert((8, 0), Seat::Empty);
        expected_seats.insert((8, 2), Seat::Empty);
        expected_seats.insert((8, 3), Seat::Empty);
        expected_seats.insert((8, 4), Seat::Empty);
        expected_seats.insert((8, 5), Seat::Empty);
        expected_seats.insert((8, 6), Seat::Empty);
        expected_seats.insert((8, 7), Seat::Empty);
        expected_seats.insert((8, 9), Seat::Empty);
        // L.LLLLL.LL
        expected_seats.insert((9, 0), Seat::Empty);
        expected_seats.insert((9, 2), Seat::Empty);
        expected_seats.insert((9, 3), Seat::Empty);
        expected_seats.insert((9, 4), Seat::Empty);
        expected_seats.insert((9, 5), Seat::Empty);
        expected_seats.insert((9, 6), Seat::Empty);
        expected_seats.insert((9, 8), Seat::Empty);
        expected_seats.insert((9, 9), Seat::Empty);

        assert_eq!(load_seats(input), expected_seats);
    }

    #[test]
    fn test_next_round_direct_neighbours() {
        let mut seats = HashMap::new();
        // L.LL.LL.LL
        seats.insert((0, 0), Seat::Empty);
        seats.insert((0, 2), Seat::Empty);
        seats.insert((0, 3), Seat::Empty);
        seats.insert((0, 5), Seat::Empty);
        seats.insert((0, 6), Seat::Empty);
        seats.insert((0, 8), Seat::Empty);
        seats.insert((0, 9), Seat::Empty);
        // LLLLLLL.LL
        seats.insert((1, 0), Seat::Empty);
        seats.insert((1, 1), Seat::Empty);
        seats.insert((1, 2), Seat::Empty);
        seats.insert((1, 3), Seat::Empty);
        seats.insert((1, 4), Seat::Empty);
        seats.insert((1, 5), Seat::Empty);
        seats.insert((1, 6), Seat::Empty);
        seats.insert((1, 8), Seat::Empty);
        seats.insert((1, 9), Seat::Empty);
        // L.L.L..L..
        seats.insert((2, 0), Seat::Empty);
        seats.insert((2, 2), Seat::Empty);
        seats.insert((2, 4), Seat::Empty);
        seats.insert((2, 7), Seat::Empty);
        // LLLL.LL.LL
        seats.insert((3, 0), Seat::Empty);
        seats.insert((3, 1), Seat::Empty);
        seats.insert((3, 2), Seat::Empty);
        seats.insert((3, 3), Seat::Empty);
        seats.insert((3, 5), Seat::Empty);
        seats.insert((3, 6), Seat::Empty);
        seats.insert((3, 8), Seat::Empty);
        seats.insert((3, 9), Seat::Empty);
        // L.LL.LL.LL
        seats.insert((4, 0), Seat::Empty);
        seats.insert((4, 2), Seat::Empty);
        seats.insert((4, 3), Seat::Empty);
        seats.insert((4, 5), Seat::Empty);
        seats.insert((4, 6), Seat::Empty);
        seats.insert((4, 8), Seat::Empty);
        seats.insert((4, 9), Seat::Empty);
        // L.LLLLL.LL
        seats.insert((5, 0), Seat::Empty);
        seats.insert((5, 2), Seat::Empty);
        seats.insert((5, 3), Seat::Empty);
        seats.insert((5, 4), Seat::Empty);
        seats.insert((5, 5), Seat::Empty);
        seats.insert((5, 6), Seat::Empty);
        seats.insert((5, 8), Seat::Empty);
        seats.insert((5, 9), Seat::Empty);
        // ..L.L.....
        seats.insert((6, 2), Seat::Empty);
        seats.insert((6, 4), Seat::Empty);
        // LLLLLLLLLL
        seats.insert((7, 0), Seat::Empty);
        seats.insert((7, 1), Seat::Empty);
        seats.insert((7, 2), Seat::Empty);
        seats.insert((7, 3), Seat::Empty);
        seats.insert((7, 4), Seat::Empty);
        seats.insert((7, 5), Seat::Empty);
        seats.insert((7, 6), Seat::Empty);
        seats.insert((7, 7), Seat::Empty);
        seats.insert((7, 8), Seat::Empty);
        seats.insert((7, 9), Seat::Empty);
        // L.LLLLLL.L
        seats.insert((8, 0), Seat::Empty);
        seats.insert((8, 2), Seat::Empty);
        seats.insert((8, 3), Seat::Empty);
        seats.insert((8, 4), Seat::Empty);
        seats.insert((8, 5), Seat::Empty);
        seats.insert((8, 6), Seat::Empty);
        seats.insert((8, 7), Seat::Empty);
        seats.insert((8, 9), Seat::Empty);
        // L.LLLLL.LL
        seats.insert((9, 0), Seat::Empty);
        seats.insert((9, 2), Seat::Empty);
        seats.insert((9, 3), Seat::Empty);
        seats.insert((9, 4), Seat::Empty);
        seats.insert((9, 5), Seat::Empty);
        seats.insert((9, 6), Seat::Empty);
        seats.insert((9, 8), Seat::Empty);
        seats.insert((9, 9), Seat::Empty);

        let mut stabilized = HashMap::new();
        // #.#L.L#.##
        stabilized.insert((0, 0), Seat::Occupied);
        stabilized.insert((0, 2), Seat::Occupied);
        stabilized.insert((0, 3), Seat::Empty);
        stabilized.insert((0, 5), Seat::Empty);
        stabilized.insert((0, 6), Seat::Occupied);
        stabilized.insert((0, 8), Seat::Occupied);
        stabilized.insert((0, 9), Seat::Occupied);
        // #LLL#LL.L#
        stabilized.insert((1, 0), Seat::Occupied);
        stabilized.insert((1, 1), Seat::Empty);
        stabilized.insert((1, 2), Seat::Empty);
        stabilized.insert((1, 3), Seat::Empty);
        stabilized.insert((1, 4), Seat::Occupied);
        stabilized.insert((1, 5), Seat::Empty);
        stabilized.insert((1, 6), Seat::Empty);
        stabilized.insert((1, 8), Seat::Empty);
        stabilized.insert((1, 9), Seat::Occupied);
        // L.#.L..#..
        stabilized.insert((2, 0), Seat::Empty);
        stabilized.insert((2, 2), Seat::Occupied);
        stabilized.insert((2, 4), Seat::Empty);
        stabilized.insert((2, 7), Seat::Occupied);
        // #L##.##.L#
        stabilized.insert((3, 0), Seat::Occupied);
        stabilized.insert((3, 1), Seat::Empty);
        stabilized.insert((3, 2), Seat::Occupied);
        stabilized.insert((3, 3), Seat::Occupied);
        stabilized.insert((3, 5), Seat::Occupied);
        stabilized.insert((3, 6), Seat::Occupied);
        stabilized.insert((3, 8), Seat::Empty);
        stabilized.insert((3, 9), Seat::Occupied);
        // #.#L.LL.LL
        stabilized.insert((4, 0), Seat::Occupied);
        stabilized.insert((4, 2), Seat::Occupied);
        stabilized.insert((4, 3), Seat::Empty);
        stabilized.insert((4, 5), Seat::Empty);
        stabilized.insert((4, 6), Seat::Empty);
        stabilized.insert((4, 8), Seat::Empty);
        stabilized.insert((4, 9), Seat::Empty);
        // #.#L#L#.##
        stabilized.insert((5, 0), Seat::Occupied);
        stabilized.insert((5, 2), Seat::Occupied);
        stabilized.insert((5, 3), Seat::Empty);
        stabilized.insert((5, 4), Seat::Occupied);
        stabilized.insert((5, 5), Seat::Empty);
        stabilized.insert((5, 6), Seat::Occupied);
        stabilized.insert((5, 8), Seat::Occupied);
        stabilized.insert((5, 9), Seat::Occupied);
        // ..L.L.....
        stabilized.insert((6, 2), Seat::Empty);
        stabilized.insert((6, 4), Seat::Empty);
        // #L#L##L#L#
        stabilized.insert((7, 0), Seat::Occupied);
        stabilized.insert((7, 1), Seat::Empty);
        stabilized.insert((7, 2), Seat::Occupied);
        stabilized.insert((7, 3), Seat::Empty);
        stabilized.insert((7, 4), Seat::Occupied);
        stabilized.insert((7, 5), Seat::Occupied);
        stabilized.insert((7, 6), Seat::Empty);
        stabilized.insert((7, 7), Seat::Occupied);
        stabilized.insert((7, 8), Seat::Empty);
        stabilized.insert((7, 9), Seat::Occupied);
        // #.LLLLLL.L
        stabilized.insert((8, 0), Seat::Occupied);
        stabilized.insert((8, 2), Seat::Empty);
        stabilized.insert((8, 3), Seat::Empty);
        stabilized.insert((8, 4), Seat::Empty);
        stabilized.insert((8, 5), Seat::Empty);
        stabilized.insert((8, 6), Seat::Empty);
        stabilized.insert((8, 7), Seat::Empty);
        stabilized.insert((8, 9), Seat::Empty);
        // #.#L#L#.##
        stabilized.insert((9, 0), Seat::Occupied);
        stabilized.insert((9, 2), Seat::Occupied);
        stabilized.insert((9, 3), Seat::Empty);
        stabilized.insert((9, 4), Seat::Occupied);
        stabilized.insert((9, 5), Seat::Empty);
        stabilized.insert((9, 6), Seat::Occupied);
        stabilized.insert((9, 8), Seat::Occupied);
        stabilized.insert((9, 9), Seat::Occupied);

        let mut seating = Seating::new(seats, NeighbourMode::Direct);
        for _ in 0..5 {
            // first 5 rounds should change the seating
            assert!(seating.next_round());
        }
        // 6th round should not
        assert!(!seating.next_round());
        // and seating should be the stabilized configuration
        assert_eq!(seating.seats, stabilized);
    }
}
