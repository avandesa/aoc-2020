use std::collections::{HashMap, HashSet};
use {
    itertools::Itertools,
    petgraph::{
        graph::{DiGraph, NodeIndex},
        Direction,
    },
};

pub fn part1(input: &str) -> u32 {
    let mut ratings: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();
    ratings.sort_unstable();
    let mut diffs_of_1 = ratings[0];
    let mut diffs_of_3 = 1;

    for (lower, upper) in ratings.iter().tuple_windows() {
        let diff = upper - lower;
        assert!(diff == 1 || diff == 3);

        if diff == 1 {
            diffs_of_1 += 1
        } else {
            diffs_of_3 += 1
        }
    }

    diffs_of_1 * diffs_of_3
}

fn find_node_value(
    node: NodeIndex<u32>,
    graph: &DiGraph<u32, u32>,
    found_values: &mut HashMap<NodeIndex<u32>, u64>,
) -> u64 {
    let val = graph
        .neighbors_directed(node, Direction::Outgoing)
        .map(|neighbor_index| {
            // There's probably a more elegant solution with `HashMap::entry`,
            // but I ran into some liftime issues I don't care to fix right now.
            if let Some(value) = found_values.get(&neighbor_index) {
                // We've already calculated the value of this node
                *value
            } else {
                // Recurse
                let value = find_node_value(neighbor_index, graph, found_values);
                found_values.insert(neighbor_index, value);

                value
            }
        })
        .sum();

    // This would produce 0 for the max adapter, so default to 1
    std::cmp::max(1, val)
}

pub fn part2(input: &str) -> u64 {
    // For this solution, we build a graph of the adapters. We define the
    // solution inductively.
    // - Let the last adapter (that with the highest rating) have a value of 1.
    // - Any other adapter A's value is the sum of the values of the adapters to
    //   which A connects.
    // The number of possible paths through the graph is the value of the
    // adapter of rating 0.

    let mut ratings: HashSet<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

    // 0 and max are not included in the input, so we add them here.
    let max_rating = ratings.iter().max().unwrap() + 3;
    ratings.insert(0);
    ratings.insert(max_rating);

    // Build the graph
    let mut adapter_indices: HashMap<u32, NodeIndex<u32>> = HashMap::new();
    let mut graph: DiGraph<u32, u32> = DiGraph::new();
    for adapter in &ratings {
        let adapter_node = *adapter_indices
            .entry(*adapter)
            .or_insert_with(|| graph.add_node(*adapter));

        for connected_adapter in (1..=3).filter_map(|diff| ratings.get(&(adapter + diff))) {
            // Add an edge from this adapter to another adapter `dist` jolts
            // higher, if present
            let connected_adapter_node = *adapter_indices
                .entry(*connected_adapter)
                .or_insert_with(|| graph.add_node(*connected_adapter));

            graph.add_edge(
                adapter_node,
                connected_adapter_node,
                connected_adapter - adapter,
            );
        }
    }

    let index_of_zero = *adapter_indices.get(&0).unwrap();
    find_node_value(index_of_zero, &graph, &mut HashMap::new())
}
