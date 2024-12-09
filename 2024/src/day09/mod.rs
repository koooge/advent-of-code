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

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day09/test2.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 9);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day09/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 1190);
    // }
}
