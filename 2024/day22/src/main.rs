use utils::{input, parse};

fn main() {
    let input = input::read_input();
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}

fn exercise1(input: &str) -> usize {
    const SECRET_AMOUNT: usize = 2000;
    let mut res = 0;

    for mut secret in parse::numbers::<usize>(input) {
        for _ in 0..SECRET_AMOUNT {
            secret = calc_secret(secret);
        }
        res += secret;
    }
    res
}

fn exercise2(input: &str) -> usize {
    const SECRET_AMOUNT: usize = 2000;
    const SEQUENCE_LEN: usize = 4;

    let buyers: Vec<Vec<(i8, i8)>> = parse::numbers::<usize>(input)
        .map(|secret| zip_prices_and_changes(secret, SECRET_AMOUNT))
        .collect();

    buyers
        .iter()
        .map(|buyer| {
            buyer
                .windows(SEQUENCE_LEN)
                .map(|sequence| sell_all(sequence, &buyers))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn sell_all(sequence: &[(i8, i8)], buyers: &[Vec<(i8, i8)>]) -> usize {
    buyers
        .iter()
        .filter_map(|buyer| sell(sequence, &buyer))
        .sum()
}

fn sell(sequence: &[(i8, i8)], buyer: &[(i8, i8)]) -> Option<usize> {
    buyer
        .windows(sequence.len())
        .find(|win| win.iter().zip(sequence).all(|(win, seq)| win.1 == seq.1))
        .map(|found| found[sequence.len() - 1].0 as usize)
}

fn zip_prices_and_changes(secret: usize, amount: usize) -> Vec<(i8, i8)> {
    let prices: Vec<i8> = get_prices(secret, amount + 1).collect();
    prices
        .iter()
        .copied()
        .skip(1)
        .zip(get_changes(&prices))
        .collect()
}

fn get_changes(prices: &[i8]) -> impl Iterator<Item = i8> + '_ {
    prices.windows(2).map(|win| win[1] - win[0])
}

fn get_prices(mut secret: usize, amount: usize) -> impl Iterator<Item = i8> {
    (0..amount).map(move |_| {
        let prev = secret;
        secret = calc_secret(secret);
        (prev % 10) as i8
    })
}

fn calc_secret(mut secret: usize) -> usize {
    secret = prune(mix(secret, secret * 64));
    secret = prune(mix(secret, secret / 32));
    secret = prune(mix(secret, secret * 2048));
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
        let input = input::read_file("input_example1.txt");
        let res = exercise1(&input);
        assert_eq!(res, 37327623);
    }

    #[test]
    fn test_ex2() {
        let input = input::read_file("input_example2.txt");
        let res = exercise2(&input);
        assert_eq!(res, 23);
    }
}
