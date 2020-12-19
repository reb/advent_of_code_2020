/// --- Day 16: Ticket Translation ---
///
/// As you're walking to yet another connecting flight, you realize that one of
/// the legs of your re-routed trip coming up is on a high-speed train. However,
/// the train ticket you were given is in a language you don't understand. You
/// should probably figure out what it says before you get to the train station
/// after the next flight.
///
/// Unfortunately, you can't actually read the words on the ticket. You can,
/// however, read the numbers, and so you figure out the fields these tickets
/// must have and the valid ranges for values in those fields.
///
/// You collect the rules for ticket fields, the numbers on your ticket, and the
/// numbers on other nearby tickets for the same train service (via the airport
/// security cameras) together into a single document you can reference (your
/// puzzle input).
///
/// The rules for ticket fields specify a list of fields that exist somewhere on
/// the ticket and the valid ranges of values for each field. For example, a
/// rule like class: 1-3 or 5-7 means that one of the fields in every ticket is
/// named class and can be any value in the ranges 1-3 or 5-7 (inclusive, such
/// that 3 and 5 are both valid in this field, but 4 is not).
///
/// Each ticket is represented by a single line of comma-separated values. The
/// values are the numbers on the ticket in the order they appear; every ticket
/// has the same format. For example, consider this ticket:
///
/// .--------------------------------------------------------.
/// | ????: 101    ?????: 102   ??????????: 103     ???: 104 |
/// |                                                        |
/// | ??: 301  ??: 302             ???????: 303      ??????? |
/// | ??: 401  ??: 402           ???? ????: 403    ????????? |
/// '--------------------------------------------------------'
///
/// Here, ? represents text in a language you don't understand. This ticket
/// might be represented as 101,102,103,104,301,302,303,401,402,403; of course,
/// the actual train tickets you're looking at are much more complicated. In any
/// case, you've extracted just the numbers in such a way that the first number
/// is always the same specific field, the second number is always a different
/// specific field, and so on - you just don't know what each position actually
/// means!
///
/// Start by determining which tickets are completely invalid; these are tickets
/// that contain values which aren't valid for any field. Ignore your ticket for
/// now.
///
/// For example, suppose you have the following notes:
///
/// class: 1-3 or 5-7
/// row: 6-11 or 33-44
/// seat: 13-40 or 45-50
///
/// your ticket:
/// 7,1,14
///
/// nearby tickets:
/// 7,3,47
/// 40,4,50
/// 55,2,20
/// 38,6,12
///
/// It doesn't matter which position corresponds to which field; you can
/// identify invalid nearby tickets by considering only whether tickets contain
/// values that are not valid for any field. In this example, the values on the
/// first nearby ticket are all valid for at least one field. This is not true
/// of the other three nearby tickets: the values 4, 55, and 12 are are not
/// valid for any field. Adding together all of the invalid values produces your
/// ticket scanning error rate: 4 + 55 + 12 = 71.
///
/// Consider the validity of the nearby tickets you scanned. What is your ticket
/// scanning error rate?
///
/// --- Part Two ---
///
/// Now that you've identified which tickets contain invalid values, discard
/// those tickets entirely. Use the remaining valid tickets to determine which
/// field is which.
///
/// Using the valid ranges for each field, determine what order the fields
/// appear on the tickets. The order is consistent between all tickets: if seat
/// is the third field, it is the third field on every ticket, including your
/// ticket.
///
/// For example, suppose you have the following notes:
///
/// class: 0-1 or 4-19
/// row: 0-5 or 8-19
/// seat: 0-13 or 16-19
///
/// your ticket:
/// 11,12,13
///
/// nearby tickets:
/// 3,9,18
/// 15,1,5
/// 5,14,9
///
/// Based on the nearby tickets in the above example, the first position must be
/// row, the second position must be class, and the third position must be seat;
/// you can conclude that in your ticket, class is 12, row is 11, and seat is
/// 13.
///
/// Once you work out which field is which, look for the six fields on your
/// ticket that start with the word departure. What do you get if you multiply
/// those six values together?
use regex::Regex;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day_16.txt");

pub fn run() {
    let notes = parse_notes(INPUT);

    let error_rate = scanning_error_rate(&notes);
    println!("The ticket scanning error rate is: {}", error_rate);

    let rule_mapping = map_rules_to_tickets(&notes);
    let departure_product: u64 = rule_mapping
        .iter()
        .filter_map(|(rule_name, &column)| {
            if rule_name.starts_with("departure") {
                Some(*notes.your_ticket.get(column).unwrap() as u64)
            } else {
                None
            }
        })
        .product();
    println!(
        "The fields starting with the word departure in your ticket multipied is: {}",
        departure_product
    );
}

fn scanning_error_rate(notes: &Notes) -> u32 {
    notes
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|number| notes.rules.iter().all(|rule| !rule.valid(number)))
        .sum()
}

fn map_rules_to_tickets<'a>(notes: &'a Notes) -> HashMap<&'a str, usize> {
    let valid_tickets = filter_out_invalid_tickets(notes);

    // create a mapping to collect every possible column for every rule
    let mut mapping: HashMap<&str, HashSet<usize>> = notes
        .rules
        .iter()
        .map(|rule| {
            (
                rule.name,
                (0..notes.your_ticket.len())
                    .into_iter()
                    .filter(|i| {
                        valid_tickets
                            .iter()
                            .all(|ticket| rule.valid(ticket.get(*i).unwrap()))
                    })
                    .collect(),
            )
        })
        .collect();

    // initialize a vector to hold the final mappings
    let mut final_mapping = HashMap::new();

    // keep trying to lock in columns to rules
    loop {
        let mut found = None;
        for (rule_name, columns) in mapping.iter() {
            if columns.len() == 1 {
                // if there is only one column then that must belong to this rule
                let found_column = *columns.iter().next().unwrap();
                final_mapping.insert(*rule_name, found_column);
                found = Some((*rule_name, found_column));
                break;
            }
        }

        if let Some((found_rule, found_column)) = found {
            // remove the locked in rule and column from the mapping
            mapping.remove(found_rule);
            for columns in mapping.values_mut() {
                columns.remove(&found_column);
            }
        } else {
            // just stop if no new column could be locked in
            break;
        }
    }

    final_mapping
}

fn filter_out_invalid_tickets<'a>(notes: &'a Notes) -> Vec<&'a Ticket> {
    notes
        .nearby_tickets
        .iter()
        .filter(|numbers| {
            numbers
                .iter()
                .all(|number| notes.rules.iter().any(|rule| rule.valid(number)))
        })
        .collect()
}

fn parse_notes(input: &str) -> Notes {
    let mut blocks = input.split("\n\n");

    let rules = blocks
        .next()
        .expect("Expected a rules block")
        .lines()
        .map(convert_to_rule)
        .collect();

    let your_ticket = convert_to_ticket(
        blocks
            .next()
            .expect("Expected a your ticket block")
            .lines()
            .skip(1)
            .next()
            .expect("Expected a line with your ticket"),
    );

    let nearby_tickets = blocks
        .next()
        .expect("Expected a nearby tickets block")
        .lines()
        .skip(1)
        .map(convert_to_ticket)
        .collect();

    Notes {
        rules,
        your_ticket,
        nearby_tickets,
    }
}

fn convert_to_rule(line: &str) -> Rule {
    lazy_static! {
        static ref NAME_RE: Regex = Regex::new(r"^([a-z ]+):").unwrap();
        static ref RANGE_RE: Regex = Regex::new(r"([0-9]+)-([0-9]+)").unwrap();
    }

    let name = NAME_RE
        .captures(line)
        .expect("Expected a match on the name")
        .get(1)
        .expect("Expected a group to be filled with the name")
        .as_str();

    let ranges = RANGE_RE
        .captures_iter(line)
        .filter_map(|captures| {
            match (
                captures.get(1).unwrap().as_str().parse(),
                captures.get(2).unwrap().as_str().parse(),
            ) {
                (Ok(min), Ok(max)) => Some(Range { min, max }),
                _ => None,
            }
        })
        .collect();

    Rule { name, ranges }
}

fn convert_to_ticket(line: &str) -> Ticket {
    line.split(",")
        .map(str::parse)
        .filter_map(Result::ok)
        .collect()
}

#[derive(Debug, PartialEq)]
struct Notes<'a> {
    rules: Vec<Rule<'a>>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[derive(Debug, PartialEq)]
struct Rule<'a> {
    name: &'a str,
    ranges: Vec<Range>,
}

impl Rule<'_> {
    fn valid(&self, value: &u32) -> bool {
        self.ranges.iter().any(|range| range.contains(value))
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn contains(&self, value: &u32) -> bool {
        self.min <= *value && *value <= self.max
    }
}

type Ticket = Vec<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ticket_notes() {
        let input = "\
            class: 1-3 or 5-7\n\
            row: 6-11 or 33-44\n\
            seat: 13-40 or 45-50\n\
            \n\
            your ticket:\n\
            7,1,14\n\
            \n\
            nearby tickets:\n\
            7,3,47\n\
            40,4,50\n\
            55,2,20\n\
            38,6,12";

        let rules = vec![
            Rule {
                name: "class",
                ranges: vec![Range { min: 1, max: 3 }, Range { min: 5, max: 7 }],
            },
            Rule {
                name: "row",
                ranges: vec![Range { min: 6, max: 11 }, Range { min: 33, max: 44 }],
            },
            Rule {
                name: "seat",
                ranges: vec![Range { min: 13, max: 40 }, Range { min: 45, max: 50 }],
            },
        ];
        let your_ticket = vec![7, 1, 14];
        let nearby_tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];

        let expected_notes = Notes {
            rules,
            your_ticket,
            nearby_tickets,
        };

        assert_eq!(parse_notes(input), expected_notes);
    }

    #[test]
    fn test_scanning_error_rate() {
        let rules = vec![
            Rule {
                name: "class",
                ranges: vec![Range { min: 1, max: 3 }, Range { min: 5, max: 7 }],
            },
            Rule {
                name: "row",
                ranges: vec![Range { min: 6, max: 11 }, Range { min: 33, max: 44 }],
            },
            Rule {
                name: "seat",
                ranges: vec![Range { min: 13, max: 40 }, Range { min: 45, max: 50 }],
            },
        ];
        let your_ticket = vec![7, 1, 14];
        let nearby_tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];

        let notes = Notes {
            rules,
            your_ticket,
            nearby_tickets,
        };

        assert_eq!(scanning_error_rate(&notes), 71);
    }

    #[test]
    fn test_map_rules_to_tickets() {
        // class: 0-1 or 4-19
        // row: 0-5 or 8-19
        // seat: 0-13 or 16-19
        //
        // your ticket:
        // 11,12,13
        //
        // nearby tickets:
        // 3,9,18
        // 15,1,5
        // 5,14,9
        let rules = vec![
            Rule {
                name: "class",
                ranges: vec![Range { min: 0, max: 1 }, Range { min: 4, max: 19 }],
            },
            Rule {
                name: "row",
                ranges: vec![Range { min: 0, max: 5 }, Range { min: 8, max: 19 }],
            },
            Rule {
                name: "seat",
                ranges: vec![Range { min: 0, max: 13 }, Range { min: 16, max: 19 }],
            },
        ];
        let your_ticket = vec![11, 12, 13];
        let nearby_tickets = vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]];

        let notes = Notes {
            rules,
            your_ticket,
            nearby_tickets,
        };

        // Based on the nearby tickets in the above example, the first position must be
        // row, the second position must be class, and the third position must be seat;
        let mut expected_mapping = HashMap::new();
        expected_mapping.insert("row", 0);
        expected_mapping.insert("class", 1);
        expected_mapping.insert("seat", 2);

        assert_eq!(map_rules_to_tickets(&notes), expected_mapping);
    }

    #[test]
    fn test_filter_out_invalid_tickets() {
        let rules = vec![Rule {
            name: "class",
            ranges: vec![Range { min: 0, max: 2 }, Range { min: 4, max: 6 }],
        }];
        let your_ticket = vec![1];
        let nearby_tickets = vec![vec![3], vec![5], vec![2]];

        let notes = Notes {
            rules,
            your_ticket,
            nearby_tickets,
        };

        let t1 = vec![5];
        let t2 = vec![2];
        let expected_valid_tickets = vec![&t1, &t2];

        assert_eq!(filter_out_invalid_tickets(&notes), expected_valid_tickets);
    }
}
