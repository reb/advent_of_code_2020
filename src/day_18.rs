/// --- Day 18: Operation Order ---
///
/// As you look out the window and notice a heavily-forested continent slowly
/// appear over the horizon, you are interrupted by the child sitting next to
/// you. They're curious if you could help them with their math homework.
///
/// Unfortunately, it seems like this "math" follows different rules than you
/// remember.
///
/// The homework (your puzzle input) consists of a series of expressions that
/// consist of addition (+), multiplication (*), and parentheses ((...)). Just
/// like normal math, parentheses indicate that the expression inside must be
/// evaluated before it can be used by the surrounding expression. Addition
/// still finds the sum of the numbers on both sides of the operator, and
/// multiplication still finds the product.
///
/// However, the rules of operator precedence have changed. Rather than
/// evaluating multiplication before addition, the operators have the same
/// precedence, and are evaluated left-to-right regardless of the order in which
/// they appear.
///
/// For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are
/// as follows:
///
/// 1 + 2 * 3 + 4 * 5 + 6
///   3   * 3 + 4 * 5 + 6
///       9   + 4 * 5 + 6
///          13   * 5 + 6
///              65   + 6
///                  71
///
/// Parentheses can override this order; for example, here is what happens if
/// parentheses are added to form 1 + (2 * 3) + (4 * (5 + 6)):
///
/// 1 + (2 * 3) + (4 * (5 + 6))
/// 1 +    6    + (4 * (5 + 6))
///      7      + (4 * (5 + 6))
///      7      + (4 *   11   )
///      7      +     44
///             51
///
/// Here are a few more examples:
///
///     2 * 3 + (4 * 5) becomes 26.
///     5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
///     5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
///     ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
///
/// Before you can help with the homework, you need to understand it yourself.
/// Evaluate the expression on each line of the homework; what is the sum of the
/// resulting values?
use std::ops::{Add, Mul};

const INPUT: &str = include_str!("../input/day_18.txt");

pub fn run() {
    let homework = INPUT;

    println!(
        "The sum of all lines of homework is: {}",
        homework.lines().map(evaluate).sum::<u64>()
    );
}

fn evaluate(expression: &str) -> u64 {
    let mut characters = expression.chars().filter(|&c| c != ' ');
    sub_evaluate(&mut characters)
}

fn sub_evaluate(characters: &mut impl Iterator<Item = char>) -> u64 {
    let mut total = None;
    let mut operator: Option<fn(u64, u64) -> u64> = None;
    while let Some(character) = characters.next() {
        let mut number = None;
        match character {
            '(' => number = Some(sub_evaluate(characters)),
            ')' => return total.expect("Expecting parenthesis to not be empty"),
            '*' => operator = Some(u64::mul),
            '+' => operator = Some(u64::add),
            digit => {
                number = Some(
                    digit
                        .to_digit(10)
                        .expect("Expecting non-operators to be digits") as u64,
                )
            }
        }
        match (total, operator, number) {
            (None, None, Some(n)) => total = Some(n),
            (Some(t), Some(o), Some(n)) => {
                total = Some(o(t, n));
                operator = None
            }
            (Some(_), Some(_), None) => (),
            situation => panic!("Unexpected situation: {:?}", situation),
        }
    }
    total.expect("Expected a non-empty expression")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_1() {
        let expression = "1 + 2 * 3 + 4 * 5 + 6";

        assert_eq!(evaluate(expression), 71);
    }

    #[test]
    fn test_evaluate_2() {
        let expression = "1 + (2 * 3) + (4 * (5 + 6))";

        assert_eq!(evaluate(expression), 51);
    }

    #[test]
    fn test_evaluate_3() {
        let expression = "2 * 3 + (4 * 5)";

        assert_eq!(evaluate(expression), 26);
    }

    #[test]
    fn test_evaluate_4() {
        let expression = "5 + (8 * 3 + 9 + 3 * 4 * 3)";

        assert_eq!(evaluate(expression), 437);
    }

    #[test]
    fn test_evaluate_5() {
        let expression = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";

        assert_eq!(evaluate(expression), 12240);
    }

    #[test]
    fn test_evaluate_6() {
        let expression = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

        assert_eq!(evaluate(expression), 13632);
    }
}
