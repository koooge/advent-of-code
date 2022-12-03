pub fn solve_part1(inputs: Vec<String>) -> u32 {
    let mut ret: u32 = 0;
    let priorities = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

    for input in inputs {
        let item_len = input.len() / 2;
        let first = &input[0..item_len];
        let second = &input[item_len..];
        for c in first.chars() {
            if second.contains(c) {
                let index = priorities.find(c).unwrap() as u32;
                ret += index + 1;
                break;
            }
        }
    }

    ret
}

pub fn solve_part2(inputs: Vec<String>) -> u32 {
    let priorities = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut ret: u32 = 0;
    let ngroup = inputs.len() / 3;

    for i in 0..ngroup {
        let badge: char = find_badge(&inputs.get((i * 3)..=(i * 3) + 2).unwrap().to_vec());
        let prio = priorities.find(badge).unwrap() as u32 + 1;
        ret += prio;
    }

    ret
}

fn find_badge(inputs: &Vec<String>) -> char {
    let mut badges: Vec<char> = inputs[0].clone().chars().collect();
    badges.sort();
    badges.dedup();

    for input in inputs[1..=2].iter() {
        let mut line: Vec<char> = input.clone().chars().collect();
        line.sort();
        line.dedup();
        badges.retain(|&x| line.contains(&x));
    }

    *badges.get(0).unwrap()
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
        assert_eq!(result, 157);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 7967);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 70);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 58);
    }

    fn read_file(file_path: &str) -> Vec<String> {
        let contents = fs::read_to_string(file_path);
        let mut ret: Vec<String> = vec![];
        match contents {
            Ok(contents) => {
                for line in contents.lines() {
                    ret.push(line.trim().to_string());
                }
            }
            Err(why) => eprintln!("{}", why),
        }
        ret
    }
}
