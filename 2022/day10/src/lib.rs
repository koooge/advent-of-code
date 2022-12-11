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

fn get_pixel(cycle: u32, sprite: i32) -> bool {
    let col: i32 = i32::try_from(cycle).unwrap() % 40;
    [sprite - 1, sprite, sprite + 1].contains(&col)
}

pub fn solve_part2(inputs: Vec<String>) -> [bool; 240] {
    let mut cycle: u32 = 0;
    let mut reg_v: i32 = 1; // 1 to 38
    let mut crt: [bool; 240] = [false; 240];

    for input in inputs {
        let ins: Vec<&str> = input.split(' ').collect();
        let opcode: &str = ins.get(0).unwrap();

        crt[cycle as usize] = get_pixel(cycle, reg_v);
        cycle += 1;

        if opcode == "addx" {
            crt[cycle as usize] = get_pixel(cycle, reg_v);
            cycle += 1;
            let x: i32 = ins.get(1).unwrap().parse().unwrap();
            reg_v += x;
        }
    }

    crt
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

    fn print_crt(crt: [bool; 240]) {
        for i in 0..240 {
            if crt[i] {
                print!("#");
            } else {
                print!(".");
            }
            if (i + 1) % 40 == 0 {
                println!("");
            }
        }
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part2(inputs);
        print_crt(result);
        assert_eq!(
            result,
            [
                true, true, false, false, true, true, false, false, true, true, false, false, true,
                true, false, false, true, true, false, false, true, true, false, false, true, true,
                false, false, true, true, false, false, true, true, false, false, true, true,
                false, false, true, true, true, false, false, false, true, true, true, false,
                false, false, true, true, true, false, false, false, true, true, true, false,
                false, false, true, true, true, false, false, false, true, true, true, false,
                false, false, true, true, true, false, true, true, true, true, false, false, false,
                false, true, true, true, true, false, false, false, false, true, true, true, true,
                false, false, false, false, true, true, true, true, false, false, false, false,
                true, true, true, true, false, false, false, false, true, true, true, true, true,
                false, false, false, false, false, true, true, true, true, true, false, false,
                false, false, false, true, true, true, true, true, false, false, false, false,
                false, true, true, true, true, true, false, false, false, false, false, true, true,
                true, true, true, true, false, false, false, false, false, false, true, true, true,
                true, true, true, false, false, false, false, false, false, true, true, true, true,
                true, true, false, false, false, false, false, false, true, true, true, true, true,
                true, true, true, true, true, true, false, false, false, false, false, false,
                false, true, true, true, true, true, true, true, false, false, false, false, false,
                false, false, true, true, true, true, true, true, true, false, false, false, false,
                false
            ]
        );
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part2(inputs);
        print_crt(result);
        assert_eq!(
            result,
            [
                true, true, true, false, false, true, true, true, true, false, true, false, false,
                true, false, true, true, true, true, false, false, true, true, false, false, true,
                true, true, false, false, true, true, true, true, false, true, true, true, true,
                false, true, false, false, true, false, true, false, false, false, false, true,
                false, true, false, false, false, false, false, true, false, true, false, false,
                true, false, true, false, false, true, false, true, false, false, false, false,
                true, false, false, false, false, true, false, false, true, false, true, true,
                true, false, false, true, true, false, false, false, false, false, true, false,
                false, true, false, false, false, false, true, false, false, true, false, true,
                true, true, false, false, true, true, true, false, false, true, true, true, false,
                false, true, false, false, false, false, true, false, true, false, false, false,
                true, false, false, false, true, false, false, false, false, true, true, true,
                false, false, true, false, false, false, false, true, false, false, false, false,
                true, false, true, false, false, true, false, false, false, false, true, false,
                true, false, false, true, false, false, false, false, true, false, false, true,
                false, true, false, false, false, false, true, false, false, false, false, true,
                false, false, false, false, true, false, false, true, false, true, false, false,
                false, false, true, false, false, true, false, true, true, true, true, false,
                false, true, true, false, false, true, false, false, false, false, true, true,
                true, true, false, true, false, false, false, false
            ]
        );
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
