pub fn solve_part1(inputs: &[String]) -> isize {
    let mut ret = 0;
    let mut left: Vec<isize> = vec![];
    let mut right: Vec<isize> = vec![];

    for input in inputs {
        let lr: Vec<isize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        left.push(lr[0]);
        right.push(lr[1]);
    }
    left.sort();
    right.sort();

    for i in 0..inputs.len() {
        ret += (left[i] - right[i]).abs();
    }

    ret
}

pub fn solve_part2(inputs: &[String]) -> isize {
    let mut ret = 0;
    let mut left: Vec<isize> = vec![];
    let mut right: Vec<isize> = vec![];

    for input in inputs {
        let lr: Vec<isize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        left.push(lr[0]);
        right.push(lr[1]);
    }

    for l in left {
        let mut n = 0;
        for r in &right {
            if l == *r {
                n += 1;
            }
        }
        ret += l * n;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn day01_part1_case1() {
        let inputs = read_file("./src/day01/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 11);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day01/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 1882714);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day01/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 31);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day01/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 19437052);
    }
}
