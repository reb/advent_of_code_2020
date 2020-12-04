/// --- Day 1: Report Repair ---
///
/// After saving Christmas five years in a row, you've decided to take a
/// vacation at a nice resort on a tropical island. Surely, Christmas will go on
/// without you.
///
/// The tropical island has its own currency and is entirely cash-only. The gold
/// coins used there have a little picture of a starfish; the locals just call
/// them stars. None of the currency exchanges seem to have heard of them, but
/// somehow, you'll need to find fifty of these coins by the time you arrive so
/// you can pay the deposit on your room.
///
/// To save your vacation, you need to get all fifty stars by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each
/// day in the Advent calendar; the second puzzle is unlocked when you complete
/// the first. Each puzzle grants one star. Good luck!
///
/// Before you leave, the Elves in accounting just need you to fix your expense
/// report (your puzzle input); apparently, something isn't quite adding up.
///
/// Specifically, they need you to find the two entries that sum to 2020 and
/// then multiply those two numbers together.
///
/// For example, suppose your expense report contained the following:
///
/// 1721
/// 979
/// 366
/// 299
/// 675
/// 1456
///
/// In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying
/// them together produces 1721 * 299 = 514579, so the correct answer is 514579.
///
/// Of course, your expense report is much larger. Find the two entries that sum
/// to 2020; what do you get if you multiply them together?
///
/// --- Part Two ---
///
/// The Elves in accounting are thankful for your help; one of them even offers
/// you a starfish coin they had left over from a past vacation. They offer you
/// a second one if you can find three numbers in your expense report that meet
/// the same criteria.
///
/// Using the above example again, the three entries that sum to 2020 are 979,
/// 366, and 675.  Multiplying them together produces the answer, 241861950.
///
/// In your expense report, what is the product of the three entries that sum to
/// 2020?
use itertools::Itertools;

const INPUT: &str = include_str!("../input/day_01.txt");

pub fn run() {
    // sort the expense report to make finding two sums easier
    let mut expense_report = parse_to_ints(INPUT);
    expense_report.sort();

    let pair = find_sum(&expense_report, 2020, 2);
    println!(
        "The two entries that sum to 2020 multiplied are {} * {} = {}",
        pair[0],
        pair[1],
        pair.iter().product::<u32>()
    );

    let triplet = find_sum(&expense_report, 2020, 3);
    println!(
        "The three entries that sum to 2020 multiplied are {} * {} * {} = {}",
        triplet[0],
        triplet[1],
        triplet[2],
        triplet.iter().product::<u32>()
    );
}

fn find_sum(expense_report: &Vec<u32>, target: u32, numbers: usize) -> Vec<u32> {
    expense_report
        .iter()
        .combinations(numbers)
        .map(|vec| vec.iter().map(|&&i| i).collect::<Vec<u32>>())
        .find(|vec| vec.iter().sum::<u32>() == target)
        .expect("No sum found")
}

fn parse_to_ints(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse())
        .filter_map(Result::ok)
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_ints() {
        let input = "123\n456\n789";
        let expected = vec![123, 456, 789];

        assert_eq!(parse_to_ints(input), expected);
    }

    #[test]
    fn test_find_sum_with_2() {
        let mut expense_report = vec![1721, 979, 366, 299, 675, 1456];
        expense_report.sort();
        let target = 2020;

        assert_eq!(find_sum(&expense_report, target, 2), vec![299, 1721]);
    }

    #[test]
    fn test_find_sum_with_3() {
        let mut expense_report = vec![1721, 979, 366, 299, 675, 1456];
        expense_report.sort();
        let target = 2020;

        assert_eq!(find_sum(&expense_report, target, 3), vec![366, 675, 979]);
    }
}
