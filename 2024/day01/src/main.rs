use utils::input;

/*
    Historian Hysteria - Day 1
    Part 1: Given two lists of numbers, pair smallest-to-smallest and sum the absolute differences between pairs.
    Part 2: For each number in left list, multiply it by its frequency in right list and sum all products.
*/

fn main() {
    let input = input::read_file("input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    let (mut list1, mut list2) = create_lists(input);
    list1.sort();
    list2.sort();

    let mut res = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        res += a.abs_diff(*b);
    }
    res
}

fn exercise2(input: &str) -> usize {
    let (list1, list2) = create_lists(input);

    let mut res = 0;
    for a in list1.iter() {
        res += a * list2.iter().filter(|b| *b == a).count();
    }
    res
}

fn create_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        list1.push(parts.next().unwrap().parse().unwrap());
        list2.push(parts.next().unwrap().parse().unwrap());
    }
    (list1, list2)
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
            assert_eq!(res, 11);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise1(&input);
            assert_eq!(res, 2756096);
        }
    }

    mod exercise2 {
        use super::*;

        #[test]
        fn example() {
            let input = input::read_file("example.txt");
            let res = exercise2(&input);
            assert_eq!(res, 31);
        }

        #[test]
        fn answer() {
            let input = input::read_file("input.txt");
            let res = exercise2(&input);
            assert_eq!(res, 23117829);
        }
    }
}
