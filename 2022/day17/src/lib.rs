use std::collections::HashMap;

fn solve(input: String, target: u64) -> u64 {
    let jet_patterns: Vec<char> = input.chars().collect();
    let mut rocks = [
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ]
    .into_iter()
    .cycle();
    let mut space = vec![[false; 7]; 0];
    let mut hm = HashMap::new();

    let mut j = 0;
    for i in 0..target {
        let mut rock = rocks
            .next()
            .unwrap()
            .iter()
            .map(|(i, j)| (i + space.len() + 3, j + 2))
            .collect::<Vec<_>>();
        space.extend(vec![[false; 7]; 7]);
        loop {
            match jet_patterns[j % jet_patterns.len()] {
                '<' => {
                    if rock.iter().all(|&(i, j)| j > 0 && !space[i][j - 1]) {
                        rock.iter_mut().for_each(|(_, j)| *j -= 1);
                    }
                }
                '>' => {
                    if rock.iter().all(|&(i, j)| j < 6 && !space[i][j + 1]) {
                        rock.iter_mut().for_each(|(_, j)| *j += 1);
                    }
                }
                _ => unreachable!("Must be > or <"),
            }
            j += 1;
            if rock.iter().any(|&(i, j)| i == 0 || space[i - 1][j]) {
                break;
            }
            rock.iter_mut().for_each(|(i, _)| *i -= 1);
        }
        rock.iter().for_each(|&(i, j)| space[i][j] = true);
        while let Some(last) = space.last() {
            if last.iter().all(|&x| !x) {
                space.pop();
            } else {
                break;
            }
        }

        let key = (i % 5, j % jet_patterns.len(), *space.last().unwrap());
        hm.entry(key)
            .or_insert_with(Vec::new)
            .push((i, space.len() as u64));
        if hm[&key].len() > 2 {
            let mut v = hm[&key]
                .windows(2)
                .map(|w| (w[1].0 - w[0].0, w[1].1 - w[0].1))
                .collect::<Vec<_>>();
            v.dedup();

            if v.len() == 1 {
                let (cycle, offset) = v[0];
                let remaining = target - i;
                if let Some((_, h)) = hm
                    .values()
                    .filter_map(|v| v.last())
                    .find(|(prev, _)| *prev == i - cycle + remaining % cycle - 1)
                {
                    return (remaining / cycle + 1) * offset + h;
                }
            }
        }
    }
    space.len() as u64
}

pub fn solve_part1(inputs: &[String]) -> u64 {
    solve(inputs[0].clone(), 2022)
}

pub fn solve_part2(inputs: &[String]) -> u64 {
    solve(inputs[0].clone(), 1000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs);
        println!("{}", result);
        assert_eq!(result, 3068);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(&inputs);
        println!("{}", result);
        assert_eq!(result, 3098);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 1514285714288);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 1525364431487);
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
