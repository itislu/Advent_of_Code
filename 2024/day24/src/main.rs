use std::collections::HashMap;
use utils::{input, parse};

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &str) -> usize {
    let gates: HashMap<String, Gate> = parse_gates(input);

    get_combined_number(&gates, 'z')
}

fn get_combined_number(gates: &HashMap<String, Gate>, gate_letter: char) -> usize {
    let mut num: usize = 0;

    for (key, gate) in gates.iter().filter(|(key, _)| key.starts_with(gate_letter)) {
        num |= (get_value(&gates, key) as usize) << gate.bit_pos().unwrap_or_default();
    }
    num
}

fn get_value(gates: &HashMap<String, Gate>, target: &str) -> u8 {
    match &gates[target] {
        Gate::Input(data) => data.value,
        Gate::Normal(data) => data
            .op
            .calc(get_value(gates, &data.in1), get_value(gates, &data.in2)),
    }
}

enum Gate {
    Input(InputData),
    Normal(GateData),
}

impl Gate {
    fn new(line: &str) -> Gate {
        if line.split_whitespace().count() == 2 {
            Gate::Input(InputData::new(line))
        } else {
            Gate::Normal(GateData::new(line))
        }
    }

    fn name(&self) -> &str {
        match self {
            Gate::Input(data) => data.name.as_str(),
            Gate::Normal(data) => data.out.as_str(),
        }
    }

    fn bit_pos(&self) -> Option<u8> {
        match self {
            Gate::Input(data) => Some(data.bit_pos),
            Gate::Normal(data) => parse::numbers::<u8>(&data.out).next(),
        }
    }
}

struct GateData {
    op: Operator,
    in1: String,
    in2: String,
    out: String,
}

impl GateData {
    fn new(line: &str) -> GateData {
        let mut split = line.split_whitespace();
        GateData {
            in1: split.next().unwrap().to_owned(),
            op: Operator::from(split.next().unwrap()),
            in2: split.next().unwrap().to_owned(),
            out: split.nth(1).unwrap().to_owned(),
        }
    }
}

struct InputData {
    name: String,
    bit_pos: u8,
    value: u8,
}

impl InputData {
    fn new(line: &str) -> InputData {
        let mut split = line.split(": ");
        let name = split.next().unwrap();
        let value = parse::numbers::<u8>(split.next().unwrap()).next().unwrap();
        InputData {
            name: name.to_owned(),
            bit_pos: parse::numbers(name).next().unwrap(),
            value,
        }
    }
}

enum Operator {
    AND,
    OR,
    XOR,
}

impl Operator {
    fn calc(&self, in1: u8, in2: u8) -> u8 {
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

fn parse_gates(input: &str) -> HashMap<String, Gate> {
    let mut gates: HashMap<String, Gate> = HashMap::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        let gate = Gate::new(line);
        gates.insert(gate.name().to_owned(), gate);
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
