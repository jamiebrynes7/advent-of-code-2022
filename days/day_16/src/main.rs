use std::collections::{HashMap, HashSet};
use utils::parse_lines;

mod raw;

fn main() {
    let input = include_str!("data.txt");
    let valves: Vec<raw::Valve> = parse_lines(input).unwrap();
    let flattened = FlattenedGraph::from_valves(valves);

    part1(&flattened);
    part2(&flattened);
}

#[derive(Debug, Clone)]
struct Traversal<'a> {
    visited: HashSet<&'a String>,
    released_pressure: u32,
    minute: u32,

    limit: u32,
}

impl<'a> Traversal<'a> {
    pub fn new(limit: u32) -> Traversal<'a> {
        Self {
            visited: HashSet::new(),
            released_pressure: 0,
            minute: 0,

            limit,
        }
    }

    pub fn step(&mut self, count: u32) {
        self.minute += count;
    }

    pub fn is_done(&self) -> bool {
        self.minute >= self.limit
    }

    pub fn visit_node(&mut self, node: &'a FlatNode) {
        self.visited.insert(&node.ident);
        self.released_pressure += node.flow_rate * (self.limit - self.minute);
    }
}

fn part1(flattened: &FlattenedGraph) {
    let mut max: u32 = 0;
    let mut traversals = vec![(&flattened.start, Traversal::new(30))];

    while !traversals.is_empty() {
        let (node, traversal) = traversals.remove(0);

        for (ident, cost) in &node.connections {
            if traversal.visited.contains(ident) {
                continue;
            }

            let mut new = traversal.clone();
            new.step(cost + 1);

            if new.is_done() {
                continue;
            }

            let next_node = flattened.nodes.get(ident).unwrap();
            new.visit_node(next_node);
            traversals.push((next_node, new));
        }

        if traversal.released_pressure > max {
            max = traversal.released_pressure;
        }
    }

    println!("Part 1 result: {}", max);
}

fn part2(flattened: &FlattenedGraph) {
    let mut finished = vec![];
    let mut traversals = vec![(&flattened.start, Traversal::new(26))];

    while !traversals.is_empty() {
        let (node, traversal) = traversals.remove(0);

        for (ident, cost) in &node.connections {
            if traversal.visited.contains(ident) {
                continue;
            }

            let mut new = traversal.clone();
            new.step(cost + 1);

            if new.is_done() {
                continue;
            }

            let next_node = flattened.nodes.get(ident).unwrap();
            new.visit_node(next_node);
            traversals.push((next_node, new));
        }

        finished.push(traversal);
    }

    // The max possible is the maximum of two paths, where the sets of visited valves are disjoint
    let mut max = 0;
    for i in 0..finished.len() {
        for j in i..finished.len() {
            let first = &finished[i];
            let second = &finished[j];

            if !first.visited.is_disjoint(&second.visited) {
                continue;
            }

            let sum = first.released_pressure + second.released_pressure;

            if sum > max {
                max = sum;
            }
        }
    }

    println!("Part 2 result: {max}");
}

#[derive(Debug)]
struct FlattenedGraph {
    start: FlatNode,
    nodes: HashMap<String, FlatNode>,
}

#[derive(Debug)]
struct FlatNode {
    ident: String,
    flow_rate: u32,
    connections: HashMap<String, u32>,
}

impl FlattenedGraph {
    pub fn from_valves(valves: Vec<raw::Valve>) -> Self {
        let valves_with_flow_rate = valves
            .iter()
            .filter(|v| v.flow_rate != 0)
            .collect::<HashSet<_>>();

        let indexed = valves
            .iter()
            .map(|v| (v.ident.clone(), v))
            .collect::<HashMap<_, _>>();

        fn traverse_node(valve: &raw::Valve, all_nodes: &HashMap<String, &raw::Valve>) -> FlatNode {
            // Do a BFS to each other valve with flow
            let mut to_explore = vec![(0, valve)];
            let mut connections = HashMap::new();

            while !to_explore.is_empty() {
                let (steps, target) = to_explore.remove(0);

                if valve != target && !connections.contains_key(&target.ident) {
                    connections.insert(target.ident.clone(), steps);
                }

                for conn in target
                    .connections
                    .iter()
                    .filter(|conn| !connections.contains_key(*conn))
                {
                    to_explore.push((steps + 1, all_nodes.get(conn).unwrap()));
                }
            }

            FlatNode {
                ident: valve.ident.clone(),
                flow_rate: valve.flow_rate,
                connections: connections
                    .into_iter()
                    .filter(|(ident, _)| all_nodes.get(ident).unwrap().flow_rate > 0)
                    .collect(),
            }
        }

        FlattenedGraph {
            start: traverse_node(indexed.get("AA").unwrap(), &indexed),
            nodes: valves_with_flow_rate
                .into_iter()
                .map(|v| traverse_node(v, &indexed))
                .map(|v| (v.ident.clone(), v))
                .collect(),
        }
    }
}
