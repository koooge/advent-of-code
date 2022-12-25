use std::collections::HashMap;

fn move_blizzards(blizzards: &mut HashMap<(u32, u32), Vec<char>>, row_len: u32, col_len: u32) {
    for (xy, bl) in blizzards {
        for (i, c) in bl.into_iter().enumerate() {
            let next_xy = match c {
                '^' => if xy.0 > 1 { (xy.0 - 1, xy.1) } else { (row_len - 2, xy.1) },
                '>' => if xy.1 < col_len - 2 { (xy.0, xy.1 + 1) } else { (xy.0, 1) },
                'v' => if xy.0 < row_len - 2  { (xy.0 + 1, xy.1) } else { (1, xy.1) },
                '<' => if xy.1 > 1 { (xy.0, xy.1 - 1) } else { (xy.0, col_len - 2)},
                _ => unreachable!(),
            };
            bl.remove(i);
                                // // &blizzards.get_mut(xy).unwrap().remove(i);
                                // blizzards
                                // .entry((xy.0, xy.1))
                                // .or_insert(vec![])
                                // .push(*c);
        }
    }
}

pub fn solve_part1(inputs: &[String]) -> u32 {
    let mut steps = 0;
    let pos = (0, 1);
    let goal = (inputs.len() - 1, inputs[inputs.len() - 1].len() - 2);
    let mut blizzards: HashMap<(u32, u32), Vec<char>> = HashMap::new();

    for i in 1..inputs.len() - 1 {
        for j in 1..inputs[0].len() - 1 {
            let c = &inputs[i].chars().nth(j).unwrap();
            if ['^', '>', 'v', '<'].contains(c) {
                blizzards
                    .entry((i as u32, j as u32))
                    .or_insert(vec![])
                    .push(*c);
            }
        }
    }

    while pos != goal {
        move_blizzards(&mut blizzards, inputs.len() as u32, inputs[0].len() as u32);
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs);
        // assert_eq!(result, 18);
    }

    #[test]
    fn part1_case12() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 18);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 3996);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 20);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 908);
    // }

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
