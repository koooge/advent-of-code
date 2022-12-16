use std::collections::HashSet;

pub fn solve_part1(inputs: &[String], target_y: i32) -> u32 {
    let mut nobeacon: HashSet<i32> = HashSet::new();
    let mut beacon_ony: HashSet<i32> = HashSet::new();

    for input in inputs {
        let strs: Vec<&str> = input.split(' ').collect();
        let sx: i32 = strs[2].split('=').collect::<Vec<&str>>()[1]
            .split(',')
            .collect::<Vec<&str>>()[0]
            .parse()
            .unwrap();
        let sy: i32 = strs[3].split('=').collect::<Vec<&str>>()[1]
            .split(':')
            .collect::<Vec<&str>>()[0]
            .parse()
            .unwrap();
        let bx: i32 = strs[8].split('=').collect::<Vec<&str>>()[1]
            .split(',')
            .collect::<Vec<&str>>()[0]
            .parse()
            .unwrap();
        let by: i32 = strs[9].split('=').collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        if by == target_y {
            beacon_ony.insert(bx);
        }

        // is overlap then nobeacon
        let d = sx.abs_diff(bx) + sy.abs_diff(by);
        let d_target = sy.abs_diff(target_y);
        if d_target > d {
            continue;
        }

        for i in 0..=(d - d_target) {
            nobeacon.insert(sx + i as i32);
            nobeacon.insert(sx - i as i32);
        }
    }

    nobeacon.len() as u32 - beacon_ony.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs, 10);
        println!("{}", result);
        assert_eq!(result, 26);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(&inputs, 2000000);
        println!("{}", result);
        assert_eq!(result, 5166077);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 93);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 26845);
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
