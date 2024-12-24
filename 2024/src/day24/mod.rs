use std::collections::{HashMap, VecDeque};

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut outs: HashMap<String, usize> = HashMap::new();
    let mut todos: VecDeque<Vec<&str>> = VecDeque::from(vec![]);
    let mut is_second = false;
    for input in inputs {
        if input.is_empty() {
            is_second = true;
            continue;
        }
        if !is_second {
            let data: Vec<&str> = input.split(": ").collect();
            outs.insert(data[0].to_string(), data[1].parse().unwrap());
        } else {
            todos.push_back(input.split_whitespace().collect());
        }
    }

    while let Some(logic) = todos.pop_front() {
        let a = outs.get(logic[0]);
        let b = outs.get(logic[2]);
        if a.is_none() || b.is_none() {
            todos.push_back(logic);
            continue;
        }

        match logic[1] {
            "AND" => outs.insert(logic[4].to_string(), a.unwrap() & b.unwrap()),
            "OR" => outs.insert(logic[4].to_string(), a.unwrap() | b.unwrap()),
            "XOR" => outs.insert(logic[4].to_string(), a.unwrap() ^ b.unwrap()),
            _ => unreachable!(),
        };
    }

    let mut ret = 0;
    for i in 0..100 {
        let k = format!("z{:02}", i);
        if let Some(z) = outs.get(&k) {
            ret += z << i;
        } else {
            break;
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
        let inputs = read_file("./src/day24/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 4);
    }

    #[test]
    fn part1_case2() {
        let inputs = read_file("./src/day24/test2.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 2024);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day24/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 42410633905894);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day24/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, String::from("co,de,ka,ta"));
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day24/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(
    //         result,
    //         String::from("bs,ey,fq,fy,he,ii,lh,ol,tc,uu,wl,xq,xv")
    //     );
    // }
}
