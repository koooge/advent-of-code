use std::collections::{HashMap, HashSet};

pub fn solve_part1(inputs: &[String]) -> u32 {
    let mut elves: HashSet<(i32, i32)> = HashSet::new();
    let (mut north, mut east, mut south, mut west) = (i32::MAX, i32::MIN, i32::MIN, i32::MAX);

    for (i, row) in inputs.iter().enumerate() {
        for j in 0..row.len() {
            if row.chars().nth(j) == Some('#') {
                elves.insert((i as i32, j as i32));
            }
        }
    }

    for i in 0..10 {
        let mut plan: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        for elf in elves.iter() {
            if !elves.contains(&(elf.0 - 1, elf.1 - 1))
                && !elves.contains(&(elf.0 - 1, elf.1))
                && !elves.contains(&(elf.0 - 1, elf.1 + 1))
                && !elves.contains(&(elf.0, elf.1 - 1))
                && !elves.contains(&(elf.0, elf.1 + 1))
                && !elves.contains(&(elf.0 + 1, elf.1 - 1))
                && !elves.contains(&(elf.0 + 1, elf.1))
                && !elves.contains(&(elf.0 + 1, elf.1 + 1))
            {
                plan.insert((elf.0, elf.1), vec![(elf.0, elf.1)]); // no need to move
            } else {
                for j in 0..4 {
                    // north
                    if (i + j) % 4 == 0
                        && !elves.contains(&(elf.0 - 1, elf.1 - 1))
                        && !elves.contains(&(elf.0 - 1, elf.1))
                        && !elves.contains(&(elf.0 - 1, elf.1 + 1))
                    {
                        plan.entry((elf.0 - 1, elf.1))
                            .or_insert(vec![])
                            .push((elf.0, elf.1));
                        break;
                    }

                    // south
                    if (i + j) % 4 == 1
                        && !elves.contains(&(elf.0 + 1, elf.1 - 1))
                        && !elves.contains(&(elf.0 + 1, elf.1))
                        && !elves.contains(&(elf.0 + 1, elf.1 + 1))
                    {
                        plan.entry((elf.0 + 1, elf.1))
                            .or_insert(vec![])
                            .push((elf.0, elf.1));
                        break;
                    }

                    // west
                    if (i + j) % 4 == 2
                        && !elves.contains(&(elf.0 - 1, elf.1 - 1))
                        && !elves.contains(&(elf.0, elf.1 - 1))
                        && !elves.contains(&(elf.0 + 1, elf.1 - 1))
                    {
                        plan.entry((elf.0, elf.1 - 1))
                            .or_insert(vec![])
                            .push((elf.0, elf.1));
                        break;
                    }

                    // east
                    if (i + j) % 4 == 3
                        && !elves.contains(&(elf.0 - 1, elf.1 + 1))
                        && !elves.contains(&(elf.0, elf.1 + 1))
                        && !elves.contains(&(elf.0 + 1, elf.1 + 1))
                    {
                        plan.entry((elf.0, elf.1 + 1))
                            .or_insert(vec![])
                            .push((elf.0, elf.1));
                        break;
                    }

                    // could not move
                    if j == 3 {
                        plan.insert((elf.0, elf.1), vec![(elf.0, elf.1)]);
                    }
                }
            }
        }
        elves.clear();
        for (k, v) in plan {
            if v.len() == 1 {
                elves.insert(k);
            } else {
                v.iter().for_each(|x| {
                    elves.insert((x.0, x.1));
                });
            }
        }
    }

    for elf in &elves {
        north = std::cmp::min(elf.0, north);
        south = std::cmp::max(elf.0, south);
        west = std::cmp::min(elf.1, west);
        east = std::cmp::max(elf.1, east);
    }

    (south.abs_diff(north) + 1) * (east.abs_diff(west) + 1) - elves.len() as u32
}

pub fn solve_part2(inputs: &[String]) -> u32 {
    let mut elves: HashSet<(i32, i32)> = HashSet::new();

    for (i, row) in inputs.iter().enumerate() {
        for j in 0..row.len() {
            if row.chars().nth(j) == Some('#') {
                elves.insert((i as i32, j as i32));
            }
        }
    }

    let mut i: u32 = 0;
    loop {
        let mut is_move = false;
        let mut plan: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        for elf in elves.iter() {
            if !elves.contains(&(elf.0 - 1, elf.1 - 1))
                && !elves.contains(&(elf.0 - 1, elf.1))
                && !elves.contains(&(elf.0 - 1, elf.1 + 1))
                && !elves.contains(&(elf.0, elf.1 - 1))
                && !elves.contains(&(elf.0, elf.1 + 1))
                && !elves.contains(&(elf.0 + 1, elf.1 - 1))
                && !elves.contains(&(elf.0 + 1, elf.1))
                && !elves.contains(&(elf.0 + 1, elf.1 + 1))
            {
                plan.insert((elf.0, elf.1), vec![(elf.0, elf.1)]); // no need to move
            } else {
                is_move = true;
                for j in 0..4 {
                    // north
                    if (i + j) % 4 == 0
                        && !elves.contains(&(elf.0 - 1, elf.1 - 1))
                        && !elves.contains(&(elf.0 - 1, elf.1))
                        && !elves.contains(&(elf.0 - 1, elf.1 + 1))
                    {
                        plan.entry((elf.0 - 1, elf.1))
                            .or_insert(vec![])
                            .push((elf.0, elf.1));
                        break;
                    }

                    // south
                    if (i + j) % 4 == 1
                        && !elves.contains(&(elf.0 + 1, elf.1 - 1))
                        && !elves.contains(&(elf.0 + 1, elf.1))
                        && !elves.contains(&(elf.0 + 1, elf.1 + 1))
                    {
                        plan.entry((elf.0 + 1, elf.1))
                            .or_insert(vec![])
                            .push((elf.0, elf.1));
                        break;
                    }

                    // west
                    if (i + j) % 4 == 2
                        && !elves.contains(&(elf.0 - 1, elf.1 - 1))
                        && !elves.contains(&(elf.0, elf.1 - 1))
                        && !elves.contains(&(elf.0 + 1, elf.1 - 1))
                    {
                        plan.entry((elf.0, elf.1 - 1))
                            .or_insert(vec![])
                            .push((elf.0, elf.1));
                        break;
                    }

                    // east
                    if (i + j) % 4 == 3
                        && !elves.contains(&(elf.0 - 1, elf.1 + 1))
                        && !elves.contains(&(elf.0, elf.1 + 1))
                        && !elves.contains(&(elf.0 + 1, elf.1 + 1))
                    {
                        plan.entry((elf.0, elf.1 + 1))
                            .or_insert(vec![])
                            .push((elf.0, elf.1));
                        break;
                    }

                    // could not move
                    if j == 3 {
                        plan.insert((elf.0, elf.1), vec![(elf.0, elf.1)]);
                    }
                }
            }
        }

        i += 1;
        if !is_move {
            return i;
        }

        elves.clear();
        for (k, v) in plan {
            if v.len() == 1 {
                elves.insert(k);
            } else {
                v.iter().for_each(|x| {
                    elves.insert((x.0, x.1));
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 110);
    }

    #[test]
    fn part1_case2() {
        let inputs = read_file("./src/test2.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 25);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 3996);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 20);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 908);
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
