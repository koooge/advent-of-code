pub fn solve_part1(inputs: &[String]) -> isize {
    let mut ret = 0;

    for input in inputs {
        let levels: Vec<isize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let is_increase = levels[0] < levels[1];
        let mut is_safe = true;
        for i in 0..levels.len() - 1 {
            if is_increase {
                let d = levels[i + 1] - levels[i];
                if d <= 0 || d > 3 {
                    is_safe = false;
                    break;
                }
            } else {
                let d = levels[i] - levels[i + 1];
                if d <= 0 || d > 3 {
                    is_safe = false;
                    break;
                }
            }
        }
        if is_safe {
            ret += 1;
        }
    }

    ret
}

pub fn solve_part2(inputs: &[String]) -> isize {
    let mut ret = 0;

    fn is_safe(numbers: &[isize]) -> bool {
        let is_increasing = numbers.windows(2).all(|w| w[1] > w[0] && w[1] - w[0] <= 3);
        let is_decreasing = numbers.windows(2).all(|w| w[1] < w[0] && w[0] - w[1] <= 3);
        is_increasing || is_decreasing
    }

    for input in inputs {
        let levels: Vec<isize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if is_safe(&levels) {
            ret += 1;
        } else {
            for i in 0..levels.len() {
                let (left, right) = levels.split_at(i);
                if is_safe(&[left, &right[1..]].concat()) {
                    ret += 1;
                    break;
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
    fn part1_case1() {
        let inputs = read_file("./src/day02/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day02/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 534);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day02/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 4);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day02/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 577);
    }
}
