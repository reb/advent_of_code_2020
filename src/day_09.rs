/// --- Day 9: Encoding Error ---
///
/// With your neighbor happily enjoying their video game, you turn your
/// attention to an open data port on the little screen in the seat in front of
/// you.
///
/// Though the port is non-standard, you manage to connect it to your computer
/// through the clever use of several paperclips. Upon connection, the port
/// outputs a series of numbers (your puzzle input).
///
/// The data appears to be encrypted with the eXchange-Masking Addition System
/// (XMAS) which, conveniently for you, is an old cypher with an important
/// weakness.
///
/// XMAS starts by transmitting a preamble of 25 numbers. After that, each
/// number you receive should be the sum of any two of the 25 immediately
/// previous numbers. The two numbers will have different values, and there
/// might be more than one such pair.
///
/// For example, suppose your preamble consists of the numbers 1 through 25 in a
/// random order. To be valid, the next number must be the sum of two of those
/// numbers:
///
///   - 26 would be a valid next number, as it could be 1 plus 25 (or many other
///     pairs, like 2 and 24).
///   - 49 would be a valid next number, as it is the sum of 24 and 25.
///   - 100 would not be valid; no two of the previous 25 numbers sum to 100.
///   - 50 would also not be valid; although 25 appears in the previous 25
///     numbers, the two numbers in the pair must be different.
///
/// Suppose the 26th number is 45, and the first number (no longer an option, as
/// it is more than 25 numbers ago) was 20. Now, for the next number to be
/// valid, there needs to be some pair of numbers among 1-19, 21-25, or 45 that
/// add up to it:
///
///   - 26 would still be a valid next number, as 1 and 25 are still within the
///     previous 25 numbers.
///   - 65 would not be valid, as no two of the available numbers sum to it.
///   - 64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.
///
/// Here is a larger example which only considers the previous 5 numbers (and
/// has a preamble of length 5):
///
/// 35
/// 20
/// 15
/// 25
/// 47
/// 40
/// 62
/// 55
/// 65
/// 95
/// 102
/// 117
/// 150
/// 182
/// 127
/// 219
/// 299
/// 277
/// 309
/// 576
///
/// In this example, after the 5-number preamble, almost every number is the sum
/// of two of the previous 5 numbers; the only number that does not follow this
/// rule is 127.
///
/// The first step of attacking the weakness in the XMAS data is to find the
/// first number in the list (after the preamble) which is not the sum of two of
/// the 25 numbers before it. What is the first number that does not have this
/// property?
use itertools::Itertools;

const INPUT: &str = include_str!("../input/day_09.txt");

pub fn run() {
    let xmas_data = parse_xmas_data(INPUT);

    let invalid_number = find_invalid_number(&xmas_data, 25).expect("No invalid number found");
    println!(
        "The first number to not be a sum of a pair of the previous 25 number is: {}",
        invalid_number
    );
}

fn find_invalid_number(xmas_data: &Vec<u64>, preamble_size: usize) -> Option<u64> {
    for index in preamble_size..xmas_data.len() {
        let number = xmas_data.get(index).unwrap();
        if !xmas_data[index - preamble_size..index]
            .iter()
            .combinations(2)
            .any(|vec| *number == vec.into_iter().sum())
        {
            return Some(*number);
        }
    }
    None
}

fn parse_xmas_data(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(str::parse)
        .filter_map(Result::ok)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_xmas_data() {
        let input = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
        let expected_data = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(parse_xmas_data(input), expected_data);
    }

    #[test]
    fn test_find_invalid_number() {
        let data = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(find_invalid_number(&data, 5), Some(127));
    }
}
