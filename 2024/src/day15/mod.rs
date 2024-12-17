pub fn solve_part1(inputs: &[String]) -> usize {
    fn move_robot(
        map: &[Vec<char>],
        pos: (usize, usize),
        c: char,
    ) -> (Vec<Vec<char>>, (usize, usize)) {
        let mut m = map.to_vec();
        let mut p = pos;
        match c {
            '^' => {
                for i in (1..p.0).rev() {
                    if m[i][p.1] == '.' {
                        for k in i..p.0 {
                            let tmp = m[k][p.1];
                            m[k][p.1] = m[k + 1][p.1];
                            m[k + 1][p.1] = tmp;
                        }
                        p = (p.0 - 1, p.1);
                        break;
                    } else if m[i][p.1] == '#' {
                        break;
                    }
                }
            }
            '>' => {
                for i in (p.1 + 1)..(m[p.0].len() - 1) {
                    if m[p.0][i] == '.' {
                        for k in (p.1..i).rev() {
                            m[p.0].swap(k, k + 1);
                        }
                        p = (p.0, p.1 + 1);
                        break;
                    } else if m[pos.0][i] == '#' {
                        break;
                    }
                }
            }
            'v' => {
                for i in (p.0 + 1)..m.len() - 1 {
                    if m[i][p.1] == '.' {
                        for k in (p.0..i).rev() {
                            let tmp = m[k][p.1];
                            m[k][p.1] = m[k + 1][p.1];
                            m[k + 1][p.1] = tmp;
                        }
                        p = (p.0 + 1, p.1);
                        break;
                    } else if m[i][p.1] == '#' {
                        break;
                    }
                }
            }
            '<' => {
                for i in (1..p.1).rev() {
                    if m[p.0][i] == '.' {
                        for k in i..p.1 {
                            m[p.0].swap(k, k + 1);
                        }
                        p = (p.0, p.1 - 1);
                        break;
                    } else if m[p.0][i] == '#' {
                        break;
                    }
                }
            }
            _ => unreachable!(),
        }
        (m, p)
    }

    let mut map: Vec<Vec<char>> = vec![];
    let mut pos: (usize, usize) = Default::default();
    for (i, input) in inputs.iter().enumerate() {
        let line: Vec<char> = input.chars().collect();
        if line.is_empty() {
            continue;
        } else if line[0] == '#' {
            map.push(line.clone());
            if let Some(j) = line.iter().position(|&c| c == '@') {
                pos = (i, j);
            }
        } else {
            for c in line {
                (map, pos) = move_robot(&map, pos, c);
            }
        }
    }

    let mut ret = 0;
    for (i, m) in map.iter().enumerate() {
        for (j, c) in m.iter().enumerate() {
            if *c == 'O' {
                ret += 100 * i + j;
            }
        }
    }

    ret
}

pub fn solve_part2(inputs: &[String]) -> usize {
    fn try_up(map: &[Vec<char>], pos: (usize, usize)) -> Vec<Vec<char>> {
        fn move_up(map: &[Vec<char>], pos: (usize, usize)) -> Vec<Vec<char>> {
            let mut m = map.to_vec();
            m[pos.0 - 1][pos.1] = '[';
            m[pos.0 - 1][pos.1 + 1] = ']';
            m[pos.0][pos.1] = '.';
            m[pos.0][pos.1 + 1] = '.';
            m
        }

        let mut m = map.to_vec();
        if m[pos.0 - 1][pos.1] == '.' {
            if m[pos.0 - 1][pos.1 + 1] == '.' {
                m = move_up(&m, pos);
            } else if m[pos.0 - 1][pos.1 + 1] == '[' {
                let m_up = try_up(&m, (pos.0 - 1, pos.1 + 1));
                if m_up[pos.0 - 1][pos.1 + 1] == '.' {
                    m = move_up(&m_up, pos);
                }
            }
        } else if m[pos.0 - 1][pos.1] == '[' {
            let m_up = try_up(&m, (pos.0 - 1, pos.1));
            if m_up[pos.0 - 1][pos.1] == '.' {
                m = move_up(&m_up, pos);
            }
        } else if m[pos.0 - 1][pos.1] == ']' {
            let m_up_left = try_up(&m, (pos.0 - 1, pos.1 - 1));
            if m_up_left[pos.0 - 1][pos.1] == '.' {
                if m_up_left[pos.0 - 1][pos.1 + 1] == '.' {
                    m = move_up(&m_up_left, pos);
                } else if m_up_left[pos.0 - 1][pos.1 + 1] == '[' {
                    let m_up = try_up(&m_up_left, (pos.0 - 1, pos.1 + 1));
                    if m_up[pos.0 - 1][pos.1 + 1] == '.' {
                        m = move_up(&m_up, pos);
                    }
                }
            }
        }
        m
    }

    fn try_down(map: &[Vec<char>], pos: (usize, usize)) -> Vec<Vec<char>> {
        fn move_down(map: &[Vec<char>], pos: (usize, usize)) -> Vec<Vec<char>> {
            let mut m = map.to_vec();
            m[pos.0 + 1][pos.1] = '[';
            m[pos.0 + 1][pos.1 + 1] = ']';
            m[pos.0][pos.1] = '.';
            m[pos.0][pos.1 + 1] = '.';
            m
        }

        let mut m = map.to_vec();
        if m[pos.0 + 1][pos.1] == '.' {
            if m[pos.0 + 1][pos.1 + 1] == '.' {
                m = move_down(&m, pos);
            } else if m[pos.0 + 1][pos.1 + 1] == '[' {
                let m_down = try_down(&m, (pos.0 + 1, pos.1 + 1));
                if m_down[pos.0 + 1][pos.1 + 1] == '.' {
                    m = move_down(&m_down, pos);
                }
            }
        } else if m[pos.0 + 1][pos.1] == '[' {
            let m_down = try_down(&m, (pos.0 + 1, pos.1));
            if m_down[pos.0 + 1][pos.1] == '.' {
                m = move_down(&m_down, pos);
            }
        } else if m[pos.0 + 1][pos.1] == ']' {
            let m_down_left = try_down(&m, (pos.0 + 1, pos.1 - 1));
            if m_down_left[pos.0 + 1][pos.1] == '.' {
                if m[pos.0 + 1][pos.1 + 1] == '.' {
                    m = move_down(&m_down_left, pos);
                } else if m[pos.0 + 1][pos.1 + 1] == '[' {
                    let m_down = try_down(&m_down_left, (pos.0 + 1, pos.1 + 1));
                    if m_down[pos.0 + 1][pos.1 + 1] == '.' {
                        m = move_down(&m_down, pos);
                    }
                }
            }
        }
        m
    }

    fn move_robot(
        map: &[Vec<char>],
        pos: (usize, usize),
        c: char,
    ) -> (Vec<Vec<char>>, (usize, usize)) {
        let mut m = map.to_vec();
        let mut p = pos;
        match c {
            '^' => match m[p.0 - 1][p.1] {
                '.' => {
                    m[p.0 - 1][p.1] = '@';
                    m[p.0][p.1] = '.';
                    p = (p.0 - 1, p.1);
                }
                '[' => {
                    let m_up = try_up(&m, (p.0 - 1, p.1));
                    if m_up[p.0 - 1][p.1] == '.' {
                        m = m_up;
                        m[p.0 - 1][p.1] = '@';
                        m[p.0][p.1] = '.';
                        p = (p.0 - 1, p.1);
                    }
                }
                ']' => {
                    let m_up = try_up(&m, (p.0 - 1, p.1 - 1));
                    if m_up[p.0 - 1][p.1] == '.' {
                        m = m_up;
                        m[p.0 - 1][p.1] = '@';
                        m[p.0][p.1] = '.';
                        p = (p.0 - 1, p.1);
                    }
                }
                _ => (),
            },
            '>' => {
                for i in (p.1 + 1)..(m[p.0].len() - 1) {
                    if m[p.0][i] == '.' {
                        for k in (p.1..i).rev() {
                            m[p.0].swap(k, k + 1);
                        }
                        p = (p.0, p.1 + 1);
                        break;
                    } else if m[p.0][i] == '#' {
                        break;
                    }
                }
            }
            'v' => match m[p.0 + 1][p.1] {
                '.' => {
                    m[p.0 + 1][p.1] = '@';
                    m[p.0][p.1] = '.';
                    p = (p.0 + 1, p.1);
                }
                '[' => {
                    let m_down = try_down(&m, (p.0 + 1, p.1));
                    if m_down[p.0 + 1][p.1] == '.' {
                        m = m_down;
                        m[p.0 + 1][p.1] = '@';
                        m[p.0][p.1] = '.';
                        p = (p.0 + 1, p.1);
                    }
                }
                ']' => {
                    let m_down = try_down(&m, (p.0 + 1, p.1 - 1));
                    if m_down[p.0 + 1][p.1] == '.' {
                        m = m_down;
                        m[p.0 + 1][p.1] = '@';
                        m[p.0][p.1] = '.';
                        p = (p.0 + 1, p.1);
                    }
                }
                _ => (),
            },
            '<' => {
                for i in (1..p.1).rev() {
                    if m[p.0][i] == '.' {
                        for k in i..p.1 {
                            m[p.0].swap(k, k + 1);
                        }
                        p = (p.0, p.1 - 1);
                        break;
                    } else if m[p.0][i] == '#' {
                        break;
                    }
                }
            }
            _ => unreachable!(),
        }
        (m, p)
    }

    let mut map: Vec<Vec<char>> = vec![];
    let mut pos: (usize, usize) = Default::default();
    for (i, input) in inputs.iter().enumerate() {
        let line: Vec<char> = input.chars().collect();
        if line.is_empty() {
            continue;
        } else if line[0] == '#' {
            let mut m: Vec<char> = vec![];
            for c in line {
                match c {
                    '#' => {
                        m.push('#');
                        m.push('#');
                    }
                    'O' => {
                        m.push('[');
                        m.push(']');
                    }
                    '.' => {
                        m.push('.');
                        m.push('.');
                    }
                    '@' => {
                        m.push('@');
                        m.push('.');
                    }
                    _ => unreachable!(),
                }
            }
            if let Some(j) = m.iter().position(|&c| c == '@') {
                pos = (i, j);
            }
            map.push(m);
        } else {
            for c in line {
                (map, pos) = move_robot(&map, pos, c);
            }
        }
    }

    let mut ret = 0;
    for (i, m) in map.iter().enumerate() {
        for (j, c) in m.iter().enumerate() {
            if *c == '[' {
                ret += 100 * i + j;
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
        let inputs = read_file("./src/day15/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 2028);
    }

    #[test]
    fn part1_case2() {
        let inputs = read_file("./src/day15/test2.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 10092);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day15/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 1514353);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day15/test3.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 105 + 207 + 306);
    }

    #[test]
    fn part2_case2() {
        let inputs = read_file("./src/day15/test2.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 9021);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day15/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 1533076);
    }
}
