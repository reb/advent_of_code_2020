/// --- Day 2: Password Philosophy ---
///
/// Your flight departs in a few days from the coastal airport; the easiest way
/// down to the coast from here is via toboggan.
///
/// The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
/// "Something's wrong with our computers; we can't log in!" You ask if you can
/// take a look.
///
/// Their password database seems to be a little corrupted: some of the
/// passwords wouldn't have been allowed by the Official Toboggan Corporate
/// Policy that was in effect when they were chosen.
///
/// To try to debug the problem, they have created a list (your puzzle input) of
/// passwords (according to the corrupted database) and the corporate policy
/// when that password was set.
///
/// For example, suppose you have the following list:
///
/// 1-3 a: abcde
/// 1-3 b: cdefg
/// 2-9 c: ccccccccc
///
/// Each line gives the password policy and then the password. The password
/// policy indicates the lowest and highest number of times a given letter must
/// appear for the password to be valid. For example, 1-3 a means that the
/// password must contain a at least 1 time and at most 3 times.
///
/// In the above example, 2 passwords are valid. The middle password, cdefg, is
/// not; it contains no instances of b, but needs at least 1. The first and
/// third passwords are valid: they contain one a or nine c, both within the
/// limits of their respective policies.
///
/// How many passwords are valid according to their policies?
use regex::Regex;

const INPUT: &str = include_str!("../input/day_02.txt");

pub fn run() {
    let passwords = parse_passwords(INPUT);

    // validate passwords
    let valid_passwords = find_valid_passwords(passwords);
    println!(
        "The amount of passwords valid according to their policies is: {}",
        valid_passwords.len()
    );
}
fn find_valid_passwords(passwords: Vec<Password>) -> Vec<Password> {
    passwords
        .into_iter()
        .filter(|password| {
            let char_count = password
                .password
                .chars()
                .filter(|&c| c == password.policy.char)
                .count();
            char_count >= password.policy.min && char_count <= password.policy.max
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Password {
    policy: Policy,
    password: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Policy {
    min: usize,
    max: usize,
    char: char,
}

fn parse_passwords(input: &str) -> Vec<Password> {
    input
        .lines()
        .filter_map(|line| convert_to_password(line))
        .collect()
}

fn convert_to_password(line: &str) -> Option<Password> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]+)-([0-9]+) (.): (.*)").unwrap();
    }
    let captures = RE.captures(line);
    match captures {
        Some(groups) => match (groups.get(1), groups.get(2), groups.get(3), groups.get(4)) {
            (Some(min), Some(max), Some(char), Some(password)) => match (
                min.as_str().parse(),
                max.as_str().parse(),
                char.as_str().chars().next(),
                password.as_str().to_string(),
            ) {
                (Ok(min), Ok(max), Some(char), password) => Some(Password {
                    password,
                    policy: Policy { min, max, char },
                }),

                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_policies() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let passwords = vec![
            Password {
                password: "abcde".to_string(),
                policy: Policy {
                    min: 1,
                    max: 3,
                    char: 'a',
                },
            },
            Password {
                password: "cdefg".to_string(),
                policy: Policy {
                    min: 1,
                    max: 3,
                    char: 'b',
                },
            },
            Password {
                password: "ccccccccc".to_string(),
                policy: Policy {
                    min: 2,
                    max: 9,
                    char: 'c',
                },
            },
        ];

        assert_eq!(parse_passwords(input), passwords);
    }

    #[test]
    fn test_find_valid_passwords() {
        let passwords = vec![
            Password {
                password: "abcde".to_string(),
                policy: Policy {
                    min: 1,
                    max: 3,
                    char: 'a',
                },
            },
            Password {
                password: "cdefg".to_string(),
                policy: Policy {
                    min: 1,
                    max: 3,
                    char: 'b',
                },
            },
            Password {
                password: "ccccccccc".to_string(),
                policy: Policy {
                    min: 2,
                    max: 9,
                    char: 'c',
                },
            },
        ];

        let valid_passwords = vec![
            Password {
                password: "abcde".to_string(),
                policy: Policy {
                    min: 1,
                    max: 3,
                    char: 'a',
                },
            },
            Password {
                password: "ccccccccc".to_string(),
                policy: Policy {
                    min: 2,
                    max: 9,
                    char: 'c',
                },
            },
        ];

        assert_eq!(find_valid_passwords(passwords), valid_passwords);
    }
}
