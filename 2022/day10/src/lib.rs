pub fn solve_part1(inputs: Vec<String>) -> i32 {
    let mut cycle: u32 = 0;
    let mut ret: i32 = 0;
    let mut reg_v: i32 = 1;

    for input in inputs {
        let ins: Vec<&str> = input.split(' ').collect();
        let opcode: &str = ins.get(0).unwrap();

        if opcode == "addx" {
            if [18, 58, 98, 138, 178, 218].contains(&cycle) {
                ret = ret + ((cycle as i32 + 2) * reg_v);
            } else if [19, 59, 99, 139, 179, 219].contains(&cycle) {
                ret = ret + ((cycle as i32 + 1) * reg_v);
            }
            let x: i32 = ins.get(1).unwrap().parse().unwrap();
            reg_v += x;
            cycle += 2;
        } else {
            // noop
            if [19, 59, 99, 139, 179, 219].contains(&cycle) {
                ret = ret + ((cycle as i32 + 1) * reg_v);
            }
            cycle += 1;
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
        assert_eq!(result, 13140);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 13760);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test2.txt");
    //     let result = solve_part2(inputs);
    //     println!("{}", result);
    //     assert_eq!(result, 36);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(inputs);
    //     println!("{}", result);
    //     assert_eq!(result, 2522);
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
