//! --- Day 7: Handy Haversacks ---
//! https://adventofcode.com/2020/day/7

use petgraph::{dot::Dot, graph::NodeIndex, visit::EdgeRef, Directed, Direction, Graph};
use std::{
    collections::{HashMap, HashSet},
    iter::Extend,
};

const INPUT: &str = include_str!("../../inputs/day07.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> usize {
    let target = "shiny gold";
    let tree = build_tree(input);

    tree.count_parents(target)
}

fn part_two(input: &str) -> usize {
    let target = "shiny gold";
    let tree = build_tree(input);

    tree.count_contained(target)
}

fn build_tree(input: &str) -> Tree {
    let mut tree = Tree::default();

    for line in input.lines() {
        let mut iter = line.split(" bags contain ");
        let parent = iter.next().unwrap().trim();

        for bag in iter.next().unwrap().trim_end_matches('.').split(", ") {
            let bag = bag
                .trim_end_matches(" bag")
                .trim_end_matches(" bags")
                .trim();
            let i = bag.find(' ').unwrap();
            match bag {
                "no other" => {
                    tree.add_node(&parent);
                }
                b => {
                    let (count, name) = b.split_at(i);
                    let count = count.trim().parse::<u16>().unwrap();
                    let name = name.trim();
                    tree.add_edge(parent, name, count);
                }
            }
        }
    }

    tree
}

#[derive(Debug, Default)]
struct Tree {
    nodes: HashMap<String, NodeIndex>,
    graph: Graph<String, u16, Directed>,
}

impl Tree {
    pub fn add_node(&mut self, name: &str) {
        if !self.nodes.contains_key(name) {
            let node = self.graph.add_node(name.into());
            self.nodes.insert(name.into(), node);
        }
    }

    pub fn get_node(&mut self, name: &str) -> NodeIndex {
        self.add_node(name);
        *self.nodes.get(name).unwrap()
    }

    pub fn add_edge(&mut self, parent: &str, child: &str, count: u16) {
        let parent = self.get_node(parent);
        let child = self.get_node(child);

        self.graph.add_edge(parent, child, count);
    }

    pub fn count_parents(&self, name: &str) -> usize {
        let node = self.nodes[name];
        let mut found: HashSet<NodeIndex> = HashSet::new();
        let mut todo = HashSet::new();
        todo.insert(node);

        while !todo.is_empty() {
            found.extend(todo.iter());
            todo = todo
                .iter()
                .flat_map(|n| {
                    self.graph
                        .edges_directed(*n, Direction::Incoming)
                        .map(|e| e.source())
                })
                .filter(|n| !found.contains(n))
                .collect();
        }
        found.remove(&node);
        found.len()
    }

    pub fn count_contained(&self, name: &str) -> usize {
        fn walk_children(
            index: NodeIndex,
            graph: &Graph<String, u16, Directed>,
            map: &mut HashMap<NodeIndex, usize>,
        ) -> usize {
            let mut total = 1;
            for edge in graph.edges_directed(index, Direction::Outgoing) {
                let weight = edge.weight();
                let index = edge.target();
                match map.get(&index) {
                    Some(count) => total += count * (*weight as usize),
                    None => {
                        let count = walk_children(index, &graph, map);
                        total += count * (*weight as usize);
                        map.insert(index, count);
                    }
                }
            }

            total
        }

        let mut map = HashMap::new();
        let node = self.nodes[name];
        walk_children(node, &self.graph, &mut map) - 1
    }

    pub fn dot(&self) {
        println!("{:?}", Dot::new(&self.graph));
    }
}

impl Tree {}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT_ONE: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const INPUT_TWO: &str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn part_one_works() {
        let res = part_one(INPUT_ONE);
        assert_eq!(4, res);
    }

    #[test]
    fn part_two_works() {
        let res = part_two(INPUT_TWO);
        assert_eq!(126, res);
    }
}
