use core::fmt;
use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &str) -> usize {
    let mut disk = Disk::new(input);

    disk.partition();
    println!("DISK:\n{}", disk);
    disk.checksum()
}

struct Disk {
    data: Vec<Byte>,
    first_free: usize,
    last_file: usize,
    size: usize,
}

impl Disk {
    fn new(input: &str) -> Self {
        let mut data: Vec<Byte> = Vec::new();
        let mut index: usize = 0;

        for (i, ch) in input.chars().enumerate() {
            let file_id = match i % 2 {
                0 => Some(i / 2),
                1 => None,
                _ => panic!(),
            };
            for _ in 0..ch.to_digit(10).unwrap() {
                data.push(Byte::new(file_id, index));
                index += 1;
            }
        }
        Disk {
            first_free: data.iter().find(|byte| !byte.is_file()).unwrap().index,
            last_file: data.iter().rev().find(|byte| byte.is_file()).unwrap().index,
            size: data.len(),
            data,
        }
    }

    fn first_free(&mut self) -> &Byte {
        let first_free = self.data.iter().find(|byte| !byte.is_file()).unwrap();
        self.first_free = first_free.index;
        first_free
    }

    fn last_file(&mut self) -> &Byte {
        let last_file = self.data.iter().rev().find(|byte| byte.is_file()).unwrap();
        self.last_file = last_file.index;
        last_file
    }

    fn partition(&mut self) {
        while self.first_free().index < self.last_file().index {
            self.data.swap(self.first_free, self.last_file);
            self.data[self.first_free].index = self.first_free;
            self.data[self.last_file].index = self.last_file;
        }
    }

    fn checksum(&self) -> usize {
        let mut checksum: usize = 0;
        for i in 0..=self.last_file {
            if let Some(file_id) = self.data[i].file_id {
                checksum += i * file_id;
            }
        }
        checksum
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.data {
            write!(f, "{}", byte)?;
        }
        Ok(())
    }
}

struct Byte {
    file_id: Option<usize>,
    index: usize,
}

impl Byte {
    fn new(file_id: Option<usize>, index: usize) -> Self {
        Byte { file_id, index }
    }

    fn is_file(&self) -> bool {
        self.file_id.is_some()
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(file_id) = self.file_id {
            write!(f, "{}", file_id)
        } else {
            write!(f, ".")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 1928);
    }
}
