/// --- Day 19: Monster Messages ---
///
/// You land in an airport surrounded by dense forest. As you walk to your
/// high-speed train, the Elves at the Mythical Information Bureau contact you
/// again. They think their satellite has collected an image of a sea monster!
/// Unfortunately, the connection to the satellite is having problems, and many
/// of the messages sent back from the satellite have been corrupted.
///
/// They sent you a list of the rules valid messages should obey and a list of
/// received messages they've collected so far (your puzzle input).
///
/// The rules for valid messages (the top part of your puzzle input) are
/// numbered and build upon each other. For example:
///
/// 0: 1 2
/// 1: "a"
/// 2: 1 3 | 3 1
/// 3: "b"
///
/// Some rules, like 3: "b", simply match a single character (in this case, b).
///
/// The remaining rules list the sub-rules that must be followed; for example,
/// the rule 0: 1 2 means that to match rule 0, the text being checked must
/// match rule 1, and the text after the part that matched rule 1 must then
/// match rule 2.
///
/// Some of the rules have multiple lists of sub-rules separated by a pipe (|).
/// This means that at least one list of sub-rules must match. (The ones that
/// match might be different each time the rule is encountered.) For example,
/// the rule 2: 1 3 | 3 1 means that to match rule 2, the text being checked
/// must match rule 1 followed by rule 3 or it must match rule 3 followed by
/// rule 1.
///
/// Fortunately, there are no loops in the rules, so the list of possible
/// matches will be finite. Since rule 1 matches a and rule 3 matches b, rule 2
/// matches either ab or ba. Therefore, rule 0 matches aab or aba.
///
/// Here's a more interesting example:
///
/// 0: 4 1 5
/// 1: 2 3 | 3 2
/// 2: 4 4 | 5 5
/// 3: 4 5 | 5 4
/// 4: "a"
/// 5: "b"
///
/// Here, because rule 4 matches a and rule 5 matches b, rule 2 matches two
/// letters that are the same (aa or bb), and rule 3 matches two letters that
/// are different (ab or ba).
///
/// Since rule 1 matches rules 2 and 3 once each in either order, it must match
/// two pairs of letters, one pair with matching letters and one pair with
/// different letters. This leaves eight possibilities: aaab, aaba, bbab, bbba,
/// abaa, abbb, baaa, or babb.
///
/// Rule 0, therefore, matches a (rule 4), then any of the eight options from rule 1, then b (rule 5): aaaabb, aaabab, abbabb, abbbab, aabaab, aabbbb, abaaab, or ababbb.
///
/// The received messages (the bottom part of your puzzle input) need to be
/// checked against the rules so you can determine which are valid and which are
/// corrupted. Including the rules and the messages together, this might look
/// like:
///
/// 0: 4 1 5
/// 1: 2 3 | 3 2
/// 2: 4 4 | 5 5
/// 3: 4 5 | 5 4
/// 4: "a"
/// 5: "b"
///
/// ababbb
/// bababa
/// abbbab
/// aaabbb
/// aaaabbb
///
/// Your goal is to determine the number of messages that completely match rule
/// 0. In the above example, ababbb and abbbab match, but bababa, aaabbb, and
/// aaaabbb do not, producing the answer 2. The whole message must match all of
/// rule 0; there can't be extra unmatched characters in the message. (For
/// example, aaaabbb might appear to match rule 0 above, but it has an extra
/// unmatched b on the end.)
///
/// How many messages completely match rule 0?
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_19.txt");

pub fn run() {
    let mut input_blocks = INPUT.split("\n\n");

    let rules = parse_rules(input_blocks.next().expect("Expected a rules block"));
    let (possibility_tree, root) = build_possibility_tree(&rules);
}

type PossibilityTree = DiGraph<u32, char>;

fn build_possibility_tree(rules: &Rules) -> (PossibilityTree, NodeIndex) {
    let mut tree = PossibilityTree::new();

    let begin = tree.add_node(0);
    let end = tree.add_node(0);

    expand_chain(&mut tree, begin, end, rules, &0);

    (tree, begin)
}

fn expand_chain(
    tree: &mut PossibilityTree,
    begin: NodeIndex,
    end: NodeIndex,
    rules: &Rules,
    rule: &RuleName,
) {
    match rules.get(rule).map(|options| options.as_slice()) {
        Some([RuleOption::Literal(letter)]) => {
            tree.add_edge(begin, end, *letter);
        }
        Some(chains) => {
            for chain in chains.iter() {
                match chain {
                    RuleOption::Chain(chained_rules) => {
                        let mut chain_begin = begin;
                        let mut chain_iter = chained_rules.iter().peekable();
                        while let Some(rule_name) = chain_iter.next() {
                            let chain_next = if chain_iter.peek().is_some() {
                                tree.add_node(0)
                            } else {
                                end
                            };
                            expand_chain(tree, chain_begin, chain_next, rules, rule_name);
                            chain_begin = chain_next;
                        }
                    }
                    RuleOption::Literal(_) => panic!("More than 1 literal in rule"),
                }
            }
        }
        None => panic!("Rule not found"),
    }
}

type Rules = HashMap<RuleName, Vec<RuleOption>>;
type RuleName = u16;
#[derive(Debug, PartialEq)]
enum RuleOption {
    Chain(Vec<RuleName>),
    Literal(char),
}

fn parse_rules(input: &str) -> Rules {
    input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(": ");
            let rule_name = line_iter
                .next()
                .and_then(|name| name.parse().ok())
                .expect("Expected a integer rule name");
            let rule_options = line_iter
                .next()
                .map(convert_to_rule_options)
                .expect("Expected rule options");
            (rule_name, rule_options)
        })
        .collect()
}

fn convert_to_rule_options(options: &str) -> Vec<RuleOption> {
    // check if the options are a literal
    if options.starts_with("\"") {
        return vec![RuleOption::Literal(
            options.trim_matches('"').chars().next().unwrap(),
        )];
    }

    // or options of chains
    options
        .split("|")
        .map(|option| {
            RuleOption::Chain(
                option
                    .split_whitespace()
                    .map(str::parse)
                    .filter_map(Result::ok)
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::algo::is_isomorphic_matching;

    #[test]
    fn test_parse_rules() {
        let input = "\
            0: 4 1 5\n\
            1: 2 3 | 3 2\n\
            2: 4 4 | 5 5\n\
            3: 4 5 | 5 4\n\
            4: \"a\"\n\
            5: \"b\"";

        let mut expected_rules = Rules::new();
        expected_rules.insert(0, vec![RuleOption::Chain(vec![4, 1, 5])]);
        expected_rules.insert(
            1,
            vec![RuleOption::Chain(vec![2, 3]), RuleOption::Chain(vec![3, 2])],
        );
        expected_rules.insert(
            2,
            vec![RuleOption::Chain(vec![4, 4]), RuleOption::Chain(vec![5, 5])],
        );
        expected_rules.insert(
            3,
            vec![RuleOption::Chain(vec![4, 5]), RuleOption::Chain(vec![5, 4])],
        );
        expected_rules.insert(4, vec![RuleOption::Literal('a')]);
        expected_rules.insert(5, vec![RuleOption::Literal('b')]);

        assert_eq!(parse_rules(input), expected_rules);
    }

    #[test]
    fn test_build_possibility_tree() {
        let mut rules = Rules::new();
        rules.insert(0, vec![RuleOption::Chain(vec![4, 1, 5])]);
        rules.insert(
            1,
            vec![RuleOption::Chain(vec![2, 3]), RuleOption::Chain(vec![3, 2])],
        );
        rules.insert(
            2,
            vec![RuleOption::Chain(vec![4, 4]), RuleOption::Chain(vec![5, 5])],
        );
        rules.insert(
            3,
            vec![RuleOption::Chain(vec![4, 5]), RuleOption::Chain(vec![5, 4])],
        );
        rules.insert(4, vec![RuleOption::Literal('a')]);
        rules.insert(5, vec![RuleOption::Literal('b')]);

        let mut expected_tree = PossibilityTree::new();
        let root = expected_tree.add_node(0);

        // {4} - 1 - 5
        let a = expected_tree.add_node(0);
        expected_tree.add_edge(root, a, 'a');

        // 4 - ({2} - 3) | (3 - 2) - 5
        let aa_2 = expected_tree.add_node(0);
        expected_tree.add_edge(a, aa_2, 'a');
        let ab_2 = expected_tree.add_node(0);
        expected_tree.add_edge(a, ab_2, 'b');
        let axx_2 = expected_tree.add_node(0);
        expected_tree.add_edge(aa_2, axx_2, 'a');
        expected_tree.add_edge(ab_2, axx_2, 'b');

        // 4 - (2 - 3) | ({3} - 2) - 5
        let aa_3 = expected_tree.add_node(0);
        expected_tree.add_edge(a, aa_3, 'a');
        let ab_3 = expected_tree.add_node(0);
        expected_tree.add_edge(a, ab_3, 'b');
        let axx_3 = expected_tree.add_node(0);
        expected_tree.add_edge(aa_3, axx_3, 'b');
        expected_tree.add_edge(ab_3, axx_3, 'a');

        // 4 - (2 - {3}) | (3 - 2) - 5
        let axxa_3 = expected_tree.add_node(0);
        expected_tree.add_edge(axx_2, axxa_3, 'a');
        let axxb_3 = expected_tree.add_node(0);
        expected_tree.add_edge(axx_2, axxb_3, 'b');

        // 4 - (2 - 3) | (3 - {2}) - 5
        let axxa_2 = expected_tree.add_node(0);
        expected_tree.add_edge(axx_3, axxa_2, 'a');
        let axxb_2 = expected_tree.add_node(0);
        expected_tree.add_edge(axx_3, axxb_2, 'b');

        let axxxx = expected_tree.add_node(0);
        expected_tree.add_edge(axxa_3, axxxx, 'b');
        expected_tree.add_edge(axxb_3, axxxx, 'a');
        expected_tree.add_edge(axxa_2, axxxx, 'a');
        expected_tree.add_edge(axxb_2, axxxx, 'b');

        // 4 - 1 - {5}
        let axxxxb = expected_tree.add_node(0);
        expected_tree.add_edge(axxxx, axxxxb, 'b');

        let (actual_tree, _) = build_possibility_tree(&rules);
        assert!(is_isomorphic_matching(
            &actual_tree,
            &expected_tree,
            |_, _| true,
            |edge_a, edge_b| edge_a == edge_b,
        ));
    }
}
