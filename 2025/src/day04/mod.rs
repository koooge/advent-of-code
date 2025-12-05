fn solve(grid: &[Vec<char>]) -> (usize, Vec<Vec<char>>) {
    let mut ret = 0;
    let mut new_grid: Vec<Vec<char>> = grid.to_vec();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '@' {
                continue;
            }

            let mut rolls = 0;
            if y > 0 {
                if x > 0 && grid[y - 1][x - 1] == '@' {
                    rolls += 1;
                }
                if grid[y - 1][x] == '@' {
                    rolls += 1;
                }
                if x < grid[y - 1].len() - 1 && grid[y - 1][x + 1] == '@' {
                    rolls += 1;
                }
            }
            if x > 0 && grid[y][x - 1] == '@' {
                rolls += 1;
            }
            if x < grid[y].len() - 1 && grid[y][x + 1] == '@' {
                rolls += 1;
            }
            if y < grid.len() - 1 {
                if x > 0 && grid[y + 1][x - 1] == '@' {
                    rolls += 1;
                }
                if grid[y + 1][x] == '@' {
                    rolls += 1;
                }
                if x < grid[y + 1].len() - 1 && grid[y + 1][x + 1] == '@' {
                    rolls += 1;
                }
            }

            if rolls < 4 {
                ret += 1;
                new_grid[y][x] = '.';
            }
        }
    }

    (ret, new_grid.to_vec())
}

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut grid: Vec<Vec<char>> = vec![];

    for line in inputs {
        grid.push(line.chars().collect());
    }

    let (rolls, _) = solve(&grid);
    rolls
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut grid: Vec<Vec<char>> = vec![];

    for line in inputs {
        grid.push(line.chars().collect());
    }

    let mut total = 0;
    let (mut rolls, mut new_grid) = solve(&grid);
    total += rolls;
    while rolls != 0 {
        (rolls, new_grid) = solve(&new_grid);
        total += rolls;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn day04_part1_case1() {
        let inputs = read_file("./src/day04/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 13);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day04/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 1320);
    }

    #[test]
    fn day04_part2_case1() {
        let inputs = read_file("./src/day04/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 43);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day04/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 8354);
    }
}
