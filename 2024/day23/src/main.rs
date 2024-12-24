use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

/*
for each K:
    for each V:
        is any other V in values of V?
*/
fn exercise1(input: &str) -> usize {
    let network = parse_network(input);

    get_threeway_connections(&network)
        .filter(|con| con.iter().any(|node| node.starts_with("t")))
        .count()
}

/*
Find maximal clique for each node.
Keep it if it is bigger than the biggest so far.
*/
fn exercise2(input: &str) -> String {
    let network = parse_network(input);

    bron_kerbosch(&network, HashSet::new(), network.keys().copied().collect())
        .iter()
        .sorted()
        .join(",")
}

fn bron_kerbosch<'a>(
    network: &'a HashMap<&'a str, HashSet<&'a str>>,
    clique: HashSet<&'a str>,
    mut candidates: HashSet<&'a str>,
) -> HashSet<&'a str> {
    if candidates.is_empty() {
        return clique;
    }

    let mut max_clique = clique.clone();

    while let Some(&candidate) = candidates.iter().next() {
        candidates.remove(candidate);

        let mut new_clique = clique.clone();
        new_clique.insert(candidate);

        let result = bron_kerbosch(
            network,
            new_clique,
            candidates
                .intersection(&network[candidate])
                .copied()
                .collect(),
        );
        if result.len() > max_clique.len() {
            max_clique = result;
        }
    }

    max_clique
}

fn parse_network(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut network = HashMap::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split('-').collect();
        add_twoway_connection(&mut network, split[0], split[1]);
        add_twoway_connection(&mut network, split[1], split[0]);
    }
    network
}

fn add_oneway_connection<'a>(
    network: &mut HashMap<&'a str, HashSet<&'a str>>,
    from: &'a str,
    to: &'a str,
) {
    if let Some(entry) = network.get_mut(from) {
        entry.insert(to);
    } else {
        let mut set: HashSet<&str> = HashSet::new();
        set.insert(to);
        network.insert(from, set);
    }
}

fn add_twoway_connection<'a>(
    network: &mut HashMap<&'a str, HashSet<&'a str>>,
    node1: &'a str,
    node2: &'a str,
) {
    add_oneway_connection(network, node1, node2);
    add_oneway_connection(network, node2, node1);
}

fn get_threeway_connections<'a>(
    network: &HashMap<&'a str, HashSet<&'a str>>,
) -> impl Iterator<Item = Vec<&'a str>> {
    network
        .keys()
        .flat_map(|&key| {
            network[key].iter().combinations(2).filter_map(move |pair| {
                if network[pair[0]].contains(pair[1]) {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_ex2() {
        let input = input::read_example();
        let res = exercise2(&input);
        assert_eq!(res, "co,de,ka,ta");
    }
}
