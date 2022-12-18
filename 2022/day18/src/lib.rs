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

fn neighbors((x, y, z): (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

use std::collections::HashSet;

pub fn solve_part2(inputs: &[String]) -> u32 {
    let mut cubes: Vec<(i32, i32, i32)> = Vec::new();
    let (mut x_min, mut x_max, mut y_min, mut y_max, mut z_min, mut z_max) =
        (i32::MAX, 0, i32::MAX, 0, i32::MAX, 0);

    for input in inputs {
        let points: Vec<i32> = input.split(',').map(|x| x.parse().unwrap()).collect();
        let (x, y, z) = (points[0], points[1], points[2]);
        cubes.push((x, y, z));
        x_min = std::cmp::min(x_min, x);
        x_max = std::cmp::max(x_max, x);
        y_min = std::cmp::min(y_min, y);
        y_max = std::cmp::max(y_max, y);
        z_min = std::cmp::min(z_min, z);
        z_max = std::cmp::max(z_max, z);
    }

    let mut queue = vec![(z_min - 1, y_min - 1, z_min - 1)];
    let mut outside: HashSet<(i32, i32, i32)> = queue.iter().copied().collect();
    while let Some(point) = queue.pop() {
        for (x, y, z) in neighbors(point) {
            if (x_min - 1..=x_max + 1).contains(&x)
                && (y_min - 1..=y_max + 1).contains(&y)
                && (z_min - 1..=z_max + 1).contains(&z)
                && !cubes.contains(&(x, y, z))
                && outside.insert((x, y, z))
            {
                queue.push((x, y, z));
            }
        }
    }

    cubes
        .iter()
        .copied()
        .flat_map(neighbors)
        .filter(|point| outside.contains(point))
        .count() as u32
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

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 58);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 2018);
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
