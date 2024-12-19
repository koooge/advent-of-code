pub fn solve_part1(inputs: &[String], len: usize, corrupted: usize) -> usize {
    let mut space: Vec<Vec<char>> = vec![vec!['.'; len]; len];
    let mut steps: Vec<Vec<usize>> = vec![vec![usize::MAX; len]; len];
    for input in inputs.iter().take(corrupted) {
        let coodinate: Vec<usize> = input.split(",").filter_map(|s| s.parse().ok()).collect();
        space[coodinate[1]][coodinate[0]] = '#';
    }

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

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day18/test2.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 117440);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day18/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 533);
    // }
}
