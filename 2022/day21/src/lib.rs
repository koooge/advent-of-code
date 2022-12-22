use std::collections::HashMap;

#[derive(Debug)]
struct Monkey {
    a: String,
    op: char,
    b: String,
}

fn yell(monkeys: &HashMap<String, Monkey>, numbers: &HashMap<String, u64>, name: &str) -> u64 {
    if monkeys.contains_key(name) {
        let monkey = monkeys.get(name).unwrap();
        return match monkey.op {
            '+' => yell(monkeys, numbers, &monkey.a) + yell(monkeys, numbers, &monkey.b),
            '-' => yell(monkeys, numbers, &monkey.a) - yell(monkeys, numbers, &monkey.b),
            '*' => yell(monkeys, numbers, &monkey.a) * yell(monkeys, numbers, &monkey.b),
            '/' => yell(monkeys, numbers, &monkey.a) / yell(monkeys, numbers, &monkey.b),
            _ => unreachable!(),
        };
    } else if numbers.contains_key(name) {
        return *numbers.get(name).unwrap();
    }

    unreachable!();
}

pub fn solve_part1(inputs: &[String]) -> u64 {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    let mut numbers: HashMap<String, u64> = HashMap::new();
    for input in inputs {
        let (name, value) = input.split_once(": ").unwrap();
        let v = value.split(' ').collect::<Vec<_>>();
        if v.len() == 1 {
            let n: u64 = value.parse().unwrap();
            numbers.insert(name.to_string(), n);
        } else {
            let a: String = value.split(' ').collect::<Vec<&str>>()[0].to_string();
            let op: char = value.split(' ').collect::<Vec<&str>>()[1]
                .to_string()
                .chars()
                .nth(0)
                .unwrap();
            let b: String = value.split(' ').collect::<Vec<&str>>()[2].to_string();
            monkeys.insert(name.to_string(), Monkey { a, op, b });
        }
    }

    yell(&monkeys, &numbers, "root")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 152);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 145167969204648);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(&inputs[0..2]);
    //     assert_eq!(result, 62);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(&inputs[0..3]);
    //     assert_eq!(result, 62);
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
