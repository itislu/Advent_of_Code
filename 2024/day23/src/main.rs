use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

/*
for each K:
    for each V:
        is any other V in values of V?
*/
fn exercise1(input: &str) -> usize {
    let network = Network::new(input);

    network
        .get_threeway_connections()
        .filter(|con| con.iter().any(|node| node.starts_with("t")))
        .count()
}

struct Network {
    connections: HashMap<String, HashSet<String>>,
}

impl Network {
    fn new(input: &str) -> Network {
        let mut network = Network {
            connections: HashMap::new(),
        };

        for line in input.lines() {
            let split: Vec<&str> = line.split('-').collect();
            network.add_twoway_connection(split[0], split[1]);
            network.add_twoway_connection(split[1], split[0]);
        }
        network
    }

    fn add_oneway_connection(&mut self, from: &str, to: &str) {
        if let Some(entry) = self.connections.get_mut(from) {
            entry.insert(to.to_string());
        } else {
            let mut set: HashSet<String> = HashSet::new();
            set.insert(to.to_string());
            self.connections.insert(from.to_string(), set);
        }
    }

    fn add_twoway_connection(&mut self, node1: &str, node2: &str) {
        self.add_oneway_connection(node1, node2);
        self.add_oneway_connection(node2, node1);
    }

    fn get_threeway_connections(&self) -> impl Iterator<Item = Vec<&String>> {
        self.connections
            .keys()
            .flat_map(|key| {
                self.connections[key]
                    .iter()
                    .combinations(2)
                    .filter_map(move |pair| {
                        if self.connections[pair[0]].contains(pair[1]) {
                            let mut three_way_connection = vec![key, pair[0], pair[1]];
                            three_way_connection.sort();
                            Some(three_way_connection)
                        } else {
                            None
                        }
                    })
            })
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 7);
    }
}
