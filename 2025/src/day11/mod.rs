use std::collections::HashMap;

fn path_num(graph: &HashMap<String, Vec<String>>, start: &str, goal: &str) -> usize {
    let mut ret = 0;
    let mut stack: Vec<String> = vec![start.to_string()];
    while let Some(ptr) = stack.pop() {
        let values = graph.get(&ptr).unwrap();
        for v in values {
            if v == goal {
                ret += 1;
            } else if v == "out" {
                continue;
            } else {
                stack.push(v.to_string());
            }
        }
    }
    ret
}

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in inputs {
        let (k, v) = line.split_once(": ").unwrap();
        let values: Vec<String> = v.split(' ').map(|s| s.to_string()).collect();
        graph.insert(k.to_string(), values);
    }

    path_num(&graph, "you", "out")
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in inputs {
        let (k, v) = line.split_once(": ").unwrap();
        let values: Vec<String> = v.split(' ').map(|s| s.to_string()).collect();
        graph.insert(k.to_string(), values);
    }

    let a = path_num(&graph, "svr", "fft");
    let b = path_num(&graph, "fft", "dac");
    let c = path_num(&graph, "dac", "out");

    a * b * c
}

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

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day11/test2.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 2);
    }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day11/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 477);
    // }
}
