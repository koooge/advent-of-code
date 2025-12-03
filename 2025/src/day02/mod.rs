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

fn is_invalid2(n: usize) -> bool {
    if n < 10 {
        return false;
    }
    let s = n.to_string();
    let len = s.len();
    if s.chars().min() == s.chars().max() {
        return true;
    }

    for i in 1..=(len / 2) {
        if !len.is_multiple_of(i) {
            continue;
        }
        let first = &s[0..i];
        if s.as_bytes()
            .chunks(i)
            .all(|chunk| chunk == first.as_bytes())
        {
            return true;
        }
    }
    false
}

pub fn solve_part2(inputs: &[String]) -> usize {
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
                if is_invalid2(n) {
                    ret += n;
                }
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn day02_part1_case1() {
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

    #[test]
    fn day02_part2_case1() {
        let inputs = read_file("./src/day02/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn day02_part2_case2() {
        let inputs = vec![String::from("11-22")];
        let result = solve_part2(&inputs);
        assert_eq!(result, 33);
    }

    #[test]
    fn day02_part2_case3() {
        let inputs = vec![String::from("1009-1011")];
        let result = solve_part2(&inputs);
        assert_eq!(result, 1010);
    }

    #[test]
    fn day02_part2_case4() {
        let inputs = vec![String::from("1-10")];
        let result = solve_part2(&inputs);
        assert_eq!(result, 0);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day02/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 79183223243);
    }
}
