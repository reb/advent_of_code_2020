/// --- Day 6: Custom Customs ---
///
/// As your flight approaches the regional airport where you'll switch to a much
/// larger plane, customs declaration forms are distributed to the passengers.
///
/// The form asks a series of 26 yes-or-no questions marked a through z. All you
/// need to do is identify the questions for which anyone in your group answers
/// "yes". Since your group is just you, this doesn't take very long.
///
/// However, the person sitting next to you seems to be experiencing a language
/// barrier and asks if you can help. For each of the people in their group, you
/// write down the questions for which they answer "yes", one per line. For
/// example:
///
/// abcx
/// abcy
/// abcz
///
/// In this group, there are 6 questions to which anyone answered "yes": a, b,
/// c, x, y, and z. (Duplicate answers to the same question don't count extra;
/// each question counts at most once.)
///
/// Another group asks for your help, then another, and eventually you've
/// collected answers from every group on the plane (your puzzle input). Each
/// group's answers are separated by a blank line, and within each group, each
/// person's answers are on a single line. For example:
///
/// abc
///
/// a
/// b
/// c
///
/// ab
/// ac
///
/// a
/// a
/// a
/// a
///
/// b
///
/// This list represents answers from five groups:
///
///     The first group contains one person who answered "yes" to 3 questions:
///     a, b, and c.
///     The second group contains three people; combined, they answered "yes" to
///     3 questions: a, b, and c.
///     The third group contains two people; combined, they answered "yes" to 3
///     questions: a, b, and c.
///     The fourth group contains four people; combined, they answered "yes" to
///     only 1 question, a.
///     The last group contains one person who answered "yes" to only 1
///     question, b.
///
/// In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.
///
/// For each group, count the number of questions to which anyone answered
/// "yes". What is the sum of those counts?
use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day_06.txt");

pub fn run() {
    let groups_answers = load_groups_answers(INPUT);

    let sum = groups_answers
        .iter()
        .map(|group| {
            // union all answer sets of a group
            let mut iter = group.iter();
            iter.next().map_or(HashSet::new(), |answers| {
                iter.fold(answers.clone(), |all_answers, more_answers| {
                    all_answers.union(more_answers).cloned().collect()
                })
            })
        })
        .map(|any_answers| any_answers.len() as u32)
        .sum::<u32>();
    println!("Counting the number or questions to which anyone answered \"yes\" to for each group gives: {}", sum);
}

fn load_groups_answers(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|answers| answers.chars().filter(|&c| c >= 'a' && c <= 'z').collect())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_groups_answers() {
        let input = "\
            abc\n\
            \n\
            a\n\
            b\n\
            c\n\
            \n\
            ab\n\
            ac\n\
            \n\
            a\n\
            a\n\
            a\n\
            a\n\
            \n\
            b";

        let abc: HashSet<char> = ['a', 'b', 'c'].iter().cloned().collect();
        let a: HashSet<char> = ['a'].iter().cloned().collect();
        let b: HashSet<char> = ['b'].iter().cloned().collect();
        let c: HashSet<char> = ['c'].iter().cloned().collect();
        let ab: HashSet<char> = ['a', 'b'].iter().cloned().collect();
        let ac: HashSet<char> = ['a', 'c'].iter().cloned().collect();

        let g1 = vec![abc];
        let g2 = vec![a.clone(), b.clone(), c.clone()];
        let g3 = vec![ab, ac];
        let g4 = vec![a.clone(), a.clone(), a.clone(), a.clone()];
        let g5 = vec![b.clone()];

        let expected = vec![g1, g2, g3, g4, g5];

        assert_eq!(load_groups_answers(input), expected);
    }
}
