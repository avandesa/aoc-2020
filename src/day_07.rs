use std::{collections::HashMap, fmt};

use {
    nom::{
        branch::alt,
        bytes::complete::{tag, take_until},
        combinator::map_res,
        multi::separated_list1,
        sequence::{separated_pair, tuple},
        IResult,
    },
    petgraph::{
        graph::{DiGraph, NodeIndex},
        visit::DfsEvent,
        Direction,
    },
};

#[derive(Debug, Hash, Eq, PartialEq)]
struct Bag {
    adjective: String,
    color: String,
}

impl Bag {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (adjective, _, color, _)) = tuple((
            take_until(" "),
            tag(" "),
            take_until(" "),
            alt((tag(" bags"), tag(" bag"))),
        ))(input)?;

        Ok((
            input,
            Self {
                adjective: adjective.to_string(),
                color: color.to_string(),
            },
        ))
    }
}

#[derive(Debug)]
struct Constraint {
    number: u32,
    bag: Bag,
}

impl Constraint {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (number, bag)) = separated_pair(
            map_res(take_until(" "), |num| u32::from_str_radix(num, 10)),
            tag(" "),
            Bag::parse,
        )(input)?;

        Ok((input, Self { number, bag }))
    }

    pub fn parse_no_other(input: &str) -> IResult<&str, &str> {
        tag("no other bags")(input)
    }

    pub fn parse_list(input: &str) -> IResult<&str, Vec<Self>> {
        if let Ok((input, _)) = Self::parse_no_other(input) {
            Ok((input, vec![]))
        } else {
            separated_list1(tag(", "), Self::parse)(input)
        }
    }
}

#[derive(Debug)]
struct Rule {
    bag: Bag,
    constraints: Vec<Constraint>,
}

impl Rule {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (bag, constraints)) =
            separated_pair(Bag::parse, tag(" contain "), Constraint::parse_list)(input)?;

        let (input, _) = tag(".")(input)?;

        Ok((input, Self { bag, constraints }))
    }
}

pub fn part1(input: &str) -> u32 {
    let rules: Vec<Rule> = input
        .lines()
        .map(|line| Rule::parse(line).unwrap().1)
        .collect();

    let mut bag_node_indices = HashMap::new();
    let mut contained_by_graph = DiGraph::<&Bag, u32>::new();

    for bag in rules.iter().map(|rule| &rule.bag) {
        let node_index = contained_by_graph.add_node(bag);
        bag_node_indices.insert(bag, node_index);
    }

    for rule in &rules {
        for constraint in &rule.constraints {
            let container_index = bag_node_indices.get(&rule.bag).copied().unwrap();
            let containee_index = bag_node_indices.get(&constraint.bag).copied().unwrap();
            contained_by_graph.add_edge(containee_index, container_index, constraint.number);
        }
    }

    let shiny_gold = Bag {
        adjective: "shiny".to_string(),
        color: "gold".to_string(),
    };
    let shiny_gold_node = bag_node_indices.get(&shiny_gold).unwrap();

    let mut visited = 0;
    petgraph::visit::depth_first_search(&contained_by_graph, Some(*shiny_gold_node), |event| {
        if let DfsEvent::Discover(node, _) = event {
            if node != *shiny_gold_node {
                visited += 1;
            }
        }
    });

    visited
}

pub fn part2(input: &str) -> u32 {
    let rules: Vec<Rule> = input
        .lines()
        .map(|line| Rule::parse(line).unwrap().1)
        .collect();

    let mut bag_node_indices = HashMap::new();
    let mut contains_graph = DiGraph::<&Bag, u32>::new();

    for bag in rules.iter().map(|rule| &rule.bag) {
        let node_index = contains_graph.add_node(bag);
        bag_node_indices.insert(bag, node_index);
    }

    for rule in &rules {
        for constraint in &rule.constraints {
            let container_index = bag_node_indices.get(&rule.bag).copied().unwrap();
            let containee_index = bag_node_indices.get(&constraint.bag).copied().unwrap();
            contains_graph.add_edge(container_index, containee_index, constraint.number);
        }
    }

    let shiny_gold = Bag {
        adjective: "shiny".to_string(),
        color: "gold".to_string(),
    };
    let shiny_gold_node = bag_node_indices.get(&shiny_gold).unwrap();

    // We don't want to include the shiny gold bag itself, so subtract 1.
    calc_required_bags(*shiny_gold_node, &mut HashMap::new(), &contains_graph) - 1
}

fn calc_required_bags(
    node_index: NodeIndex,
    found_costs: &mut HashMap<NodeIndex, u32>,
    graph: &DiGraph<&Bag, u32>,
) -> u32 {
    graph
        .neighbors_directed(node_index, Direction::Outgoing)
        .map(|neighbor_index| {
            if let Some(cost) = found_costs.get(&neighbor_index) {
                // We've already calculated the total number of bags this neighbor requires.
                let edge = graph.find_edge(node_index, neighbor_index).unwrap();
                let num_required = &graph[edge];

                cost * num_required
            } else {
                // Recurse
                let cost = calc_required_bags(neighbor_index, found_costs, graph);
                found_costs.insert(neighbor_index, cost);

                let edge = graph.find_edge(node_index, neighbor_index).unwrap();
                let num_required = &graph[edge];

                cost * num_required
            }
        })
        .sum::<u32>()
        + 1
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.adjective, self.color)
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} bag", self.number, self.bag)
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} bags contain:", self.bag)?;

        if self.constraints.is_empty() {
            writeln!(f, "\tNo other bags")?;
        } else {
            for constraint in &self.constraints {
                writeln!(f, "\t{}", constraint)?;
            }
        }

        Ok(())
    }
}
