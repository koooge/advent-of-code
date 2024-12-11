use std::collections::HashSet;

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut map: Vec<Vec<usize>> = vec![];
    let mut trailheads: Vec<(usize, usize)> = vec![];
    for (i, input) in inputs.iter().enumerate() {
        let mut row: Vec<usize> = vec![];
        for (j, c) in input.chars().enumerate() {
            let n = c.to_digit(10).unwrap() as usize;
            if n == 0 {
                trailheads.push((i, j));
            }
            row.push(n);
        }
        map.push(row);
    }

    fn trail(
        height: usize,
        map: &[Vec<usize>],
        pos: (usize, usize),
        prev_path: &[(usize, usize)],
    ) -> Vec<Vec<(usize, usize)>> {
        let mut path = prev_path.to_vec();
        path.push(pos);
        if height == 9 {
            return vec![path.to_vec()];
        }

        let mut ret = vec![];
        // top
        if pos.0 > 0 && map[pos.0 - 1][pos.1] == height + 1 {
            let paths = trail(height + 1, map, (pos.0 - 1, pos.1), &path);
            ret.extend(paths);
        }
        // right
        if pos.1 < map[pos.0].len() - 1 && map[pos.0][pos.1 + 1] == height + 1 {
            let paths = trail(height + 1, map, (pos.0, pos.1 + 1), &path);
            ret.extend(paths);
        }
        // bottom
        if pos.0 < map.len() - 1 && map[pos.0 + 1][pos.1] == height + 1 {
            let paths = trail(height + 1, map, (pos.0 + 1, pos.1), &path);
            ret.extend(paths);
        }
        // left
        if pos.1 > 0 && map[pos.0][pos.1 - 1] == height + 1 {
            let paths = trail(height + 1, map, (pos.0, pos.1 - 1), &path);
            ret.extend(paths);
        }
        ret
    }

    let mut scores: Vec<usize> = vec![];
    for trailhead in trailheads {
        let path: Vec<(usize, usize)> = vec![];
        let paths: Vec<Vec<(usize, usize)>> = trail(0, &map, trailhead, &path);
        let mut goals: HashSet<(usize, usize)> = HashSet::new();
        for p in paths {
            goals.insert(*p.last().unwrap());
        }

        scores.push(goals.len());
    }

    scores.iter().sum()
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut map: Vec<Vec<usize>> = vec![];
    let mut trailheads: Vec<(usize, usize)> = vec![];
    for (i, input) in inputs.iter().enumerate() {
        let mut row: Vec<usize> = vec![];
        for (j, c) in input.chars().enumerate() {
            let n = c.to_digit(10).unwrap() as usize;
            if n == 0 {
                trailheads.push((i, j));
            }
            row.push(n);
        }
        map.push(row);
    }

    fn trail(
        height: usize,
        map: &[Vec<usize>],
        pos: (usize, usize),
        prev_path: &[(usize, usize)],
    ) -> Vec<Vec<(usize, usize)>> {
        let mut path = prev_path.to_vec();
        path.push(pos);
        if height == 9 {
            return vec![path.to_vec()];
        }

        let mut ret = vec![];
        // top
        if pos.0 > 0 && map[pos.0 - 1][pos.1] == height + 1 {
            let paths = trail(height + 1, map, (pos.0 - 1, pos.1), &path);
            ret.extend(paths);
        }
        // right
        if pos.1 < map[pos.0].len() - 1 && map[pos.0][pos.1 + 1] == height + 1 {
            let paths = trail(height + 1, map, (pos.0, pos.1 + 1), &path);
            ret.extend(paths);
        }
        // bottom
        if pos.0 < map.len() - 1 && map[pos.0 + 1][pos.1] == height + 1 {
            let paths = trail(height + 1, map, (pos.0 + 1, pos.1), &path);
            ret.extend(paths);
        }
        // left
        if pos.1 > 0 && map[pos.0][pos.1 - 1] == height + 1 {
            let paths = trail(height + 1, map, (pos.0, pos.1 - 1), &path);
            ret.extend(paths);
        }
        ret
    }

    let mut scores: Vec<usize> = vec![];
    for trailhead in trailheads {
        let path: Vec<(usize, usize)> = vec![];
        let paths: Vec<Vec<(usize, usize)>> = trail(0, &map, trailhead, &path);
        scores.push(paths.len());
    }

    scores.iter().sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day10/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 36);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day10/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 582);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day10/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 81);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day10/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 1302);
    }
}
