use std::collections::HashMap;

fn blink(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut ret: HashMap<usize, usize> = HashMap::new();
    for (k, v) in stones {
        if *k == 0 {
            *ret.entry(1).or_insert(0) += v;
        } else {
            let stone_s = k.to_string();
            if stone_s.len() % 2 == 0 {
                let (left_s, right_s) = stone_s.split_at(stone_s.len() / 2);
                *ret.entry(left_s.parse().unwrap()).or_insert(0) += v;
                *ret.entry(right_s.parse().unwrap()).or_insert(0) += v;
            } else {
                *ret.entry(k * 2024).or_insert(0) += v;
            }
        }
    }
    ret
}

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut stones: HashMap<usize, usize> = inputs[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .map(|k| (k, 1))
        .collect();

    for _ in 0..25 {
        stones = blink(&stones);
    }

    stones.values().sum()
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut stones: HashMap<usize, usize> = inputs[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .map(|k| (k, 1))
        .collect();

    for _ in 0..75 {
        stones = blink(&stones);
    }

    stones.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day11/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 55312);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day11/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 229043);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day11/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 65601038650482);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day11/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 272673043446478);
    }
}
