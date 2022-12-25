fn to_i64(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => unreachable!(),
    }
}

fn to_snafu(n: i64) -> char {
    match n {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => unreachable!(),
    }
}

pub fn solve_part1(inputs: &[String]) -> String {
    let mut decimal: i64 = 0;

    for input in inputs {
        for (i, c) in input.chars().rev().enumerate() {
            let n: i64 = to_i64(c);
            decimal = decimal + n * 5_i64.pow(i as u32);
        }
    }

    fn step_once(val: i64) -> (i64, char) {
        let (div, rem) = (val / 5, val % 5);
        if rem > 2 {
            (div + 1, to_snafu(rem - 5))
        } else {
            (div, to_snafu(rem))
        }
    }
    let mut ret = Vec::new();

    while decimal > 0 {
        let (new_input, last) = step_once(decimal);
        ret.push(last);
        decimal = new_input;
    }

    ret.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = vec![String::from("2=-01")];
        let result = solve_part1(&inputs);
        assert_eq!(result, String::from("2=-01")); // 976
    }

    #[test]
    fn part1_case2() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, String::from("2=-1=0")); // 4890
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, String::from("2-2=21=0021=-02-1=-0")); // 35617696411945
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 20);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 908);
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
