use std::collections::HashMap;
use utils::{input, parse};

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &str) -> usize {
    let mut res: usize = 0;
    let mut gates: HashMap<&str, Gate> = parse_gates(input);
    let mut targets: Vec<&str> = gates
        .keys()
        .filter(|&key| key.starts_with('z'))
        .copied()
        .collect();
    targets.sort();

    for (i, target) in targets.iter().enumerate() {
        res |= (get_out(&mut gates, target) as usize) << i;
    }
    res
}

fn get_out(gates: &mut HashMap<&str, Gate>, target: &str) -> u8 {
    match gates[target].clone() {
        Gate::Out(out) => out,
        Gate::In((in1, op, in2)) => op.out(get_out(gates, &in1), get_out(gates, &in2)),
    }
}

#[derive(Clone)]
enum Operator {
    AND,
    OR,
    XOR,
}

impl Operator {
    fn out(&self, in1: u8, in2: u8) -> u8 {
        match self {
            Operator::AND => in1 & in2,
            Operator::OR => in1 | in2,
            Operator::XOR => in1 ^ in2,
        }
    }
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Operator::AND,
            "OR" => Operator::OR,
            "XOR" => Operator::XOR,
            _ => panic!("Invalid operator found!"),
        }
    }
}

#[derive(Clone)]
enum Gate {
    In((String, Operator, String)),
    Out(u8),
}

fn parse_gates(input: &str) -> HashMap<&str, Gate> {
    let mut gates: HashMap<&str, Gate> = HashMap::new();
    let (values, prerequisites) = input.split_once("\n\n").unwrap();

    for line in values.lines() {
        let mut split = line.split(": ");
        let out = split.next().unwrap();
        let value = parse::numbers::<u8>(split.next().unwrap()).next().unwrap();
        gates.insert(out, Gate::Out(value));
    }
    for line in prerequisites.lines() {
        let mut split = line.split_whitespace();
        let in1 = split.next().unwrap();
        let op = Operator::from(split.next().unwrap());
        let in2 = split.next().unwrap();
        let out = split.nth(1).unwrap();
        gates.insert(out, Gate::In((in1.to_owned(), op, in2.to_owned())));
    }
    gates
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 2024);
    }
}
