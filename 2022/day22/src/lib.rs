fn move_to(tiles: &[Vec<char>], pos: &(usize, usize, u32), n: u32) -> (usize, usize, u32) {
    if n == 0 {
        return (pos.0, pos.1, pos.2);
    }

    match pos.2 {
        0 => {
            let next_col: usize = if pos.1 == tiles[pos.0].len() - 1 {
                0
            } else {
                pos.1 + 1
            };

            match tiles[pos.0][next_col] {
                '#' => (pos.0, pos.1, pos.2),
                '.' => move_to(tiles, &(pos.0, next_col, pos.2), n - 1),
                _ => {
                    let col = tiles[pos.0].iter().position(|&x| x != ' ').unwrap();
                    if tiles[pos.0][col] == '#' {
                        (pos.0, pos.1, pos.2)
                    } else {
                        move_to(tiles, &(pos.0, col, pos.2), n - 1)
                    }
                }
            }
        }
        1 => {
            let next_row: usize = if pos.0 == tiles.len() - 1 {
                0
            } else {
                pos.0 + 1
            };

            match tiles[next_row].get(pos.1) {
                Some('#') => (pos.0, pos.1, pos.2),
                Some('.') => move_to(tiles, &(next_row, pos.1, pos.2), n - 1),
                _ => {
                    for i in 0.. {
                        if tiles[i][pos.1] == ' ' {
                            continue;
                        }

                        if tiles[i][pos.1] == '#' {
                            return (pos.0, pos.1, pos.2);
                        } else {
                            return move_to(tiles, &(i, pos.1, pos.2), n - 1);
                        }
                    }
                    unreachable!();
                }
            }
        }
        2 => {
            let next_col: usize = if pos.1 == 0 {
                tiles[pos.0].len() - 1
            } else {
                pos.1 - 1
            };

            match tiles[pos.0][next_col] {
                '#' => (pos.0, pos.1, pos.2),
                '.' => move_to(tiles, &(pos.0, next_col, pos.2), n - 1),
                _ => {
                    let col = tiles[pos.0].iter().rposition(|&x| x != ' ').unwrap();
                    if tiles[pos.0][col] == '#' {
                        (pos.0, pos.1, pos.2)
                    } else {
                        move_to(tiles, &(pos.0, col, pos.2), n - 1)
                    }
                }
            }
        }
        3 => {
            let next_row: usize = if pos.0 == 0 {
                tiles.len() - 1
            } else {
                pos.0 - 1
            };

            match tiles[next_row].get(pos.1) {
                Some('#') => (pos.0, pos.1, pos.2),
                Some('.') => move_to(tiles, &(next_row, pos.1, pos.2), n - 1),
                _ => {
                    for i in (0..tiles.len()).rev() {
                        if tiles[i].get(pos.1) == None || tiles[i][pos.1] == ' ' {
                            continue;
                        }

                        if tiles[i][pos.1] == '#' {
                            return (pos.0, pos.1, pos.2);
                        } else {
                            return move_to(tiles, &(i, pos.1, pos.2), n - 1);
                        }
                    }
                    unreachable!();
                }
            }
        }
        _ => unreachable!(),
    }
}

pub fn solve_part1(inputs: &[String]) -> u32 {
    let mut tiles: Vec<Vec<char>> = Vec::new();

    for input in inputs.iter() {
        match input.chars().nth(0) {
            Some(' ') | Some('.') | Some('#') => tiles.push(input.chars().collect()),
            _ => break,
        };
    }
    let (mut path, mut s) = (Vec::<String>::new(), String::new());
    for c in inputs.last().unwrap().chars() {
        if c.is_numeric() {
            s.push(c);
        } else {
            if let Ok(n) = s.parse() {
                path.push(n);
            }
            s.clear();
            if let Ok(rl) = String::from(c).parse() {
                path.push(rl);
            }
        }
    }
    path.push(s);

    let mut pos: (usize, usize, u32) = (0, 0, 0);
    pos.1 = tiles[0].iter().position(|&x| x == '.').unwrap();

    for p in path {
        match &p[..] {
            "R" => pos.2 = (pos.2 + 1) % 4,
            "L" => pos.2 = if pos.2 == 0 { 3 } else { pos.2 - 1 },
            _ => {
                let n: u32 = p.parse().unwrap();
                pos = move_to(&tiles, &pos, n);
            }
        }
    }

    ((pos.0 as u32 + 1) * 1000) + ((pos.1 as u32 + 1) * 4) + pos.2
}

fn move_to2(tiles: &[Vec<char>], pos: &(usize, usize, u32), n: u32) -> (usize, usize, u32) {
    if n == 0 {
        return (pos.0, pos.1, pos.2);
    }

    match pos.2 {
        0 => {
            let next_col: usize = if pos.1 == tiles[pos.0].len() - 1 {
                0
            } else {
                pos.1 + 1
            };

            match tiles[pos.0][next_col] {
                '#' => (pos.0, pos.1, pos.2),
                '.' => move_to2(tiles, &(pos.0, next_col, pos.2), n - 1),
                _ => {
                    let col = tiles[pos.0].iter().position(|&x| x != ' ').unwrap();
                    if tiles[pos.0][col] == '#' {
                        (pos.0, pos.1, pos.2)
                    } else {
                        move_to2(tiles, &(pos.0, col, pos.2), n - 1)
                    }
                }
            }
        }
        1 => {
            let next_row: usize = if pos.0 == tiles.len() - 1 {
                0
            } else {
                pos.0 + 1
            };

            match tiles[next_row].get(pos.1) {
                Some('#') => (pos.0, pos.1, pos.2),
                Some('.') => move_to2(tiles, &(next_row, pos.1, pos.2), n - 1),
                _ => {
                    for i in 0.. {
                        if tiles[i][pos.1] == ' ' {
                            continue;
                        }

                        if tiles[i][pos.1] == '#' {
                            return (pos.0, pos.1, pos.2);
                        } else {
                            return move_to2(tiles, &(i, pos.1, pos.2), n - 1);
                        }
                    }
                    unreachable!();
                }
            }
        }
        2 => {
            let next_col: usize = if pos.1 == 0 {
                tiles[pos.0].len() - 1
            } else {
                pos.1 - 1
            };

            match tiles[pos.0][next_col] {
                '#' => (pos.0, pos.1, pos.2),
                '.' => move_to2(tiles, &(pos.0, next_col, pos.2), n - 1),
                _ => {
                    let col = tiles[pos.0].iter().rposition(|&x| x != ' ').unwrap();
                    if tiles[pos.0][col] == '#' {
                        (pos.0, pos.1, pos.2)
                    } else {
                        move_to2(tiles, &(pos.0, col, pos.2), n - 1)
                    }
                }
            }
        }
        3 => {
            let next_row: usize = if pos.0 == 0 {
                tiles.len() - 1
            } else {
                pos.0 - 1
            };

            match tiles[next_row].get(pos.1) {
                Some('#') => (pos.0, pos.1, pos.2),
                Some('.') => move_to2(tiles, &(next_row, pos.1, pos.2), n - 1),
                _ => {
                    for i in (0..tiles.len()).rev() {
                        if tiles[i].get(pos.1) == None || tiles[i][pos.1] == ' ' {
                            continue;
                        }

                        if tiles[i][pos.1] == '#' {
                            return (pos.0, pos.1, pos.2);
                        } else {
                            return move_to2(tiles, &(i, pos.1, pos.2), n - 1);
                        }
                    }
                    unreachable!();
                }
            }
        }
        _ => unreachable!(),
    }
}

pub fn solve_part2(inputs: &[String]) -> u32 {
    let mut tiles: Vec<Vec<char>> = Vec::new();

    for input in inputs.iter() {
        match input.chars().nth(0) {
            Some(' ') | Some('.') | Some('#') => tiles.push(input.chars().collect()),
            _ => break,
        };
    }
    let (mut path, mut s) = (Vec::<String>::new(), String::new());
    for c in inputs.last().unwrap().chars() {
        if c.is_numeric() {
            s.push(c);
        } else {
            if let Ok(n) = s.parse() {
                path.push(n);
            }
            s.clear();
            if let Ok(rl) = String::from(c).parse() {
                path.push(rl);
            }
        }
    }
    path.push(s);

    let mut pos: (usize, usize, u32) = (0, 0, 0);
    pos.1 = tiles[0].iter().position(|&x| x == '.').unwrap();

    for p in path {
        match &p[..] {
            "R" => pos.2 = (pos.2 + 1) % 4,
            "L" => pos.2 = if pos.2 == 0 { 3 } else { pos.2 - 1 },
            _ => {
                let n: u32 = p.parse().unwrap();
                pos = move_to2(&tiles, &pos, n);
            }
        }
    }

    ((pos.0 as u32 + 1) * 1000) + ((pos.1 as u32 + 1) * 4) + pos.2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 6032);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 159034);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 5031);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 62);
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
