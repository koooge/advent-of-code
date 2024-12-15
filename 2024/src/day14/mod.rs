pub fn solve_part1(inputs: &[String], wide: usize, tall: usize) -> usize {
    let mut robots: Vec<Vec<isize>> = vec![];

    fn parse_line(s: &str) -> Vec<isize> {
        s.split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<isize>().unwrap())
            .collect()
    }

    for input in inputs {
        robots.push(parse_line(input));
    }

    fn next(robot: &[isize], wide: usize, tall: usize) -> Vec<isize> {
        let x = if robot[0] + robot[2] < 0 {
            robot[0] + robot[2] + wide as isize
        } else if robot[0] + robot[2] >= wide as isize {
            robot[0] + robot[2] - wide as isize
        } else {
            robot[0] + robot[2]
        };
        let y = if robot[1] + robot[3] < 0 {
            robot[1] + robot[3] + tall as isize
        } else if robot[1] + robot[3] >= tall as isize {
            robot[1] + robot[3] - tall as isize
        } else {
            robot[1] + robot[3]
        };
        vec![x, y, robot[2], robot[3]]
    }

    for _ in 0..100 {
        for robot in &mut robots {
            *robot = next(robot, wide, tall);
        }
    }

    let mut ret: Vec<usize> = vec![0; 4];
    let center = ((wide - 1) / 2, (tall - 1) / 2);

    for robot in robots {
        if (robot[0] as usize) < center.0 && (robot[1] as usize) < center.1 {
            ret[0] += 1;
        } else if (robot[0] as usize) > center.0 && (robot[1] as usize) < center.1 {
            ret[1] += 1;
        } else if (robot[0] as usize) < center.0 && (robot[1] as usize) > center.1 {
            ret[2] += 1;
        } else if (robot[0] as usize) > center.0 && (robot[1] as usize) > center.1 {
            ret[3] += 1;
        }
    }
    ret.iter().product()
}

// pub fn solve_part2(inputs: &[String]) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day14/test1.txt");
        let result = solve_part1(&inputs, 11, 7);
        assert_eq!(result, 12);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day14/input1.txt");
        let result = solve_part1(&inputs, 101, 103);
        assert_eq!(result, 230900224);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day14/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 480); // stack over flow
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day14/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 272673043446478);
    // }
}
