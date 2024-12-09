pub fn solve_part1(inputs: &[String]) -> usize {
    let diskmap: Vec<char> = inputs[0].chars().collect();
    let mut disk: Vec<Option<usize>> = vec![];

    fn compact(disk: &[Option<usize>]) -> Vec<Option<usize>> {
        let mut ret = vec![];
        let (mut cur_top, mut cur_last) = (0, disk.len() - 1);

        while cur_top <= cur_last {
            if disk[cur_top].is_some() {
                ret.push(disk[cur_top]);
            } else {
                while disk[cur_last].is_none() {
                    cur_last -= 1;
                }
                ret.push(disk[cur_last]);
                cur_last -= 1;
            }
            cur_top += 1;
        }
        ret
    }

    let mut id: usize = 0;
    let mut is_block = true;
    for c in diskmap {
        let n: u8 = c.to_digit(10).unwrap() as u8;
        if is_block {
            for _ in 0..n {
                disk.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..n {
                disk.push(None);
            }
        }
        is_block = !is_block;
    }

    disk = compact(&disk);

    let mut ret = 0;
    for (i, id) in disk.iter().enumerate() {
        if let Some(id) = id {
            ret += id * i;
        }
    }

    ret
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let diskmap: Vec<char> = inputs[0].chars().collect();
    let mut disk: Vec<(Option<usize>, usize)> = vec![];

    fn compact_n(disk: &[(Option<usize>, usize)], id: usize) -> Vec<(Option<usize>, usize)> {
        let mut compacted = disk.to_vec();
        if let Some(i) = compacted.iter().position(|&x| x.0 == Some(id)) {
            for j in 0..i {
                if compacted[j].0.is_some() {
                    continue;
                }
                match compacted[i].1.cmp(&compacted[j].1) {
                    std::cmp::Ordering::Equal => compacted.swap(i, j),
                    std::cmp::Ordering::Less => {
                        let d = compacted[j].1 - compacted[i].1;
                        compacted[j] = compacted[i];
                        compacted[i].0 = None;
                        compacted.insert(j + 1, (None, d));
                    }
                    _ => (),
                }
            }
        }
        compacted
    }

    fn compact(disk: &[(Option<usize>, usize)], max_id: usize) -> Vec<Option<usize>> {
        let mut compacted = disk.to_vec();
        for i in (0..=max_id).rev() {
            compacted = compact_n(&compacted, i);
        }

        let mut ret = vec![];
        for d in compacted {
            for _ in 0..d.1 {
                if d.0.is_some() {
                    ret.push(d.0);
                } else {
                    ret.push(None);
                }
            }
        }
        ret
    }

    let mut id: usize = 0;
    let mut is_block = true;
    for c in diskmap {
        let n = c.to_digit(10).unwrap() as usize;
        if is_block {
            disk.push((Some(id), n));
            id += 1;
        } else {
            disk.push((None, n));
        }
        is_block = !is_block;
    }

    let compacted = compact(&disk, id - 1);

    let mut ret = 0;
    for (i, id) in compacted.iter().enumerate() {
        if let Some(id) = id {
            ret += id * i;
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
        let inputs = read_file("./src/day09/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 1928);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day09/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 6463499258318);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day09/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 2858);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day09/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 9675443525851); // too high
    }
}
