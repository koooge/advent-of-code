pub fn solve_part1(inputs: &[String]) -> usize {
    let mut stones: Vec<usize> = inputs[0]
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    fn blink(stones: &[usize]) -> Vec<usize> {
        let mut ret = vec![];
        for stone in stones {
            if *stone == 0 {
                ret.push(1);
            } else {
                let stone_s = stone.to_string();
                if stone_s.len() % 2 == 0 {
                    let (left_s, right_s) = stone_s.split_at(stone_s.len() / 2);
                    ret.push(left_s.parse().unwrap());
                    ret.push(right_s.parse().unwrap());
                } else {
                    ret.push(stone * 2024);
                }
            }
        }

        ret
    }

    for _ in 0..25 {
        stones = blink(&stones);
    }

    stones.len()
}

// pub fn solve_part2(inputs: &[String]) -> usize {

// }

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

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day11/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 81);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day11/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 1302);
    // }
}
