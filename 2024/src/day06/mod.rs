pub fn solve_part1(inputs: &[String]) -> usize {
    let mut map: Vec<Vec<char>> = vec![];
    let mut pos: (usize, usize) = (usize::MAX, usize::MAX);

    for (i, input) in inputs.iter().enumerate() {
        let row: Vec<char> = input.chars().collect();
        map.push(row);
        for (j, c) in input.chars().enumerate() {
            if c == '^' {
                pos = (i, j);
            }
        }
    }

    let mut is_goal = false;
    while !is_goal {
        match map[pos.0][pos.1] {
            '^' => {
                if pos.0 > 0 {
                    match map[pos.0 - 1][pos.1] {
                        '.' | 'X' => {
                            map[pos.0][pos.1] = 'X';
                            map[pos.0 - 1][pos.1] = '^';
                            pos = (pos.0 - 1, pos.1);
                        }
                        '#' => {
                            map[pos.0][pos.1] = '>';
                        }
                        _ => unreachable!(),
                    }
                } else {
                    map[pos.0][pos.1] = 'X';
                    is_goal = true;
                }
            }
            '>' => {
                if pos.1 < inputs[0].len() - 1 {
                    match map[pos.0][pos.1 + 1] {
                        '.' | 'X' => {
                            map[pos.0][pos.1] = 'X';
                            map[pos.0][pos.1 + 1] = '>';
                            pos = (pos.0, pos.1 + 1);
                        }
                        '#' => {
                            map[pos.0][pos.1] = 'v';
                        }
                        _ => unreachable!(),
                    }
                } else {
                    map[pos.0][pos.1] = 'X';
                    is_goal = true;
                }
            }
            'v' => {
                if pos.0 < inputs.len() - 1 {
                    match map[pos.0 + 1][pos.1] {
                        '.' | 'X' => {
                            map[pos.0][pos.1] = 'X';
                            map[pos.0 + 1][pos.1] = 'v';
                            pos = (pos.0 + 1, pos.1);
                        }
                        '#' => {
                            map[pos.0][pos.1] = '<';
                        }
                        _ => unreachable!(),
                    }
                } else {
                    map[pos.0][pos.1] = 'X';
                    is_goal = true;
                }
            }
            '<' => {
                if pos.1 > 0 {
                    match map[pos.0][pos.1 - 1] {
                        '.' | 'X' => {
                            map[pos.0][pos.1] = 'X';
                            map[pos.0][pos.1 - 1] = '<';
                            pos = (pos.0, pos.1 - 1);
                        }
                        '#' => {
                            map[pos.0][pos.1] = '^';
                        }
                        _ => unreachable!(),
                    }
                } else {
                    map[pos.0][pos.1] = 'X';
                    is_goal = true;
                }
            }
            _ => unreachable!(),
        }
    }

    map.iter()
        .map(|row| row.iter().filter(|&&c| c == 'X').count())
        .sum()
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut map: Vec<Vec<char>> = vec![];

    for input in inputs {
        let row: Vec<char> = input.chars().collect();
        map.push(row);
    }

    fn find_obstacle(map: &[Vec<char>], i: usize, j: usize, d: usize) -> Option<(usize, usize)> {
        if map[i][j] == '#' {
            return None;
        }

        match d {
            0 => {
                for k in (1..=i).rev() {
                    if map[k - 1][j] == '#' {
                        return Some((k, j));
                    }
                }
            }
            1 => {
                for k in j..(map[i].len() - 1) {
                    if map[i][k + 1] == '#' {
                        return Some((i, k));
                    }
                }
            }
            2 => {
                for k in i..(map.len() - 1) {
                    if map[k + 1][j] == '#' {
                        return Some((k, j));
                    }
                }
            }
            3 => {
                for k in (1..=j).rev() {
                    if map[i][k - 1] == '#' {
                        return Some((i, k));
                    }
                }
            }
            _ => unreachable!(),
        }

        None
    }

    let mut ret = 0;
    for (i, input) in inputs.iter().enumerate() {
        for j in 0..input.len() {
            let mut new_map = map.clone();
            new_map[i][j] = '#';
            // top
            if i > 0 {
                let mut pos = (i - 1, j);
                let mut d = 3;
                while let Some(cur) = find_obstacle(&new_map, pos.0, pos.1, d) {
                    if d == 2 && cur == (i - 1, j) {
                        ret += 1;
                        break;
                    } else if (d == 3 && cur == (i, j + 1))
                        || (d == 0 && cur == (i + 1, j))
                        || (d == 1 && j > 0 && cur == (i, j - 1))
                    {
                        break;
                    }
                    pos = cur;
                    d = if d == 3 { 0 } else { d + 1 }
                }
            }

            // right
            if j < input.len() - 1 {
                let mut pos = (i, j + 1);
                let mut d = 0;
                while let Some(cur) = find_obstacle(&new_map, pos.0, pos.1, d) {
                    if d == 3 && cur == (i, j + 1) {
                        ret += 1;
                        break;
                    } else if (d == 0 && cur == (i + 1, j))
                        || (d == 1 && j > 0 && cur == (i, j - 1))
                        || (d == 2 && i > 0 && cur == (i - 1, j))
                    {
                        break;
                    }
                    pos = cur;
                    d = if d == 3 { 0 } else { d + 1 }
                }
            }

            // bottom
            if i < inputs.len() - 1 {
                let mut pos = (i + 1, j);
                let mut d = 1;
                while let Some(cur) = find_obstacle(&new_map, pos.0, pos.1, d) {
                    if d == 0 && cur == (i + 1, j) {
                        ret += 1;
                        break;
                    } else if (d == 1 && j > 0 && cur == (i, j - 1))
                        || (d == 2 && i > 0 && cur == (i - 1, j))
                        || (d == 3 && cur == (i, j + 1))
                    {
                        break;
                    }
                    pos = cur;
                    d = if d == 3 { 0 } else { d + 1 }
                }
            }

            // left
            if j > 0 {
                let mut pos = (i, j - 1);
                let mut d = 2;
                while let Some(cur) = find_obstacle(&new_map, pos.0, pos.1, d) {
                    if d == 1 && cur == (i, j - 1) {
                        ret += 1;
                        break;
                    } else if (i > 0 && cur == (i - 1, j)) || cur == (i, j + 1) || cur == (i + 1, j)
                    {
                        break;
                    }
                    pos = cur;
                    d = if d == 3 { 0 } else { d + 1 }
                }
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day06/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 41);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day06/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 5153);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day06/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 6);
    }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day06/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 1);
    // }
}
