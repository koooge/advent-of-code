use std::collections::HashSet;

pub fn solve_part1(inputs: Vec<String>) -> u32 {
    let input = inputs[0].clone();
    let mut ret: u32 = 0;

    for i in 0..input.len() - 3 {
        let seq = &input[i..=(i + 3)];
        let uniq: HashSet<char> = seq.chars().collect();
        if uniq.len() == 4 {
            ret = i as u32 + 4;
            break;
        }
    }

    ret
}

// pub fn solve_part2(inputs: Vec<String>) -> String {
//     let mut stacks: Vec<Vec<char>> = Vec::new();

//     for input in inputs {
//         let line = input.trim();

//         if line.starts_with('[') {
//             for (i, c) in input.chars().enumerate() {
//                 if c == '[' || c == ']' || c == ' ' {
//                     continue;
//                 }

//                 let index = (i - 1) / 4;
//                 while stacks.len() <= index {
//                     stacks.push(vec![]);
//                 }
//                 stacks[index].insert(0, c);
//             }
//         } else if line.starts_with("move") {
//             let ops: Vec<&str> = input.split(' ').collect();
//             let n: usize = ops[1].parse().unwrap();
//             let from: usize = ops[3].parse().unwrap();
//             let to: usize = ops[5].parse().unwrap();
//             let mut moving: Vec<char> = Vec::new();
//             for _ in 0..n {
//                 moving.push(stacks[from - 1].pop().unwrap());
//             }
//             moving.reverse();
//             stacks[to - 1].append(&mut moving);
//         }
//     }

//     stacks.iter().map(|v| v.last().unwrap()).collect()
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 7);
    }

    #[test]
    fn part1_case2() {
        let inputs = read_file("./src/test2.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 5);
    }

    #[test]
    fn part1_case3() {
        let inputs = read_file("./src/test3.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 6);
    }

    #[test]
    fn part1_case4() {
        let inputs = read_file("./src/test4.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 10);
    }

    #[test]
    fn part1_case5() {
        let inputs = read_file("./src/test5.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 11);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 1140);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(inputs);
    //     println!("{}", result);
    //     assert_eq!(result, "MCD");
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(inputs);
    //     println!("{}", result);
    //     assert_eq!(result, "QRQFHFWCL");
    // }

    fn read_file(file_path: &str) -> Vec<String> {
        let contents = fs::read_to_string(file_path);
        let mut ret: Vec<String> = vec![];
        match contents {
            Ok(contents) => {
                for line in contents.lines() {
                    ret.push(line.to_string());
                }
            }
            Err(why) => eprintln!("{}", why),
        }
        ret
    }
}
