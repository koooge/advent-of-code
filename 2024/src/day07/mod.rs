pub fn solve_part1(inputs: &[String]) -> usize {
    let mut ret = 0;

    fn is_equal(test_value: usize, nums: &[usize], ops: &[usize]) -> bool {
        let mut sum = nums[0];
        for (i, op) in ops.iter().enumerate() {
            match op {
                0 => sum += nums[i + 1],
                1 => sum *= nums[i + 1],
                _ => unreachable!(),
            };
        }
        test_value == sum
    }

    for input in inputs {
        let values: Vec<usize> = input
            .split(|c: char| c == ':' || c.is_whitespace())
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let test_value = values[0];
        let nums = &values[1..];

        let ops = vec![0; nums.len() - 1];
        if is_equal(test_value, nums, &ops) {
            ret += test_value;
        } else {
            let mut is_equaled = false;
            for i in 0..ops.len() {
                let mut ops_i = ops.clone();
                for op in ops_i.iter_mut().take(i) {
                    *op = 1;
                }

                for j in i..ops_i.len() {
                    let mut ops_j = ops_i.clone();
                    ops_j[j] = 1;
                    if is_equal(test_value, nums, &ops_j) {
                        ret += test_value;
                        is_equaled = true;
                        break;
                    }
                }
                if is_equaled {
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
        let inputs = read_file("./src/day07/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 3749);
    }

    #[test]
    fn part1_case2() {
        let case2 = vec![String::from("29: 10 19")];
        let result = solve_part1(&case2);
        assert_eq!(result, 29);
    }

    #[test]
    fn part1_case3() {
        let case2 = vec![String::from("87480: 81 40 27")];
        let result = solve_part1(&case2);
        assert_eq!(result, 87480);
    }

    // #[test]
    // fn part1() {
    //     let inputs = read_file("./src/day07/input1.txt");
    //     let result = solve_part1(&inputs);
    //     assert_eq!(result, 1247928705891); // too low
    // }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day07/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 6);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day07/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 1);
    // }
}
