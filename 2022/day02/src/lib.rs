pub fn solve_part1(inputs: Vec<String>) -> u32 {
    let mut ret: u32 = 0;

    for input in inputs {
        let opponent = input.chars().nth(2).unwrap();
        let you = input.chars().nth(0).unwrap();
        ret += match opponent {
            'X' => match you {
                'A' => 1 + 3,
                'B' => 1 + 0,
                'C' => 1 + 6,
                _ => 0,
            },
            'Y' => match you {
                'A' => 2 + 6,
                'B' => 2 + 3,
                'C' => 2 + 0,
                _ => 0,
            },
            'Z' => match you {
                'A' => 3 + 0,
                'B' => 3 + 6,
                'C' => 3 + 3,
                _ => 0,
            },
            _ => 0,
        };
    }

    ret
}

pub fn solve_part2(inputs: Vec<String>) -> u32 {
    let mut ret: u32 = 0;

    for input in inputs {
        let opponent = input.chars().nth(2).unwrap();
        let you = input.chars().nth(0).unwrap();
        ret += match opponent {
            'X' => match you {
                'A' => 3 + 0,
                'B' => 1 + 0,
                'C' => 2 + 0,
                _ => 0,
            },
            'Y' => match you {
                'A' => 1 + 3,
                'B' => 2 + 3,
                'C' => 3 + 3,
                _ => 0,
            },
            'Z' => match you {
                'A' => 2 + 6,
                'B' => 3 + 6,
                'C' => 1 + 6,
                _ => 0,
            },
            _ => 0,
        };
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
        assert_eq!(result, 15);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 12276);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 12);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 9975);
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
