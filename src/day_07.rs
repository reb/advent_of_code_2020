/// --- Day 7: Handy Haversacks ---
///
/// You land at the regional airport in time for your next flight. In fact, it
/// looks like you'll even have time to grab some food: all flights are
/// currently delayed due to issues in luggage processing.
///
/// Due to recent aviation regulations, many rules (your puzzle input) are being
/// enforced about bags and their contents; bags must be color-coded and must
/// contain specific quantities of other color-coded bags. Apparently, nobody
/// responsible for these regulations considered how long they would take to
/// enforce!
///
/// For example, consider the following rules:
///
/// light red bags contain 1 bright white bag, 2 muted yellow bags.
/// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
/// bright white bags contain 1 shiny gold bag.
/// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
/// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
/// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
/// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
/// faded blue bags contain no other bags.
/// dotted black bags contain no other bags.
///
/// These rules specify the required contents for 9 bag types. In this example,
/// every faded blue bag is empty, every vibrant plum bag contains 11 bags (5
/// faded blue and 6 dotted black), and so on.
///
/// You have a shiny gold bag. If you wanted to carry it in at least one other
/// bag, how many different bag colors would be valid for the outermost bag? (In
/// other words: how many colors can, eventually, contain at least one shiny
/// gold bag?)
///
/// In the above rules, the following options would be available to you:
///
///     A bright white bag, which can hold your shiny gold bag directly.
///     A muted yellow bag, which can hold your shiny gold bag directly, plus
///     some other bags.
///     A dark orange bag, which can hold bright white and muted yellow bags,
///     either of which could then hold your shiny gold bag.
///     A light red bag, which can hold bright white and muted yellow bags,
///     either of which could then hold your shiny gold bag.
///
/// So, in this example, the number of bag colors that can eventually contain at
/// least one shiny gold bag is 4.
///
/// How many bag colors can eventually contain at least one shiny gold bag? (The
/// list of rules is quite long; make sure you get all of it.)
///
/// --- Part Two ---
///
/// It's getting pretty expensive to fly these days - not because of ticket
/// prices, but because of the ridiculous number of bags you need to buy!
///
/// Consider again your shiny gold bag and the rules from the above example:
///
///     faded blue bags contain 0 other bags.
///     dotted black bags contain 0 other bags.
///     vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted
///     black bags.
///     dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted
///     black bags.
///
/// So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags
/// within it) plus 2 vibrant plum bags (and the 11 bags within each of those):
/// 1 + 1*7 + 2 + 2*11 = 32 bags!
///
/// Of course, the actual rules have a small chance of going several levels
/// deeper than this example; be sure to count all of the bags, even if the
/// nesting becomes topologically impractical!
///
/// Here's another example:
///
/// shiny gold bags contain 2 dark red bags.
/// dark red bags contain 2 dark orange bags.
/// dark orange bags contain 2 dark yellow bags.
/// dark yellow bags contain 2 dark green bags.
/// dark green bags contain 2 dark blue bags.
/// dark blue bags contain 2 dark violet bags.
/// dark violet bags contain no other bags.
///
/// In this example, a single shiny gold bag must contain 126 other bags.
///
/// How many individual bags are required inside your single shiny gold bag?
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::{Bfs, Reversed};
use petgraph::Direction::Outgoing;
use regex::Regex;

const INPUT: &str = include_str!("../input/day_07.txt");

pub fn run() {
    let rule_graph = parse_bag_rules(INPUT);

    let mut bfs = Bfs::new(Reversed(&rule_graph), "shiny gold");
    let mut bags_above = 0;
    while let Some(_) = bfs.next(Reversed(&rule_graph)) {
        bags_above += 1;
    }
    bags_above -= 1; // remove the shiny gold bag
    println!(
        "The amount of bag colors that can contain a shiny gold bag is: {}",
        bags_above
    );

    let containing_bags = count_containing_bags(&rule_graph, "shiny gold");
    println!(
        "The amount of individual bags required inside a single shiny gold bag is: {}",
        containing_bags
    );
}

type Graph<'a> = DiGraphMap<&'a str, u32>;

fn count_containing_bags(graph: &Graph, start: &str) -> u32 {
    graph
        .neighbors_directed(start, Outgoing)
        .map(|next| {
            let weight = *graph.edge_weight(start, next).unwrap();
            weight * (1 + count_containing_bags(graph, next))
        })
        .sum()
}

fn parse_bag_rules(input: &str) -> Graph {
    let weighted_edges: Vec<(&str, &str, u32)> = input
        .lines()
        .flat_map(|line| convert_to_edges(line))
        .collect();
    Graph::from_edges(weighted_edges)
}

fn convert_to_edges(line: &str) -> Vec<(&str, &str, u32)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]*) ?([a-z]+ [a-z]+) bag").unwrap();
    }
    let mut captures = RE.captures_iter(line);
    let color = captures
        .next()
        .expect("No first match in line")
        .get(2)
        .expect("No first color")
        .as_str();

    captures
        .filter_map(|groups| {
            match (
                groups.get(1).unwrap().as_str().parse(),
                groups.get(2).unwrap().as_str(),
            ) {
                (Ok(amount), connecting_color) => Some((color, connecting_color, amount)),
                _ => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph_eq(a: &Graph, b: &Graph) -> bool {
        a.nodes().eq(b.nodes()) && a.all_edges().eq(b.all_edges())
    }

    #[test]
    fn test_parse_bag_rules() {
        let input = "\
            light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
            bright white bags contain 1 shiny gold bag.\n\
            dotted black bags contain no other bags.";

        let expected_graph = Graph::from_edges(&[
            ("light red", "bright white", 1),
            ("light red", "muted yellow", 2),
            ("dark orange", "bright white", 3),
            ("dark orange", "muted yellow", 4),
            ("bright white", "shiny gold", 1),
        ]);

        let resulting_graph = parse_bag_rules(input);
        assert!(
            graph_eq(&resulting_graph, &expected_graph),
            format!(
                "Graphs are not equal\n\nresult: {:?}\n\nexpected: {:?}\n",
                resulting_graph, expected_graph
            )
        );
    }

    #[test]
    fn test_count_containing_bags_simple() {
        /// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        /// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        /// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        /// faded blue bags contain no other bags.
        /// dotted black bags contain no other bags.
        ///
        /// So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags
        /// within it) plus 2 vibrant plum bags (and the 11 bags within each of those):
        /// 1 + 1*7 + 2 + 2*11 = 32 bags!
        let graph = Graph::from_edges(&[
            ("shiny gold", "dark olive", 1),
            ("shiny gold", "vibrant plum", 2),
            ("dark olive", "faded blue", 3),
            ("dark olive", "dotted black", 4),
            ("vibrant plum", "faded blue", 5),
            ("vibrant plum", "dotted black", 6),
        ]);

        assert_eq!(count_containing_bags(&graph, "shiny gold"), 32);
    }

    #[test]
    fn test_count_containing_bags_deep() {
        /// shiny gold bags contain 2 dark red bags.
        /// dark red bags contain 2 dark orange bags.
        /// dark orange bags contain 2 dark yellow bags.
        /// dark yellow bags contain 2 dark green bags.
        /// dark green bags contain 2 dark blue bags.
        /// dark blue bags contain 2 dark violet bags.
        /// dark violet bags contain no other bags.
        ///
        /// In this example, a single shiny gold bag must contain 126 other bags.
        let graph = Graph::from_edges(&[
            ("shiny gold", "dark red", 2),
            ("dark red", "dark orange", 2),
            ("dark orange", "dark yellow", 2),
            ("dark yellow", "dark green", 2),
            ("dark green", "dark blue", 2),
            ("dark blue", "dark violet", 2),
        ]);

        assert_eq!(count_containing_bags(&graph, "shiny gold"), 126);
    }
}
