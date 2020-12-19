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
use regex::Regex;

const INPUT: &str = include_str!("../input/day_16.txt");

pub fn run() {
    let notes = parse_notes(INPUT);

    let error_rate = scanning_error_rate(&notes);
    println!("The ticket scanning error rate is: {}", error_rate);
}

fn scanning_error_rate(notes: &Notes) -> u32 {
    let all_ranges: Vec<&Range> = notes.rules.iter().flat_map(|rule| &rule.ranges).collect();

    notes
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|number| all_ranges.iter().all(|range| !range.contains(number)))
        .sum()
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
}
