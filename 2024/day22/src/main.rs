use utils::input;

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
}

fn exercise1(input: &str) -> usize {
    utils::parse::numbers::<usize>(input)
        .map(|secret| calc_secret(secret, 2000))
        .sum()
}

fn calc_secret(mut secret: usize, iterations: usize) -> usize {
    for _ in 0..iterations {
        secret = prune(mix(secret, secret * 64));
        secret = prune(mix(secret, secret / 32));
        secret = prune(mix(secret, secret * 2048));
    }
    secret
}

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

fn prune(a: usize) -> usize {
    a % 16777216
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = input::read_example();
        let res = exercise1(&input);
        assert_eq!(res, 37327623);
    }
}
