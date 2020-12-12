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
use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_11.txt");

pub fn run() {
    let start_seating = load_seats(INPUT);

    let mut seating = start_seating.clone();

    // run rounds until a stable solution forms
    loop {
        print_seating(&seating);
        let new_seating = next_round(&seating);
        if new_seating == seating {
            break;
        }
        seating = new_seating;
    }

    // count the number of occupied seats
    let occupied_seats = seating
        .values()
        .filter(|seat| seat == &&Seat::Occupied)
        .count();
    println!(
        "After no more seats change the amount of seats that are occupied is: {}",
        occupied_seats
    );
}

type Location = (i32, i32);
type Seating = HashMap<Location, Seat>;

#[derive(Debug, PartialEq, Clone)]
enum Seat {
    Empty,
    Occupied,
}

fn print_seating(seating: &Seating) {
    let (&x_min, &x_max) = seating
        .keys()
        .map(|(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (&y_min, &y_max) = seating
        .keys()
        .map(|(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    // clear the screen
    print!("\x1B[2J\x1B[1;1H");

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            match seating.get(&(x, y)) {
                Some(Seat::Empty) => print!("L"),
                Some(Seat::Occupied) => print!("#"),
                None => print!("."),
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn neighbours(&(x, y): &Location) -> Vec<Location> {
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

fn get_neighbour_counts(seating: &Seating) -> HashMap<Location, usize> {
    // count all the neighbours for each seating location
    seating
        .iter()
        .filter_map(|(location, seat)| match seat {
            Seat::Occupied => Some(location),
            Seat::Empty => None,
        })
        .flat_map(neighbours)
        .filter(|location| seating.contains_key(location))
        .fold(HashMap::new(), |mut map, neighbour| {
            *map.entry(neighbour).or_insert(0) += 1;
            map
        })
}

fn next_round(seating: &Seating) -> Seating {
    // - If a seat is empty (L) and there are no occupied seats adjacent to it,
    //   the seat becomes occupied.
    // - If a seat is occupied (#) and four or more seats adjacent to it are also
    //   occupied, the seat becomes empty.
    // - Otherwise, the seat's state does not change.
    let neighbour_counts = get_neighbour_counts(&seating);
    seating
        .iter()
        .map(|(&location, seat)| {
            let new_seat = match seat {
                &Seat::Empty => match neighbour_counts.get(&location) {
                    Some(&n) if n > 0 => Seat::Empty,
                    _ => Seat::Occupied,
                },
                &Seat::Occupied => match neighbour_counts.get(&location) {
                    Some(&n) if n >= 4 => Seat::Empty,
                    _ => Seat::Occupied,
                },
            };
            (location, new_seat)
        })
        .collect()
}

fn load_seats(input: &str) -> Seating {
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
    fn test_next_round() {
        let mut seating = HashMap::new();
        // L.LL.LL.LL
        seating.insert((0, 0), Seat::Empty);
        seating.insert((0, 2), Seat::Empty);
        seating.insert((0, 3), Seat::Empty);
        seating.insert((0, 5), Seat::Empty);
        seating.insert((0, 6), Seat::Empty);
        seating.insert((0, 8), Seat::Empty);
        seating.insert((0, 9), Seat::Empty);
        // LLLLLLL.LL
        seating.insert((1, 0), Seat::Empty);
        seating.insert((1, 1), Seat::Empty);
        seating.insert((1, 2), Seat::Empty);
        seating.insert((1, 3), Seat::Empty);
        seating.insert((1, 4), Seat::Empty);
        seating.insert((1, 5), Seat::Empty);
        seating.insert((1, 6), Seat::Empty);
        seating.insert((1, 8), Seat::Empty);
        seating.insert((1, 9), Seat::Empty);
        // L.L.L..L..
        seating.insert((2, 0), Seat::Empty);
        seating.insert((2, 2), Seat::Empty);
        seating.insert((2, 4), Seat::Empty);
        seating.insert((2, 7), Seat::Empty);
        // LLLL.LL.LL
        seating.insert((3, 0), Seat::Empty);
        seating.insert((3, 1), Seat::Empty);
        seating.insert((3, 2), Seat::Empty);
        seating.insert((3, 3), Seat::Empty);
        seating.insert((3, 5), Seat::Empty);
        seating.insert((3, 6), Seat::Empty);
        seating.insert((3, 8), Seat::Empty);
        seating.insert((3, 9), Seat::Empty);
        // L.LL.LL.LL
        seating.insert((4, 0), Seat::Empty);
        seating.insert((4, 2), Seat::Empty);
        seating.insert((4, 3), Seat::Empty);
        seating.insert((4, 5), Seat::Empty);
        seating.insert((4, 6), Seat::Empty);
        seating.insert((4, 8), Seat::Empty);
        seating.insert((4, 9), Seat::Empty);
        // L.LLLLL.LL
        seating.insert((5, 0), Seat::Empty);
        seating.insert((5, 2), Seat::Empty);
        seating.insert((5, 3), Seat::Empty);
        seating.insert((5, 4), Seat::Empty);
        seating.insert((5, 5), Seat::Empty);
        seating.insert((5, 6), Seat::Empty);
        seating.insert((5, 8), Seat::Empty);
        seating.insert((5, 9), Seat::Empty);
        // ..L.L.....
        seating.insert((6, 2), Seat::Empty);
        seating.insert((6, 4), Seat::Empty);
        // LLLLLLLLLL
        seating.insert((7, 0), Seat::Empty);
        seating.insert((7, 1), Seat::Empty);
        seating.insert((7, 2), Seat::Empty);
        seating.insert((7, 3), Seat::Empty);
        seating.insert((7, 4), Seat::Empty);
        seating.insert((7, 5), Seat::Empty);
        seating.insert((7, 6), Seat::Empty);
        seating.insert((7, 7), Seat::Empty);
        seating.insert((7, 8), Seat::Empty);
        seating.insert((7, 9), Seat::Empty);
        // L.LLLLLL.L
        seating.insert((8, 0), Seat::Empty);
        seating.insert((8, 2), Seat::Empty);
        seating.insert((8, 3), Seat::Empty);
        seating.insert((8, 4), Seat::Empty);
        seating.insert((8, 5), Seat::Empty);
        seating.insert((8, 6), Seat::Empty);
        seating.insert((8, 7), Seat::Empty);
        seating.insert((8, 9), Seat::Empty);
        // L.LLLLL.LL
        seating.insert((9, 0), Seat::Empty);
        seating.insert((9, 2), Seat::Empty);
        seating.insert((9, 3), Seat::Empty);
        seating.insert((9, 4), Seat::Empty);
        seating.insert((9, 5), Seat::Empty);
        seating.insert((9, 6), Seat::Empty);
        seating.insert((9, 8), Seat::Empty);
        seating.insert((9, 9), Seat::Empty);

        let mut after_4_rounds = HashMap::new();
        // #.#L.L#.##
        after_4_rounds.insert((0, 0), Seat::Occupied);
        after_4_rounds.insert((0, 2), Seat::Occupied);
        after_4_rounds.insert((0, 3), Seat::Empty);
        after_4_rounds.insert((0, 5), Seat::Empty);
        after_4_rounds.insert((0, 6), Seat::Occupied);
        after_4_rounds.insert((0, 8), Seat::Occupied);
        after_4_rounds.insert((0, 9), Seat::Occupied);
        // #LLL#LL.L#
        after_4_rounds.insert((1, 0), Seat::Occupied);
        after_4_rounds.insert((1, 1), Seat::Empty);
        after_4_rounds.insert((1, 2), Seat::Empty);
        after_4_rounds.insert((1, 3), Seat::Empty);
        after_4_rounds.insert((1, 4), Seat::Occupied);
        after_4_rounds.insert((1, 5), Seat::Empty);
        after_4_rounds.insert((1, 6), Seat::Empty);
        after_4_rounds.insert((1, 8), Seat::Empty);
        after_4_rounds.insert((1, 9), Seat::Occupied);
        // L.L.L..#..
        after_4_rounds.insert((2, 0), Seat::Empty);
        after_4_rounds.insert((2, 2), Seat::Empty);
        after_4_rounds.insert((2, 4), Seat::Empty);
        after_4_rounds.insert((2, 7), Seat::Occupied);
        // #LLL.##.L#
        after_4_rounds.insert((3, 0), Seat::Occupied);
        after_4_rounds.insert((3, 1), Seat::Empty);
        after_4_rounds.insert((3, 2), Seat::Empty);
        after_4_rounds.insert((3, 3), Seat::Empty);
        after_4_rounds.insert((3, 5), Seat::Occupied);
        after_4_rounds.insert((3, 6), Seat::Occupied);
        after_4_rounds.insert((3, 8), Seat::Empty);
        after_4_rounds.insert((3, 9), Seat::Occupied);
        // #.LL.LL.LL
        after_4_rounds.insert((4, 0), Seat::Occupied);
        after_4_rounds.insert((4, 2), Seat::Empty);
        after_4_rounds.insert((4, 3), Seat::Empty);
        after_4_rounds.insert((4, 5), Seat::Empty);
        after_4_rounds.insert((4, 6), Seat::Empty);
        after_4_rounds.insert((4, 8), Seat::Empty);
        after_4_rounds.insert((4, 9), Seat::Empty);
        // #.LL#L#.##
        after_4_rounds.insert((5, 0), Seat::Occupied);
        after_4_rounds.insert((5, 2), Seat::Empty);
        after_4_rounds.insert((5, 3), Seat::Empty);
        after_4_rounds.insert((5, 4), Seat::Occupied);
        after_4_rounds.insert((5, 5), Seat::Empty);
        after_4_rounds.insert((5, 6), Seat::Occupied);
        after_4_rounds.insert((5, 8), Seat::Occupied);
        after_4_rounds.insert((5, 9), Seat::Occupied);
        // ..L.L.....
        after_4_rounds.insert((6, 2), Seat::Empty);
        after_4_rounds.insert((6, 4), Seat::Empty);
        // #L#LLLL#L#
        after_4_rounds.insert((7, 0), Seat::Occupied);
        after_4_rounds.insert((7, 1), Seat::Empty);
        after_4_rounds.insert((7, 2), Seat::Occupied);
        after_4_rounds.insert((7, 3), Seat::Empty);
        after_4_rounds.insert((7, 4), Seat::Empty);
        after_4_rounds.insert((7, 5), Seat::Empty);
        after_4_rounds.insert((7, 6), Seat::Empty);
        after_4_rounds.insert((7, 7), Seat::Occupied);
        after_4_rounds.insert((7, 8), Seat::Empty);
        after_4_rounds.insert((7, 9), Seat::Occupied);
        // #.LLLLLL.L
        after_4_rounds.insert((8, 0), Seat::Occupied);
        after_4_rounds.insert((8, 2), Seat::Empty);
        after_4_rounds.insert((8, 3), Seat::Empty);
        after_4_rounds.insert((8, 4), Seat::Empty);
        after_4_rounds.insert((8, 5), Seat::Empty);
        after_4_rounds.insert((8, 6), Seat::Empty);
        after_4_rounds.insert((8, 7), Seat::Empty);
        after_4_rounds.insert((8, 9), Seat::Empty);
        // #.#L#L#.##
        after_4_rounds.insert((9, 0), Seat::Occupied);
        after_4_rounds.insert((9, 2), Seat::Occupied);
        after_4_rounds.insert((9, 3), Seat::Empty);
        after_4_rounds.insert((9, 4), Seat::Occupied);
        after_4_rounds.insert((9, 5), Seat::Empty);
        after_4_rounds.insert((9, 6), Seat::Occupied);
        after_4_rounds.insert((9, 8), Seat::Occupied);
        after_4_rounds.insert((9, 9), Seat::Occupied);

        for _ in 0..4 {
            seating = next_round(&seating);
        }

        assert_eq!(seating, after_4_rounds);
    }
}
