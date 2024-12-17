use std::collections::HashSet;

type ScoresInfo = (
    Vec<Vec<char>>,
    Vec<Vec<Vec<usize>>>,
    (usize, usize, usize),
    (usize, usize),
);

fn get_scores(inputs: &[String]) -> ScoresInfo {
    let mut map: Vec<Vec<char>> = vec![];
    let mut scores: Vec<Vec<Vec<usize>>> = vec![];
    let mut start: (usize, usize, usize) = Default::default();
    let mut end: (usize, usize) = Default::default();
    for (i, input) in inputs.iter().enumerate() {
        let row: Vec<char> = input.chars().collect();
        map.push(row.clone());
        if let Some(j) = row.iter().position(|&c| c == 'S') {
            start = (i, j, 1);
        }
        if let Some(j) = row.iter().position(|&c| c == 'E') {
            end = (i, j);
        }
        scores.push(vec![vec![usize::MAX; 4]; row.len()]);
    }

    fn get_score(src: &[usize], dest: &[usize], dir: usize) -> (Vec<usize>, bool) {
        let mut updated = false;
        let mut dst = dest.to_vec();
        for (i, s) in src.iter().enumerate() {
            if *s == usize::MAX {
                continue;
            }
            let score = if i == dir { s + 1 } else { s + 1001 };
            for (j, d) in dst.iter_mut().enumerate() {
                let dst_score = if j == dir { score } else { score + 1000 };
                if dst_score <= *d {
                    *d = dst_score;
                    updated = true;
                }
            }
        }
        (dst, updated)
    }

    let mut positions: Vec<(usize, usize, usize)> = vec![start];
    scores[start.0][start.1][1] = 0;
    while !positions.is_empty() {
        let mut po = vec![];
        for (y, x, dir) in positions {
            // top
            if dir != 2 && y > 0 && map[y - 1][x] != '#' {
                let (s, updated) = get_score(&scores[y][x], &scores[y - 1][x], 0);
                scores[y - 1][x] = s;
                if updated {
                    po.push((y - 1, x, 0));
                }
            }
            // right
            if dir != 3 && x < map[y].len() - 1 && map[y][x + 1] != '#' {
                let (s, updated) = get_score(&scores[y][x], &scores[y][x + 1], 1);
                scores[y][x + 1] = s;
                if updated {
                    po.push((y, x + 1, 1));
                }
            }
            // bottom
            if dir != 0 && y < map.len() - 1 && map[y + 1][x] != '#' {
                let (s, updated) = get_score(&scores[y][x], &scores[y + 1][x], 2);
                scores[y + 1][x] = s;
                if updated {
                    po.push((y + 1, x, 2));
                }
            }
            // left
            if dir != 1 && x > 0 && map[y][x - 1] != '#' {
                let (s, updated) = get_score(&scores[y][x], &scores[y][x - 1], 3);
                scores[y][x - 1] = s;
                if updated {
                    po.push((y, x - 1, 3));
                }
            }
        }
        positions = po;
    }

    (map, scores, start, end)
}

pub fn solve_part1(inputs: &[String]) -> usize {
    let (_, scores, _, end) = get_scores(inputs);
    *scores[end.0][end.1].iter().min().unwrap()
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let (map, scores, start, e) = get_scores(inputs);
    let (end_dir, _) = scores[e.0][e.1]
        .iter()
        .enumerate()
        .min_by_key(|&(_, &v)| v)
        .unwrap();

    fn get_best_paths(score: usize, dest: &[usize], dst_dir: usize) -> Vec<usize> {
        dest.iter()
            .enumerate()
            .filter(|&(i, &s)| {
                let dst_score = if i == dst_dir {
                    score - 1
                } else {
                    score - 1001
                };
                s == dst_score
            })
            .map(|(index, _)| index)
            .collect()
    }

    let mut best_path: HashSet<(usize, usize)> = HashSet::new();
    best_path.insert((e.0, e.1));
    let mut positions: Vec<(usize, usize, usize)> = vec![(e.0, e.1, end_dir)];
    while !positions.is_empty() {
        let mut po: Vec<(usize, usize, usize)> = vec![];
        for (y, x, dir) in positions {
            let score = scores[y][x][dir];
            // top
            if y > 0 && map[y - 1][x] != '#' {
                let best_paths = get_best_paths(score, &scores[y - 1][x], 2);
                for p in best_paths {
                    best_path.insert((y - 1, x));
                    if y - 1 != start.0 || x != start.1 || x != start.2 {
                        po.push((y - 1, x, p));
                    }
                }
            }
            // right
            if x < map[y].len() - 1 && map[y][x + 1] != '#' {
                let best_paths = get_best_paths(score, &scores[y][x + 1], 3);
                for p in best_paths {
                    best_path.insert((y, x + 1));
                    if y != start.0 || x + 1 != start.1 || x != start.2 {
                        po.push((y, x + 1, p));
                    }
                }
            }
            // bottom
            if y < map.len() - 1 && map[y + 1][x] != '#' {
                let best_paths = get_best_paths(score, &scores[y + 1][x], 0);
                for p in best_paths {
                    best_path.insert((y + 1, x));
                    if y + 1 != start.0 || x != start.1 || x != start.2 {
                        po.push((y + 1, x, p));
                    }
                }
            }
            // left
            if x > 0 && map[y][x - 1] != '#' {
                let best_paths = get_best_paths(score, &scores[y][x - 1], 1);
                for p in best_paths {
                    best_path.insert((y, x - 1));
                    if y != start.0 || x - 1 != start.1 || x != start.2 {
                        po.push((y, x - 1, p));
                    }
                }
            }
        }
        positions = po;
    }
    best_path.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day16/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 7036);
    }

    #[test]
    fn part1_case2() {
        let inputs = read_file("./src/day16/test2.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 11048);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day16/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 107468);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day16/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 45);
    }

    #[test]
    fn part2_case2() {
        let inputs = read_file("./src/day16/test2.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 64);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day16/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 533);
    }
}
