pub fn solve_part1(inputs: &[String]) -> u32 {
    let mut adj: u32 = 0;
    let mut cubes: Vec<(u32, u32, u32)> = Vec::new();

    for input in inputs {
        let points: Vec<u32> = input.split(',').map(|x| x.parse().unwrap()).collect();
        let (x, y, z) = (points[0], points[1], points[2]);
        for cube in &cubes {
            if (cube.0 == x && cube.1 == y && cube.2.abs_diff(z) == 1)
                || (cube.0.abs_diff(x) == 1 && cube.1 == y && cube.2 == z)
                || (cube.0 == x && cube.1.abs_diff(y) == 1 && cube.2 == z)
            {
                adj += 1;
            }
        }
        cubes.push((x, y, z));
    }

    (cubes.len() as u32 * 6) - (adj * 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs);
        println!("{}", result);
        assert_eq!(result, 64);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(&inputs);
        println!("{}", result);
        assert_eq!(result, 3412);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 1514285714288);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 1525364431487);
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
