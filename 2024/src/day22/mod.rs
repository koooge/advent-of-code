pub fn solve_part1(inputs: &[String]) -> usize {
    fn mix(n: usize, secret: usize) -> usize {
        n ^ secret
    }
    fn prune(secret: usize) -> usize {
        secret % 16777216
    }

    let mut secrets: Vec<usize> = vec![];
    for input in inputs {
        secrets.push(input.parse().unwrap());
    }

    for _ in 0..2000 {
        for secret in secrets.iter_mut() {
            *secret = prune(mix(*secret * 64, *secret));
            *secret = prune(mix(*secret / 32, *secret));
            *secret = prune(mix(*secret * 2048, *secret));
        }
    }

    secrets.iter().sum()
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

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day22/test1.txt");
    //     let result = solve_part2(&inputs, 50);
    //     assert_eq!(
    //         result,
    //         32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
    //     );
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day22/input1.txt");
    //     let result = solve_part2(&inputs, 100);
    //     assert_eq!(result, 0);
    // }
}
