use core::fmt;
use std::mem::swap;
use utils::input;

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let mut disk = Disk::new(input);

    disk.partition();
    println!("DISK:\n{}", disk);
    disk.checksum()
}

fn exercise2(input: &str) -> usize {
    let mut disk = Disk::new(input);

    disk.defragment();
    println!("DISK:\n{}", disk);
    disk.checksum()
}

struct Disk {
    data: Vec<Byte>,
    first_free_byte: usize,
    last_file_byte: usize,
    max_file_id: usize,
    size: usize,
}

impl Disk {
    fn new(input: &str) -> Self {
        let mut disk = Disk {
            data: Vec::new(),
            first_free_byte: 0,
            last_file_byte: 0,
            max_file_id: 0,
            size: 0,
        };
        let mut index: usize = 0;

        for (i, ch) in input.chars().enumerate() {
            let file_id = match i % 2 {
                0 => Some(i / 2),
                1 => None,
                _ => panic!(),
            };
            for _ in 0..ch.to_digit(10).unwrap() {
                disk.data.push(Byte::new(file_id, index));
                index += 1;
            }
            if let Some(file_id) = file_id {
                disk.max_file_id = file_id;
            }
        }
        disk.size = disk.data.len();
        Disk::update(&mut disk);
        disk
    }

    fn partition(&mut self) {
        while self.first_free_byte < self.last_file_byte {
            self.swap(self.first_free_byte, self.last_file_byte);
        }
    }

    fn defragment(&mut self) {
        for file_id in (0..=self.max_file_id).rev() {
            if let Some(file_range) = self.get_file_range(file_id) {
                if let Some(free_range) = self.get_free_range(file_range.len()) {
                    if free_range.start < file_range.start {
                        for (free_idx, file_idx) in free_range.zip(file_range) {
                            self.swap(free_idx, file_idx);
                        }
                    }
                }
            }
        }
    }

    fn checksum(&self) -> usize {
        let mut checksum: usize = 0;
        for i in 0..=self.last_file_byte {
            if let Some(file_id) = self.data[i].file_id {
                checksum += i * file_id;
            }
        }
        checksum
    }

    fn update(&mut self) {
        if let Some(first_free_byte) = self.first_free_byte(0) {
            self.first_free_byte = first_free_byte.index;
        }
        if let Some(last_file_byte) = self.last_file_byte(self.size) {
            self.last_file_byte = last_file_byte.index
        }
    }

    fn swap(&mut self, mut low: usize, mut high: usize) {
        if high < low {
            swap(&mut high, &mut low);
        }
        self.data.swap(low, high);
        self.data[low].index = low;
        self.data[high].index = high;
        if low == self.first_free_byte {
            self.first_free_byte = self.first_free_byte(low).unwrap().index;
        }
        if high == self.last_file_byte {
            self.last_file_byte = self.last_file_byte(high + 1).unwrap().index;
        }
    }

    fn first_free_byte(&self, start: usize) -> Option<&Byte> {
        self.data.iter().skip(start).find(|byte| !byte.is_file())
    }

    fn last_file_byte(&self, end: usize) -> Option<&Byte> {
        self.data
            .iter()
            .rev()
            .skip(self.size.checked_sub(end)?)
            .find(|byte| byte.is_file())
    }

    fn get_free_range(&self, size: usize) -> Option<std::ops::Range<usize>> {
        let mut start: usize = 0;
        loop {
            start = self.first_free_byte(start)?.index;
            let mut end = start;
            while end < self.size && !self.data[end].is_file() {
                end += 1;
            }
            if end - start >= size {
                return Some(start..end);
            }
            if end == self.size {
                return None;
            }
            start = end;
        }
    }

    fn get_file_range(&self, file_id: usize) -> Option<std::ops::Range<usize>> {
        let start = self
            .data
            .iter()
            .find(|byte| byte.is_file_id(file_id))?
            .index;
        let end = start
            + self.data[start..]
                .iter()
                .take_while(|byte| byte.is_file_id(file_id))
                .count();
        Some(start..end)
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

    fn is_file_id(&self, file_id: usize) -> bool {
        self.is_file() && self.file_id.unwrap() == file_id
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
mod tests {
    use super::*;

    mod exercise1 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise1(&input);
            assert_eq!(res, 1928);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 6385338159127);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input);
            assert_eq!(res, 2858);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 6415163624282);
        }
    }
}
