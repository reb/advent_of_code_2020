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
///
/// --- Part Two ---
///
/// You manage to answer the child's questions and they finish part 1 of their
/// homework, but get stuck when they reach the next section: advanced math.
///
/// Now, addition and multiplication have different precedence levels, but
/// they're not the ones you're familiar with. Instead, addition is evaluated
/// before multiplication.
///
/// For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are
/// now as follows:
///
/// 1 + 2 * 3 + 4 * 5 + 6
///   3   * 3 + 4 * 5 + 6
///   3   *   7   * 5 + 6
///   3   *   7   *  11
///      21       *  11
///          231
///
/// Here are the other examples from above:
///
///     1 + (2 * 3) + (4 * (5 + 6)) still becomes 51.
///     2 * 3 + (4 * 5) becomes 46.
///     5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 1445.
///     5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 669060.
///     ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 23340.
///
/// What do you get if you add up the results of evaluating the homework
/// problems using these new rules?
use std::ops::{Add, Mul};

const INPUT: &str = include_str!("../input/day_18.txt");

pub fn run() {
    let homework = INPUT;

    println!(
        "The sum of all lines of homework is: {}",
        homework.lines().map(evaluate).sum::<u64>()
    );

    println!(
        "Adding up the results of all the homework problems using the new rules gives: {}",
        homework.lines().map(advanced_evaluate).sum::<u64>()
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

fn advanced_evaluate(expression: &str) -> u64 {
    let mut characters = expression.chars().filter(|&c| c != ' ');
    advanced_sub_evaluate(&mut characters)
}

fn advanced_sub_evaluate(characters: &mut impl Iterator<Item = char>) -> u64 {
    let mut number_stack = Vec::new();
    let mut is_addition = false;

    while let Some(character) = characters.next() {
        let mut number = None;
        match character {
            '(' => number = Some(advanced_sub_evaluate(characters)),
            ')' => return number_stack.iter().product(),
            '*' => (),
            '+' => is_addition = true,
            digit => {
                number = Some(
                    digit
                        .to_digit(10)
                        .expect("Expecting non-operators to be digits") as u64,
                )
            }
        }
        match (number_stack.as_slice(), is_addition, number) {
            ([], false, Some(n)) => number_stack.push(n),
            (_, true, Some(n)) => {
                let last = number_stack
                    .pop()
                    .expect("Expected a number before the addition");
                number_stack.push(last + n);
                is_addition = false;
            }
            (_, false, Some(n)) => number_stack.push(n), // multiplication is handled at return
            (_, _, None) => (),
        }
    }
    number_stack.iter().product()
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

    #[test]
    fn test_advanced_evaluate_1() {
        let expression = "1 + 2 * 3 + 4 * 5 + 6";

        assert_eq!(advanced_evaluate(expression), 231);
    }

    #[test]
    fn test_advanced_evaluate_2() {
        let expression = "1 + (2 * 3) + (4 * (5 + 6))";

        assert_eq!(advanced_evaluate(expression), 51);
    }

    #[test]
    fn test_advanced_evaluate_3() {
        let expression = "2 * 3 + (4 * 5)";

        assert_eq!(advanced_evaluate(expression), 46);
    }

    #[test]
    fn test_advanced_evaluate_4() {
        let expression = "5 + (8 * 3 + 9 + 3 * 4 * 3)";

        assert_eq!(advanced_evaluate(expression), 1445);
    }

    #[test]
    fn test_advanced_evaluate_5() {
        let expression = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";

        assert_eq!(advanced_evaluate(expression), 669060);
    }

    #[test]
    fn test_advanced_evaluate_6() {
        let expression = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

        assert_eq!(advanced_evaluate(expression), 23340);
    }
}
