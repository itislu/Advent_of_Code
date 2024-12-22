use itertools::Itertools;
use utils::{input, parse};

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> String {
    let mut computer = Computer::new(input);
    computer.run();
    computer.output()
}

fn exercise2(input: &str) -> usize {
    let mut computer = Computer::new(input);
    let mut reg_a = 202992820169258;
    loop {
        computer.reset(reg_a);
        if computer.run_until_copy() {
            println!(
                "trying {} -> {} (len: {})",
                reg_a,
                computer.output(),
                computer.out.len()
            );
            // break;
        }
        reg_a -= 8;
    }
    reg_a
}

struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    instr: Vec<usize>,
    ptr: usize,
    out: Vec<char>,
    out_cnt: usize,
}

impl Computer {
    fn new(input: &str) -> Self {
        let mut split_input = input.split("\n\n");
        let registers: Vec<usize> = parse::numbers(split_input.nth(0).unwrap()).collect();
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
            instr,
            ptr: 0,
            out: Vec::new(),
            out_cnt: 0,
        }
    }

    fn reset(&mut self, reg_a: usize) {
        self.reg_a = reg_a;
        self.reg_b = 0;
        self.reg_c = 0;
        self.ptr = 0;
        self.out.clear();
        self.out_cnt = 0;
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
                _ => panic!("Invalid instruction!"),
            };
            if self.ptr == prev_ptr {
                self.ptr += 2;
            }
        }
    }

    fn run_until_copy(&mut self) -> bool {
        while self.ptr < self.instr.len() {
            if !self.is_out_still_same() {
                return false;
            }

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
                _ => panic!("Invalid instruction!"),
            };
            if self.ptr == prev_ptr {
                self.ptr += 2;
            }
        }
        self.out.len() == self.instr.len()
    }

    fn is_out_still_same(&self) -> bool {
        if self.out_cnt == 0 {
            return true;
        }
        if self.out.len() > self.instr.len() {
            return false;
        }
        (self.out[self.out_cnt - 1] as u8 - b'0') as usize == self.instr[self.out_cnt - 1]
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
            self.ptr = operand;
        }
    }

    fn bxc(&mut self, _operand: usize) {
        self.reg_b ^= self.reg_c;
    }

    fn out(&mut self, operand: usize) {
        self.out
            .push(((self.combo(operand) & 7) as u8 + b'0') as char);
        self.out_cnt += 1;
    }

    fn bdv(&mut self, operand: usize) {
        self.reg_b = self.reg_a >> self.combo(operand);
    }

    fn cdv(&mut self, operand: usize) {
        self.reg_c = self.reg_a >> self.combo(operand);
    }

    fn output(&self) -> String {
        self.out.iter().join(",")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_file("input_example1.txt");
        let res = exercise1(&input);
        assert_eq!(res, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_ex2() {
        let input = input::read_file("input_example2.txt");
        let res = exercise2(&input);
        assert_eq!(res, 117440);
    }
}
