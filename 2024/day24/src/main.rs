use std::collections::HashMap;
use utils::{input, parse};

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let gates: HashMap<String, Gate> = parse_gates(input);

    get_combined_number(&gates, 'z')
}

fn exercise2(input: &str) -> &str {
    let gates: HashMap<String, Gate> = parse_gates(input);

    let x = get_combined_number(&gates, 'x');
    let y = get_combined_number(&gates, 'y');
    let expected = x + y;
    let z = get_combined_number(&gates, 'z');
    println!(" x gates: {}", x);
    println!(" y gates: {}", y);
    println!("expected: {}", expected);
    println!(" z gates: {}", z);
    compare_bits(expected, z);

    ""
}

fn compare_bits(expected: usize, actual: usize) {
    for i in 0..usize::BITS {
        let expected_bit = (expected >> i) & 1;
        let actual_bit = (actual >> i) & 1;
        if expected_bit != actual_bit {
            println!(
                "Bit {:2} differs - expected: {}, actual: {}",
                i, expected_bit, actual_bit
            );
        }
    }

    println!("expected: {:b}", expected);
    println!("  actual: {:b}", actual);
}

// I know that the OUTPUT wires have been swapped on gates, NOT input wires!

/*
z00 XOR
    x00
    y00

z01 XOR
    XOR
        x01
        y01
    AND
        x00
        y00

zn XOR
    XOR
        xn
        yn
    OR
        AND
            xn-1
            yn-1
        AND
            same as zn-1
*/

/*
- If an input name for XOR and AND contains digit { half1 } else { half2 }
*/

struct HalfAdder {
    bit: u8,
    in1: String,
    in2: String,
    sum: String,
    carry: String,
}

struct CarryOut {
    in1: String,
    in2: String,
    out: String,
}

/*
FullAdder:
- carry_in from prev adder should be same as 1 in for half2
- sum from half1 should be 1 in for half2
- sum from half2 should be sum of FullAdder
- carry from half1 should be 1 for carry_out
- carry from half2 should be 1 for carry_out

- There always is a pair of XOR and AND with same inputs belonging to same bit.
    - Both ANDs output to the same OR.
*/
struct FullAdder {
    bit: u8,
    in1: String,
    in2: String,
    sum: String,
    half1: HalfAdder,
    half2: HalfAdder,
    carry_in: String,
    carry_out: String,
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
