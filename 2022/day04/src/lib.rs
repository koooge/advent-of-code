pub fn solve_part1(inputs: Vec<String>) -> u32 {
    let mut ret: u32 = 0;

    for input in inputs {
        let pairs: Vec<&str> = input.split(',').collect();
        let first: Vec<u32> = pairs
            .get(0)
            .unwrap()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();
        let second: Vec<u32> = pairs
            .get(1)
            .unwrap()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();

        if (first[0] <= second[0] && second[1] <= first[1])
            || (second[0] <= first[0] && first[1] <= second[1])
        {
            ret += 1;
        }
    }

    ret
}

pub fn solve_part2(inputs: Vec<String>) -> u32 {
    let mut ret: u32 = 0;

    for input in inputs {
        let pairs: Vec<&str> = input.split(',').collect();
        let first: Vec<u32> = pairs
            .get(0)
            .unwrap()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();
        let second: Vec<u32> = pairs
            .get(1)
            .unwrap()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();

        if (first[0] <= second[0] && second[0] <= first[1])
            || (second[0] <= first[0] && first[0] <= second[1])
        {
            ret += 1;
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
        assert_eq!(result, 2);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 483);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 4);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 874);
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
