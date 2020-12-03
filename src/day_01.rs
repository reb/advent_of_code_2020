/// --- Day 1: Report Repair ---
///
/// After saving Christmas five years in a row, you've decided to take a vacation at a nice resort
/// on a tropical island. Surely, Christmas will go on without you.
///
/// The tropical island has its own currency and is entirely cash-only. The gold coins used there
/// have a little picture of a starfish; the locals just call them stars. None of the currency
/// exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by
/// the time you arrive so you can pay the deposit on your room.
///
/// To save your vacation, you need to get all fifty stars by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
/// calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one
/// star. Good luck!
///
/// Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle
/// input); apparently, something isn't quite adding up.
///
/// Specifically, they need you to find the two entries that sum to 2020 and then multiply those
/// two numbers together.
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
/// In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together
/// produces 1721 * 299 = 514579, so the correct answer is 514579.
///
/// Of course, your expense report is much larger. Find the two entries that sum to 2020; what do
/// you get if you multiply them together?

const INPUT: &str = include_str!("../input/day_01.txt");

pub fn run() {
    // sort the expense report to make finding two sums easier
    let mut expense_report = parse_to_ints(INPUT);
    expense_report.sort();

    let (n1, n2) = find_sum(&expense_report, 2020);

    println!(
        "The two entries that sum to 2020 multiplied are {} * {} = {}",
        n1,
        n2,
        n1 * n2
    );
}

fn find_sum(expense_report: &Vec<u32>, target: u32) -> (u32, u32) {
    for i in 0..expense_report.len() {
        for j in i..expense_report.len() {
            let sum = expense_report[i] + expense_report[j];
            if sum == target {
                return (expense_report[i], expense_report[j]);
            } else if sum > target {
                break;
            }
        }
    }
    panic!("No sum found!")
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
    fn test_find_sum() {
        let mut expense_report = vec![1721, 979, 366, 299, 675, 1456];
        expense_report.sort();
        let target = 2020;

        assert_eq!(find_sum(&expense_report, target), (299, 1721));
    }
}
