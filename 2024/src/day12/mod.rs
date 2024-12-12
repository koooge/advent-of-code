pub fn solve_part1(inputs: &[String]) -> usize {
    let mut map: Vec<Vec<char>> = vec![];
    let mut checked: Vec<Vec<bool>> = vec![];
    for input in inputs {
        map.push(input.chars().collect());
        let c: Vec<bool> = vec![false; input.len()];
        checked.push(c);
    }

    fn get_region(
        map: &[Vec<char>],
        checked: &[Vec<bool>],
        region: &[(usize, usize)],
        pos: (usize, usize),
    ) -> (Vec<(usize, usize)>, Vec<Vec<bool>>) {
        let mut r = region.to_vec();
        r.push((pos.0, pos.1));
        let mut ch = checked.to_vec();
        ch[pos.0][pos.1] = true;

        let c = map[pos.0][pos.1];
        // top
        if pos.0 > 0 && !ch[pos.0 - 1][pos.1] && c == map[pos.0 - 1][pos.1] {
            (r, ch) = get_region(map, &ch, &r, (pos.0 - 1, pos.1));
        }
        // right
        if pos.1 < map[pos.0].len() - 1 && !ch[pos.0][pos.1 + 1] && c == map[pos.0][pos.1 + 1] {
            (r, ch) = get_region(map, &ch, &r, (pos.0, pos.1 + 1));
        }
        // bottom
        if pos.0 < map.len() - 1 && !ch[pos.0 + 1][pos.1] && c == map[pos.0 + 1][pos.1] {
            (r, ch) = get_region(map, &ch, &r, (pos.0 + 1, pos.1));
        }
        // left
        if pos.1 > 0 && !ch[pos.0][pos.1 - 1] && c == map[pos.0][pos.1 - 1] {
            (r, ch) = get_region(map, &ch, &r, (pos.0, pos.1 - 1));
        }

        (r, ch)
    }

    fn get_perimeter(region: &[(usize, usize)]) -> usize {
        let mut ret: usize = 0;

        for (y, x) in region {
            let mut adj: usize = 0;
            for pos in region {
                if (*y > 0 && *pos == (y - 1, *x))
                    || *pos == (*y, x + 1)
                    || *pos == (y + 1, *x)
                    || (*x > 0 && *pos == (*y, x - 1))
                {
                    adj += 1;
                }
            }

            ret += 4 - adj;
        }

        ret
    }

    let mut prices: Vec<usize> = vec![];
    for i in 0..checked.len() {
        for j in 0..checked[i].len() {
            if checked[i][j] {
                continue;
            }
            let mut region = vec![];
            (region, checked) = get_region(&map, &checked, &region, (i, j));
            let perimeter = get_perimeter(&region);
            prices.push(region.len() * perimeter);
        }
    }

    prices.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day12/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 140);
    }

    #[test]
    fn part1_case2() {
        let inputs = read_file("./src/day12/test2.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 772);
    }

    #[test]
    fn part1_case3() {
        let inputs = read_file("./src/day12/test3.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 1930);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day12/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 1477762);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day12/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 65601038650482);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day12/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 272673043446478);
    // }
}
