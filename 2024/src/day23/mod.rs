use std::collections::{HashMap, HashSet};

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    for input in inputs {
        let kv: Vec<&str> = input.split('-').collect();
        connections.entry(kv[0]).or_default().push(kv[1]);
        connections.entry(kv[1]).or_default().push(kv[0]);
    }
    let mut ret: HashSet<String> = HashSet::new();
    for (a, b_s) in &connections {
        if !a.starts_with('t') {
            continue;
        }
        for b in b_s {
            if let Some(c_s) = connections.get(b) {
                for c in c_s {
                    if let Some(a_s) = connections.get(c) {
                        for d in a_s {
                            if d == a {
                                let mut set_v: Vec<&str> = vec![a, b, c];
                                set_v.sort();
                                let r: String = set_v.join(",");
                                ret.insert(r);
                            }
                        }
                    }
                }
            }
        }
    }

    ret.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day23/test1.txt");

        let result = solve_part1(&inputs);
        assert_eq!(result, 7);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day23/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 1378);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day23/test2.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 23);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day23/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 2242);
    // }
}
