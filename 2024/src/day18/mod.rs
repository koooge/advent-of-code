fn get_steps(space: &[Vec<char>], len: usize) -> usize {
    let mut steps: Vec<Vec<usize>> = vec![vec![usize::MAX; len]; len];
    let mut positions: Vec<(usize, usize)> = vec![(0, 0)];
    steps[0][0] = 0;
    while !positions.is_empty() {
        let mut po: Vec<(usize, usize)> = vec![];
        for (y, x) in positions {
            // top
            if y > 0 && space[y - 1][x] != '#' && steps[y][x] + 1 < steps[y - 1][x] {
                steps[y - 1][x] = steps[y][x] + 1;
                po.push((y - 1, x));
            }
            // right
            if x < space[y].len() - 1 && space[y][x + 1] != '#' && steps[y][x] + 1 < steps[y][x + 1]
            {
                steps[y][x + 1] = steps[y][x] + 1;
                po.push((y, x + 1));
            }
            // bottom
            if y < space.len() - 1 && space[y + 1][x] != '#' && steps[y][x] + 1 < steps[y + 1][x] {
                steps[y + 1][x] = steps[y][x] + 1;
                po.push((y + 1, x));
            }
            // left
            if x > 0 && space[y][x - 1] != '#' && steps[y][x] + 1 < steps[y][x - 1] {
                steps[y][x - 1] = steps[y][x] + 1;
                po.push((y, x - 1));
            }
        }
        positions = po;
    }

    steps[len - 1][len - 1]
}

pub fn solve_part1(inputs: &[String], len: usize, corrupted: usize) -> usize {
    let mut space: Vec<Vec<char>> = vec![vec!['.'; len]; len];
    for input in inputs.iter().take(corrupted) {
        let coordinate: Vec<usize> = input.split(",").filter_map(|s| s.parse().ok()).collect();
        space[coordinate[1]][coordinate[0]] = '#';
    }

    get_steps(&space, len)
}

pub fn solve_part2(inputs: &[String], len: usize) -> String {
    let mut space: Vec<Vec<char>> = vec![vec!['.'; len]; len];
    for input in inputs {
        let coordinate: Vec<usize> = input.split(",").filter_map(|s| s.parse().ok()).collect();
        space[coordinate[1]][coordinate[0]] = '#';
        let steps = get_steps(&space, len);
        if steps == usize::MAX {
            return input.to_string();
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day18/test1.txt");
        let result = solve_part1(&inputs, 7, 12);
        assert_eq!(result, 22);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day18/input1.txt");
        let result = solve_part1(&inputs, 71, 1024);
        assert_eq!(result, 286);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day18/test1.txt");
        let result = solve_part2(&inputs, 7);
        assert_eq!(result, String::from("6,1"));
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day18/input1.txt");
        let result = solve_part2(&inputs, 71);
        assert_eq!(result, String::from("20,64"));
    }
}
