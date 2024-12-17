use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &str) -> String {
    let mut computer = Computer::new(input);
    computer.run();
    computer.out
}

struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    instr: Vec<usize>,
    ptr: usize,
    out: String,
}

impl Computer {
    fn new(input: &str) -> Self {
        let mut split_input = input.split("\n\n");
        let registers = parse_numbers(split_input.nth(0).unwrap());
        let instr: Vec<usize> = split_input
            .nth(0)
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Self {
            reg_a: registers[0] as usize,
            reg_b: registers[1] as usize,
            reg_c: registers[2] as usize,
            instr: instr,
            ptr: 0,
            out: String::new(),
        }
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid combo operand found!"),
        }
    }

    fn run(&mut self) {
        while self.ptr < self.instr.len() {
            let prev_ptr = self.ptr;
            let operand = self.instr[self.ptr + 1];
            match self.instr[self.ptr] {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!("Invalid instruction"),
            };
            if self.ptr == prev_ptr {
                self.ptr += 2;
            }
        }
    }

    fn adv(&mut self, operand: usize) {
        self.reg_a >>= self.combo(operand);
    }
    
    fn bxl(&mut self, operand: usize) {
        self.reg_b ^= operand;
    }

    fn bst(&mut self, operand: usize) {
        self.reg_b = self.combo(operand) & 7;
    }

    fn jnz(&mut self, operand: usize) {
        if self.reg_a != 0 {
            self.ptr = operand as usize;
        }
    }

    fn bxc(&mut self, operand: usize) {
        self.reg_b ^= self.reg_c;
    }

    fn out(&mut self, operand: usize) {
        if !self.out.is_empty() {
            self.out += ",";
        }
        self.out += &(self.combo(operand) & 7).to_string();
    }

    fn bdv(&mut self, operand: usize) {
        self.reg_b = self.reg_a >> self.combo(operand);
    }

    fn cdv(&mut self, operand: usize) {
        self.reg_c = self.reg_a >> self.combo(operand);
    }
}

fn parse_numbers(s: &str) -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();
    let mut current_number = String::new();

    for c in s.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else if !current_number.is_empty() {
            numbers.push(current_number.parse().unwrap());
            current_number.clear();
        }
    }
    if !current_number.is_empty() {
        numbers.push(current_number.parse().unwrap());
    }
    numbers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, "4,6,3,5,6,3,5,2,1,0");
    }
}
