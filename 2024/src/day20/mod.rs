pub fn solve_part1(inputs: &[String], at_least: usize) -> usize {
    let mut map: Vec<Vec<char>> = vec![vec![Default::default(); inputs[0].len()]; inputs.len()];
    let mut start: (usize, usize) = Default::default();
    let mut end: (usize, usize) = Default::default();

    fn move_top(
        map: &[Vec<char>],
        y: usize,
        x: usize,
        ps: usize,
        can_cheat: bool,
    ) -> Option<(usize, usize, usize, bool)> {
        if can_cheat
            && y > 1
            && map[y - 1][x] == '#'
            && (map[y - 2][x] == '.' || map[y - 2][x] == 'E')
        {
            return Some((y - 2, x, ps + 2, false));
        } else if y > 0 && (map[y - 1][x] == '.' || map[y - 1][x] == 'E') {
            return Some((y - 1, x, ps + 1, can_cheat));
        }
        None
    }

    fn move_right(
        map: &[Vec<char>],
        y: usize,
        x: usize,
        ps: usize,
        can_cheat: bool,
    ) -> Option<(usize, usize, usize, bool)> {
        if can_cheat
            && x < map[y].len() - 2
            && map[y][x + 1] == '#'
            && (map[y][x + 2] == '.' || map[y][x + 2] == 'E')
        {
            return Some((y, x + 2, ps + 2, false));
        } else if x < map[y].len() - 1 && (map[y][x + 1] == '.' || map[y][x + 1] == 'E') {
            return Some((y, x + 1, ps + 1, can_cheat));
        }
        None
    }

    fn move_bottom(
        map: &[Vec<char>],
        y: usize,
        x: usize,
        ps: usize,
        can_cheat: bool,
    ) -> Option<(usize, usize, usize, bool)> {
        if can_cheat
            && y < map.len() - 2
            && map[y + 1][x] == '#'
            && (map[y + 2][x] == '.' || map[y + 2][x] == 'E')
        {
            return Some((y + 2, x, ps + 2, false));
        } else if y > 0 && (map[y + 1][x] == '.' || map[y + 1][x] == 'E') {
            return Some((y + 1, x, ps + 1, can_cheat));
        }
        None
    }

    fn move_left(
        map: &[Vec<char>],
        y: usize,
        x: usize,
        ps: usize,
        can_cheat: bool,
    ) -> Option<(usize, usize, usize, bool)> {
        if can_cheat
            && x > 1
            && map[y][x - 1] == '#'
            && (map[y][x - 2] == '.' || map[y][x - 2] == 'E')
        {
            return Some((y, x - 2, ps + 2, false));
        } else if x > 0 && (map[y][x - 1] == '.' || map[y][x - 1] == 'E') {
            return Some((y, x - 1, ps + 1, can_cheat));
        }
        None
    }

    for (i, input) in inputs.iter().enumerate() {
        for (j, c) in input.chars().enumerate() {
            match c {
                'S' => start = (i, j),
                'E' => end = (i, j),
                _ => (),
            }
            map[i][j] = c;
        }
    }
    let mut ref_time: Vec<Vec<usize>> = vec![vec![usize::MAX; map[0].len()]; map.len()];
    ref_time[start.0][start.1] = 0;
    let mut positions: Vec<(usize, usize, usize, usize, bool)> =
        vec![(start.0, start.1, 4, 0, false)];
    while let Some((y, x, from, ps, _)) = positions.pop() {
        if y == end.0 && x == end.1 {
            break;
        }
        if from != 0 {
            if let Some(next) = move_top(&map, y, x, ps, false) {
                ref_time[next.0][next.1] = next.2;
                positions.push((next.0, next.1, 2, next.2, false));
            }
        }
        if from != 1 {
            if let Some(next) = move_right(&map, y, x, ps, false) {
                ref_time[next.0][next.1] = next.2;
                positions.push((next.0, next.1, 3, next.2, false));
            }
        }
        if from != 2 {
            if let Some(next) = move_bottom(&map, y, x, ps, false) {
                ref_time[next.0][next.1] = next.2;
                positions.push((next.0, next.1, 0, next.2, false));
            }
        }
        if from != 3 {
            if let Some(next) = move_left(&map, y, x, ps, false) {
                ref_time[next.0][next.1] = next.2;
                positions.push((next.0, next.1, 1, next.2, false));
            }
        }
    }

    let mut ret: usize = 0;
    let mut positions: Vec<(usize, usize, usize, usize, bool)> =
        vec![(start.0, start.1, 4, 0, true)];
    while let Some((y, x, from, ps, can_cheat)) = positions.pop() {
        if y == end.0 && x == end.1 {
            if can_cheat && ref_time[end.0][end.1] - ps >= at_least {
                ret += 1;
            }
            continue;
        }
        if from != 0 {
            if let Some(next) = move_top(&map, y, x, ps, can_cheat) {
                if !next.3 {
                    if ref_time[next.0][next.1] > ref_time[y][x]
                        && ref_time[next.0][next.1] - next.2 >= at_least
                    {
                        ret += 1;
                    }
                } else {
                    positions.push((next.0, next.1, 2, next.2, true));
                }
            }
        }
        if from != 1 {
            if let Some(next) = move_right(&map, y, x, ps, can_cheat) {
                if !next.3 {
                    if ref_time[next.0][next.1] > ref_time[y][x]
                        && ref_time[next.0][next.1] - next.2 >= at_least
                    {
                        ret += 1;
                    }
                } else {
                    positions.push((next.0, next.1, 3, next.2, true));
                }
            }
        }
        if from != 2 {
            if let Some(next) = move_bottom(&map, y, x, ps, can_cheat) {
                if !next.3 {
                    if ref_time[next.0][next.1] > ref_time[y][x]
                        && ref_time[next.0][next.1] - next.2 >= at_least
                    {
                        ret += 1;
                    }
                } else {
                    positions.push((next.0, next.1, 0, next.2, true));
                }
            }
        }
        if from != 3 {
            if let Some(next) = move_left(&map, y, x, ps, can_cheat) {
                if !next.3 {
                    if ref_time[next.0][next.1] > ref_time[y][x]
                        && ref_time[next.0][next.1] - next.2 >= at_least
                    {
                        ret += 1;
                    }
                } else {
                    positions.push((next.0, next.1, 1, next.2, true));
                }
            }
        }
    }

    ret
}

// pub fn solve_part2(inputs: &[String]) -> String {
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day20/test1.txt");
        let result = solve_part1(&inputs, 1);
        assert_eq!(result, 14 + 14 + 2 + 4 + 2 + 3 + 5);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day20/input1.txt");
        let result = solve_part1(&inputs, 100);
        assert_eq!(result, 1286);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day20/test1.txt");
    //     let result = solve_part2(&inputs, 7);
    //     assert_eq!(result, String::from("6,1"));
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day20/input1.txt");
    //     let result = solve_part2(&inputs, 71);
    //     assert_eq!(result, String::from("20,64"));
    // }
}
