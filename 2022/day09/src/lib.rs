use std::collections::HashSet;

pub fn solve_part1(inputs: Vec<String>) -> u32 {
    let mut head: [i32; 2] = [0, 0];
    let mut tail: [i32; 2] = [0, 0];
    let mut visited: HashSet<[i32; 2]> = HashSet::new();
    visited.insert([0, 0]);

    for input in inputs {
        let direction: char = input.chars().nth(0).unwrap();
        let n: u32 = input.split_at(2).1.parse().unwrap();
        for _ in 0..n {
            match direction {
                'U' => head[1] += 1,
                'R' => head[0] += 1,
                'D' => head[1] -= 1,
                'L' => head[0] -= 1,
                _ => panic!(),
            }

            let dx = head[0] - tail[0];
            let dy = head[1] - tail[1];
            if (dx.abs() >= 2 && dy.abs() > 0) || (dx.abs() > 0 && dy.abs() >= 2) {
                tail[0] = if dx > 0 { tail[0] + 1 } else { tail[0] - 1 };
                tail[1] = if dy > 0 { tail[1] + 1 } else { tail[1] - 1 };
                visited.insert(tail);
            } else if dx.abs() >= 2 {
                tail[0] = if dx > 0 { tail[0] + 1 } else { tail[0] - 1 };
                visited.insert(tail);
            } else if dy.abs() >= 2 {
                tail[1] = if dy > 0 { tail[1] + 1 } else { tail[1] - 1 };
                visited.insert(tail);
            }
        }
    }

    visited.len() as u32
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
        assert_eq!(result, 13);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 6212);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(inputs);
    //     println!("{}", result);
    //     assert_eq!(result, 8);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(inputs);
    //     println!("{}", result);
    //     assert_eq!(result, 259308);
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
