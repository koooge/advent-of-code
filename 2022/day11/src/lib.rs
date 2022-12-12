enum Operation {
  Add(u32),
  Mul(u32),
  Square,
}

struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test: u32,
    test_true: u32,
    test_false: u32,
    inspect_cnt: u32,
}

fn get_value(s: &String) -> String {
  String::from(s.split(": ").collect::<Vec<&str>>()[1])
}

pub fn solve_part1(inputs: Vec<String>) -> u32 {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut iter = inputs.into_iter();
    while let Some(_) = iter.next() {
        let items: Vec<u32> = get_value(&iter.next().unwrap())
            .split(", ")
            .filter_map(|x| x.parse().ok())
            .collect();
        let op: Vec<&str> = get_value(&iter.next().unwrap()).split(" ").collect(); // TODO: Compile Error
        let operation = match (op[3], op[4]) {
          ("*", "old") => Operation::Square,
          ("+", n) => Operation::Add(n.parse().unwrap()),
          ("*", n) => Operation::Mul(n.parse().unwrap()),
          _ => unreachable!(),
        };
        let monkey: Monkey = Monkey {
            items,
            operation,
            test: 4,
            test_true: 1,
            test_false: 2,
            inspect_cnt: 0,
        };
        monkeys.push(monkey);
    }

    let mut max2: Vec<u32> = monkeys.iter().map(|monkey| monkey.inspect_cnt).collect();
    max2.sort();
    max2.get(0).unwrap() * max2.get(1).unwrap()
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
        assert_eq!(result, 10605);
    }

    // #[test]
    // fn part1() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part1(inputs);
    //     println!("{}", result);
    //     assert_eq!(result, 13760);
    // }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(inputs);
    //     print_crt(result);
    //     assert_eq!(
    //         result,
    //         [
    //             true, true, false, false, true, true, false, false, true, true, false, false, true,
    //             true, false, false, true, true, false, false, true, true, false, false, true, true,
    //             false, false, true, true, false, false, true, true, false, false, true, true,
    //             false, false, true, true, true, false, false, false, true, true, true, false,
    //             false, false, true, true, true, false, false, false, true, true, true, false,
    //             false, false, true, true, true, false, false, false, true, true, true, false,
    //             false, false, true, true, true, false, true, true, true, true, false, false, false,
    //             false, true, true, true, true, false, false, false, false, true, true, true, true,
    //             false, false, false, false, true, true, true, true, false, false, false, false,
    //             true, true, true, true, false, false, false, false, true, true, true, true, true,
    //             false, false, false, false, false, true, true, true, true, true, false, false,
    //             false, false, false, true, true, true, true, true, false, false, false, false,
    //             false, true, true, true, true, true, false, false, false, false, false, true, true,
    //             true, true, true, true, false, false, false, false, false, false, true, true, true,
    //             true, true, true, false, false, false, false, false, false, true, true, true, true,
    //             true, true, false, false, false, false, false, false, true, true, true, true, true,
    //             true, true, true, true, true, true, false, false, false, false, false, false,
    //             false, true, true, true, true, true, true, true, false, false, false, false, false,
    //             false, false, true, true, true, true, true, true, true, false, false, false, false,
    //             false
    //         ]
    //     );
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(inputs);
    //     print_crt(result);
    //     assert_eq!(
    //         result,
    //         [
    //             true, true, true, false, false, true, true, true, true, false, true, false, false,
    //             true, false, true, true, true, true, false, false, true, true, false, false, true,
    //             true, true, false, false, true, true, true, true, false, true, true, true, true,
    //             false, true, false, false, true, false, true, false, false, false, false, true,
    //             false, true, false, false, false, false, false, true, false, true, false, false,
    //             true, false, true, false, false, true, false, true, false, false, false, false,
    //             true, false, false, false, false, true, false, false, true, false, true, true,
    //             true, false, false, true, true, false, false, false, false, false, true, false,
    //             false, true, false, false, false, false, true, false, false, true, false, true,
    //             true, true, false, false, true, true, true, false, false, true, true, true, false,
    //             false, true, false, false, false, false, true, false, true, false, false, false,
    //             true, false, false, false, true, false, false, false, false, true, true, true,
    //             false, false, true, false, false, false, false, true, false, false, false, false,
    //             true, false, true, false, false, true, false, false, false, false, true, false,
    //             true, false, false, true, false, false, false, false, true, false, false, true,
    //             false, true, false, false, false, false, true, false, false, false, false, true,
    //             false, false, false, false, true, false, false, true, false, true, false, false,
    //             false, false, true, false, false, true, false, true, true, true, true, false,
    //             false, true, true, false, false, true, false, false, false, false, true, true,
    //             true, true, false, true, false, false, false, false
    //         ]
    //     );
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
