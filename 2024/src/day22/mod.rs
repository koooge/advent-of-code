use std::collections::{HashMap, HashSet};

fn mix(n: usize, secret: usize) -> usize {
    n ^ secret
}
fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn get_next_secret(secret: usize) -> usize {
    let mut ret = prune(mix(secret * 64, secret));
    ret = prune(mix(ret / 32, ret));
    prune(mix(ret * 2048, ret))
}

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut secrets: Vec<usize> = vec![];
    for input in inputs {
        secrets.push(input.parse().unwrap());
    }

    for _ in 0..2000 {
        for secret in secrets.iter_mut() {
            *secret = get_next_secret(*secret);
        }
    }

    secrets.iter().sum()
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut prices: Vec<HashMap<(isize, isize, isize, isize), usize>> = vec![];
    let mut seqs: HashSet<(isize, isize, isize, isize)> = HashSet::new();

    for input in inputs {
        let mut secret: usize = input.parse().unwrap();
        let mut price: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();
        let mut p: Vec<isize> = vec![];
        for i in 0..2000 {
            let v = secret % 10;
            let next = get_next_secret(secret);
            let next_v = next % 10;
            p.push(next_v as isize - v as isize);
            secret = next;
            if i >= 3 {
                let seq: (isize, isize, isize, isize) = (
                    p[p.len() - 4],
                    p[p.len() - 3],
                    p[p.len() - 2],
                    p[p.len() - 1],
                );
                seqs.insert(seq);
                price.entry(seq).or_insert(next_v);
            }
        }
        prices.push(price);
    }

    let mut ret: Vec<usize> = vec![];
    for seq in seqs {
        let mut p: usize = 0;
        for price in &prices {
            p += price.get(&seq).unwrap_or(&0);
        }
        ret.push(p);
    }
    *ret.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day22/test1.txt");

        let result = solve_part1(&inputs);
        assert_eq!(result, 37327623);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day22/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 20071921341);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day22/test2.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 23);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day22/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 2242);
    }
}
