/// --- Day 5: Binary Boarding ---
///
/// You board your plane only to discover a new problem: you dropped your
/// boarding pass! You aren't sure which seat is yours, and all of the flight
/// attendants are busy with the flood of people that suddenly made it through
/// passport control.
///
/// You write a quick program to use your phone's camera to scan all of the
/// nearby boarding passes (your puzzle input); perhaps you can find your seat
/// through process of elimination.
///
/// Instead of zones or groups, this airline uses binary space partitioning to
/// seat people. A seat might be specified like FBFBBFFRLR, where F means
/// "front", B means "back", L means "left", and R means "right".
///
/// The first 7 characters will either be F or B; these specify exactly one of
/// the 128 rows on the plane (numbered 0 through 127). Each letter tells you
/// which half of a region the given seat is in. Start with the whole list of
/// rows; the first letter indicates whether the seat is in the front (0 through
/// 63) or the back (64 through 127). The next letter indicates which half of
/// that region the seat is in, and so on until you're left with exactly one
/// row.
///
/// For example, consider just the first seven characters of FBFBBFFRLR:
///
///     Start by considering the whole range, rows 0 through 127.
///     F means to take the lower half, keeping rows 0 through 63.
///     B means to take the upper half, keeping rows 32 through 63.
///     F means to take the lower half, keeping rows 32 through 47.
///     B means to take the upper half, keeping rows 40 through 47.
///     B keeps rows 44 through 47.
///     F keeps rows 44 through 45.
///     The final F keeps the lower of the two, row 44.
///
/// The last three characters will be either L or R; these specify exactly one
/// of the 8 columns of seats on the plane (numbered 0 through 7). The same
/// process as above proceeds again, this time with only three steps. L means to
/// keep the lower half, while R means to keep the upper half.
///
/// For example, consider just the last 3 characters of FBFBBFFRLR:
///
///     Start by considering the whole range, columns 0 through 7.
///     R means to take the upper half, keeping columns 4 through 7.
///     L means to take the lower half, keeping columns 4 through 5.
///     The final R keeps the upper of the two, column 5.
///
/// So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.
///
/// Every seat also has a unique seat ID: multiply the row by 8, then add the
/// column. In this example, the seat has ID 44 * 8 + 5 = 357.
///
/// Here are some other boarding passes:
///
///     BFFFBBFRRR: row 70, column 7, seat ID 567.
///     FFFBBBFRRR: row 14, column 7, seat ID 119.
///     BBFFBBFRLL: row 102, column 4, seat ID 820.
///
/// As a sanity check, look through your list of boarding passes. What is the
/// highest seat ID on a boarding pass?
///
/// --- Part Two ---
///
/// Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
///
/// It's a completely full flight, so your seat should be the only missing
/// boarding pass in your list. However, there's a catch: some of the seats at
/// the very front and back of the plane don't exist on this aircraft, so
/// they'll be missing from your list as well.
///
/// Your seat wasn't at the very front or back, though; the seats with IDs +1
/// and -1 from yours will be in your list.
///
/// What is the ID of your seat?

const INPUT: &str = include_str!("../input/day_05.txt");

pub fn run() {
    let mut seat_ids: Vec<_> = INPUT.lines().map(|line| convert_to_seat_id(line)).collect();
    seat_ids.sort();

    let highest_seat_id = seat_ids.last().expect("No max found");
    println!(
        "The highest seat ID in the list of boarding passes is: {}",
        highest_seat_id
    );

    let your_seat_id = find_gap(&seat_ids).expect("Your seat id not found");
    println!("Your seat ID is: {}", your_seat_id);
}

fn find_gap(list: &Vec<u16>) -> Option<u16> {
    // Find gaps in sorted lists
    list.iter()
        .zip(list.iter().skip(1))
        .find(|(&n1, &n2)| (n1 + 2) == n2) // determine the next number is 2 away
        .map(|(n1, _)| n1 + 1) // return the number in the middle
}

fn convert_to_seat_id(boarding_pass: &str) -> u16 {
    boarding_pass.chars().fold(0, |seat_id, char| {
        (seat_id << 1)
            + (match char {
                'F' => 0,
                'B' => 1,
                'L' => 0,
                'R' => 1,
                _ => panic!("Unknown character in boarding pass"),
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_seat_id_1() {
        // BFFFBBFRRR: row 70, column 7, seat ID 567.
        let boarding_pass = "BFFFBBFRRR";
        let seat_id = 567;

        assert_eq!(convert_to_seat_id(boarding_pass), seat_id);
    }

    #[test]
    fn test_convert_to_seat_id_2() {
        // FFFBBBFRRR: row 14, column 7, seat ID 119.
        let boarding_pass = "FFFBBBFRRR";
        let seat_id = 119;

        assert_eq!(convert_to_seat_id(boarding_pass), seat_id);
    }

    #[test]
    fn test_convert_to_seat_id_3() {
        // BBFFBBFRLL: row 102, column 4, seat ID 820.
        let boarding_pass = "BBFFBBFRLL";
        let seat_id = 820;

        assert_eq!(convert_to_seat_id(boarding_pass), seat_id);
    }

    #[test]
    fn test_find_gap() {
        let sequence = vec![5, 6, 8, 9];
        let expected = Some(7);

        assert_eq!(find_gap(&sequence), expected);
    }
}
