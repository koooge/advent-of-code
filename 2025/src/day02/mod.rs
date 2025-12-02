fn is_invalid(n: usize) -> bool {
    let s = n.to_string();
    let len = s.len();
    if !len.is_multiple_of(2) {
        return false;
    }
    let (a, b) = s.split_at(len / 2);
    a == b
}

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut ret = 0;

    for line in inputs {
        for r in line.split(',') {
            if r.is_empty() {
                continue;
            }

            let (first, last): (usize, usize) = r
                .split_once('-')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap();

            for n in first..=last {
                if is_invalid(n) {
                    ret += n;
                }
            }
        }
    }

    ret
}

// pub fn solve_part2(inputs: &[String], p: usize) -> usize {

// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn day01_part1_case1() {
        let inputs = read_file("./src/day02/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day02/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 56660955519);
    }
}
