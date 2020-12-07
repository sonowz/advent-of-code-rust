extern crate derive_more;

extern crate petgraph;
use petgraph::graph::NodeIndex;
use petgraph::visit;
use petgraph::Graph;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, space0, space1};
use nom::multi::separated_list0;
use nom::{sequence, IResult};

use std::collections::HashMap;
use std::path::Path;

use advent_of_code_rust_2020 as aoc;

// Type Declarations //

type BagColor = String;
#[derive(Debug, Clone)]
struct Rule {
    bag: BagColor,
    containments: Vec<(BagColor, i32)>,
}

type BagGraph = Graph<BagColor, i32>;

// Part 1 //

fn solve1(rules: &Vec<Rule>) -> i32 {
    let mut graph = rules_to_graph(rules);
    let start_index = shiny_gold_index(&graph);

    // Do DFS with reversed edges,
    // then the footprint will be the answer
    graph.reverse();
    let mut count = 0;
    let mut dfs = visit::Dfs::new(&graph, start_index);
    dfs.next(&graph); // Skip "shiny gold" itself
    while let Some(_) = dfs.next(&graph) {
        count = count + 1;
    }
    count
}

fn rules_to_graph(rules: &Vec<Rule>) -> BagGraph {
    let mut graph = BagGraph::new();
    let mut node_map = HashMap::<&str, NodeIndex>::new();
    rules.into_iter().for_each(|rule| {
        let start = rule.bag.as_str();
        // Try create start node
        node_map
            .entry(start)
            .or_insert(graph.add_node(String::from(start)));
        (&rule.containments).into_iter().for_each(|(end, weight)| {
            let end = end.as_str();
            // Try create end node
            node_map
                .entry(end)
                .or_insert(graph.add_node(String::from(end)));
            // Create edge
            graph.add_edge(node_map[start], node_map[end], *weight);
        })
    });
    graph
}

fn shiny_gold_index(graph: &BagGraph) -> NodeIndex {
    graph
        .node_indices()
        .find(|i| graph[*i] == "shiny gold")
        .unwrap()
}

// Part 2 //

fn solve2(rules: &Vec<Rule>) -> i32 {
    let graph = rules_to_graph(rules);
    let start_index = shiny_gold_index(&graph);

    // "Shiny gold" bag itself is not counted
    count_bags(&graph, start_index) - 1
}

// DFS while counting bags
fn count_bags(graph: &BagGraph, i: NodeIndex) -> i32 {
    let self_count = 1;
    let inner_count: i32 = graph
        .neighbors(i)
        .map(|j| {
            let bag_mult = graph
                .edge_weight(graph.find_edge(i, j).unwrap())
                .unwrap_or(&0);
            (*bag_mult) * count_bags(graph, j)
        })
        .sum();
    self_count + inner_count
}

// I/O //

fn main() {
    let lines = aoc::io::read_file_line(Path::new("inputs/day07.txt"));
    let rules: Vec<Rule> = lines
        .map(|s| aoc::nom::unwrap_parsed(parse_rule(&s)))
        .collect();
    println!("{}", solve1(&rules));
    println!("{}", solve2(&rules));
}

fn parse_bag(input: &str) -> IResult<&str, BagColor> {
    let mut bag_color = String::new();
    let mut i = input;
    loop {
        let (ii, word) = sequence::terminated(alpha1, space0)(i)?;
        i = ii;
        if word == "bag" || word == "bags" {
            break;
        }
        if bag_color != "" {
            bag_color = bag_color + " ";
        }
        bag_color = bag_color + word;
    }
    Ok((i, bag_color))
}

fn parse_contained_bag(input: &str) -> IResult<&str, (BagColor, i32)> {
    sequence::tuple((aoc::nom::number, space1, parse_bag))(input)
        .map(|(input, (n, _, bag_color))| (input, (bag_color, n)))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, bag) = parse_bag(input)?;
    let (input, _) = sequence::tuple((tag("contain"), space1))(input)?;
    let (_, contains) =
        separated_list0(sequence::pair(char(','), space1), parse_contained_bag)(input)?;
    Ok((
        "", // Ignore remaining inputs
        Rule {
            bag: bag,
            containments: contains,
        },
    ))
}
