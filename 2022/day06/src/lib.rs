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

pub fn solve_part2(inputs: Vec<String>) -> u32 {
    let input = inputs[0].clone();
    let mut ret: u32 = 0;

    for i in 0..input.len() - 3 {
        let seq = &input[i..=(i + 13)];
        let uniq: HashSet<char> = seq.chars().collect();
        if uniq.len() == 14 {
            ret = i as u32 + 14;
            break;
        }
    }

    ret
}

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

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 19);
    }

    #[test]
    fn part2_case2() {
        let inputs = read_file("./src/test2.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 23);
    }

    #[test]
    fn part2_case3() {
        let inputs = read_file("./src/test3.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 23);
    }

    #[test]
    fn part2_case4() {
        let inputs = read_file("./src/test4.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 29);
    }

    #[test]
    fn part2_case5() {
        let inputs = read_file("./src/test5.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 26);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 3495);
    }

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
