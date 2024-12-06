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

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day06/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 123);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day06/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 7380);
    // }
}
