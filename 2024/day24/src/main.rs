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

fn exercise2(input: &str) -> String {
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

    let mut bad_gates: Vec<String> = collect_bad_gates(&gates)
        .iter()
        .map(|gate| gate.out.clone())
        .collect();
    bad_gates.sort();
    bad_gates.join(",")
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

fn collect_bad_gates(gates: &HashMap<String, Gate>) -> Vec<&GateData> {
    let bad_xor_gates = collect_bad_xor_gates(gates);
    let bad_and_gates = collect_bad_and_gates(gates);
    let bad_or_gates = collect_bad_or_gates(gates);

    println!("\nBAD XOR GATES:");
    for gate in &bad_xor_gates {
        println!("{:?}", gate);
    }
    println!("\nBAD AND GATES:");
    for gate in &bad_and_gates {
        println!("{:?}", gate);
    }
    println!("\nBAD OR GATES:");
    for gate in &bad_or_gates {
        println!("{:?}", gate);
    }

    bad_xor_gates
        .iter()
        .copied()
        .chain(bad_and_gates.iter().copied())
        .chain(bad_or_gates.iter().copied())
        .collect()
}

// Either both inputs contain digit, or output contains digit (except 00)
fn collect_bad_xor_gates(gates: &HashMap<String, Gate>) -> Vec<&GateData> {
    let mut bad_gates = Vec::new();

    for gate in gates.iter().filter_map(|(_, gate)| match gate {
        Gate::Normal(data) if data.op == Operator::XOR => Some(data),
        _ => None,
    }) {
        if gate.in1.ends_with("00") && gate.in2.ends_with("00") {
            if !gate.out.ends_with("00") {
                bad_gates.push(gate);
            }
            continue;
        }
        let in1_digit = gate.in1.chars().any(|c| c.is_digit(10));
        let in2_digit = gate.in2.chars().any(|c| c.is_digit(10));
        let out_digit = gate.out.chars().any(|c| c.is_digit(10));
        if !((in1_digit && in2_digit && !out_digit) || (out_digit && !in1_digit && !in2_digit)) {
            bad_gates.push(gate);
        }
    }
    bad_gates
}

// Output to OR (except if inputs are 00)
fn collect_bad_and_gates(gates: &HashMap<String, Gate>) -> Vec<&GateData> {
    let mut bad_gates = Vec::new();

    for gate in gates.iter().filter_map(|(_, gate)| match gate {
        Gate::Normal(data) if data.op == Operator::AND => Some(data),
        _ => None,
    }) {
        let outputs = outputs_to(gates, &gate.out).collect::<Vec<&GateData>>();
        if gate.in1.ends_with("00") && gate.in2.ends_with("00") {
            if outputs.len() != 2
                || count_operator(&outputs, Operator::XOR) != 1
                || count_operator(&outputs, Operator::AND) != 1
            {
                bad_gates.push(gate);
            }
            continue;
        }
        if outputs.len() != 1 || count_operator(&outputs, Operator::OR) != 1 {
            bad_gates.push(gate);
        }
    }
    bad_gates
}

// Output to 1 XOR and 1 AND (except for last, which outputs as last bit)
fn collect_bad_or_gates(gates: &HashMap<String, Gate>) -> Vec<&GateData> {
    let mut bad_gates = Vec::new();

    for gate in gates.iter().filter_map(|(_, gate)| match gate {
        Gate::Normal(data) if data.op == Operator::OR => Some(data),
        _ => None,
    }) {
        if let Some(bit_pos) = gate.bit_pos() {
            if bit_pos as usize == gates.values().filter(|g| g.is_input()).count() / 2 {
                continue;
            }
        }
        let outputs = outputs_to(gates, &gate.out).collect::<Vec<&GateData>>();
        if outputs.len() != 2
            || count_operator(&outputs, Operator::XOR) != 1
            || count_operator(&outputs, Operator::AND) != 1
        {
            bad_gates.push(gate);
        }
    }
    bad_gates
}

fn outputs_to<'a>(
    gates: &'a HashMap<String, Gate>,
    from: &'a str,
) -> impl Iterator<Item = &'a GateData> {
    gates.values().filter_map(move |gate| match gate {
        Gate::Normal(data) if data.in1 == from || data.in2 == from => Some(data),
        _ => None,
    })
}

fn count_operator(outputs: &[&GateData], op: Operator) -> usize {
    outputs.iter().filter(|gate| gate.op == op).count()
}

// I know that only OUTPUT wires have been swapped, NOT input wires!

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

z45 OR
    AND
        x44
        y44
    AND
        same as z44
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

    fn is_input(&self) -> bool {
        match self {
            Gate::Input(_) => true,
            Gate::Normal(_) => false,
        }
    }
}

#[derive(Debug)]
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

    fn bit_pos(&self) -> Option<u8> {
        parse::numbers::<u8>(&self.out).next()
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

#[derive(PartialEq, Eq, Debug)]
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
