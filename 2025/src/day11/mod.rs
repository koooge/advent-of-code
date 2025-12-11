use std::collections::HashMap;

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut ret = 0;
    let mut list: HashMap<String, Vec<String>> = HashMap::new();

    for line in inputs {
        let (k, v) = line.split_once(": ").unwrap();
        let values: Vec<String> = v.split(' ').map(|s| s.to_string()).collect();
        list.insert(k.to_string(), values);
    }

    let mut stack: Vec<String> = vec!["you".to_string()];
    while let Some(ptr) = stack.pop() {
        let values = list.get(&ptr).unwrap();
        for v in values {
            if v == "out" {
                ret += 1;
            } else {
                stack.push(v.to_string());
            }
        }
    }

    ret
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
        assert_eq!(result, 5);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day11/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 477);
    }
}
