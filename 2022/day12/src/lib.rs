fn map_steps(
    heights: &[Vec<char>],
    steps: &mut [Vec<u32>],
    y_len: usize,
    x_len: usize,
    y: usize,
    x: usize,
) {
    let yxu8: u8 = heights[y][x] as u8;
    if y > 0
        && steps[y - 1][x] > steps[y][x] + 1
        && ((heights[y - 1][x] > 'a' && heights[y - 1][x] as u8 <= yxu8 + 1)
            || heights[y][x] == 'S' && heights[y - 1][x] == 'a'
            || heights[y][x] == 'z' && heights[y - 1][x] == 'E')
    {
        steps[y - 1][x] = steps[y][x] + 1;
        map_steps(heights, steps, y_len, x_len, y - 1, x);
    }

    if x + 1 < x_len
        && steps[y][x + 1] > steps[y][x] + 1
        && ((heights[y][x + 1] > 'a' && heights[y][x + 1] as u8 <= yxu8 + 1)
            || heights[y][x] == 'S' && heights[y][x + 1] == 'a'
            || heights[y][x] == 'z' && heights[y][x + 1] == 'E')
    {
        steps[y][x + 1] = steps[y][x] + 1;
        map_steps(heights, steps, y_len, x_len, y, x + 1);
    }

    if y + 1 < y_len
        && steps[y + 1][x] > steps[y][x] + 1
        && ((heights[y + 1][x] > 'a' && heights[y + 1][x] as u8 <= yxu8 + 1)
            || heights[y][x] == 'S' && heights[y + 1][x] == 'a'
            || heights[y][x] == 'z' && heights[y + 1][x] == 'E')
    {
        steps[y + 1][x] = steps[y][x] + 1;
        map_steps(heights, steps, y_len, x_len, y + 1, x);
    }

    if x > 0
        && steps[y][x - 1] > steps[y][x] + 1
        && ((heights[y][x - 1] > 'a' && heights[y][x - 1] as u8 <= yxu8 + 1)
            || heights[y][x] == 'S' && heights[y][x - 1] == 'a'
            || heights[y][x] == 'z' && heights[y][x - 1] == 'E')
    {
        steps[y][x - 1] = steps[y][x] + 1;
        map_steps(heights, steps, y_len, x_len, y, x - 1);
    }
}

pub fn solve_part1(inputs: Vec<String>) -> u32 {
    let mut heights: Vec<Vec<char>> = Vec::new();
    let mut pos_s: (usize, usize) = (0, 0);
    let mut pos_e: (usize, usize) = (0, 0);

    for (i, input) in inputs.iter().enumerate() {
        let line: Vec<char> = input.chars().collect();
        if let Some(j) = line.iter().position(|&x| x == 'S') {
            pos_s = (i, j);
        }
        if let Some(j) = line.iter().position(|&x| x == 'E') {
            pos_e = (i, j);
        }
        heights.push(line);
    }
    let map_size: (usize, usize) = (heights.len(), heights[0].len());
    let mut steps: Vec<Vec<u32>> = vec![vec![u32::MAX; map_size.1]; map_size.0];
    steps[pos_s.0][pos_s.1] = 0;

    map_steps(
        &heights, &mut steps, map_size.0, map_size.1, pos_s.0, pos_s.1,
    );

    steps[pos_e.0][pos_e.1] as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 31);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 361);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(inputs);
    //     print_crt(result);
    //     assert_eq!(
    //         result,
    //         [
    //             true, true, false, false, true, true, false, false, true, true, false, false, true,
    //             true, false, false, true, true, false, false, true, true, false, false, true, true,
    //             false, false, true, true, false, false, true, true, false, false, true, true,
    //             false, false, true, true, true, false, false, false, true, true, true, false,
    //             false, false, true, true, true, false, false, false, true, true, true, false,
    //             false, false, true, true, true, false, false, false, true, true, true, false,
    //             false, false, true, true, true, false, true, true, true, true, false, false, false,
    //             false, true, true, true, true, false, false, false, false, true, true, true, true,
    //             false, false, false, false, true, true, true, true, false, false, false, false,
    //             true, true, true, true, false, false, false, false, true, true, true, true, true,
    //             false, false, false, false, false, true, true, true, true, true, false, false,
    //             false, false, false, true, true, true, true, true, false, false, false, false,
    //             false, true, true, true, true, true, false, false, false, false, false, true, true,
    //             true, true, true, true, false, false, false, false, false, false, true, true, true,
    //             true, true, true, false, false, false, false, false, false, true, true, true, true,
    //             true, true, false, false, false, false, false, false, true, true, true, true, true,
    //             true, true, true, true, true, true, false, false, false, false, false, false,
    //             false, true, true, true, true, true, true, true, false, false, false, false, false,
    //             false, false, true, true, true, true, true, true, true, false, false, false, false,
    //             false
    //         ]
    //     );
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(inputs);
    //     print_crt(result);
    //     assert_eq!(
    //         result,
    //         [
    //             true, true, true, false, false, true, true, true, true, false, true, false, false,
    //             true, false, true, true, true, true, false, false, true, true, false, false, true,
    //             true, true, false, false, true, true, true, true, false, true, true, true, true,
    //             false, true, false, false, true, false, true, false, false, false, false, true,
    //             false, true, false, false, false, false, false, true, false, true, false, false,
    //             true, false, true, false, false, true, false, true, false, false, false, false,
    //             true, false, false, false, false, true, false, false, true, false, true, true,
    //             true, false, false, true, true, false, false, false, false, false, true, false,
    //             false, true, false, false, false, false, true, false, false, true, false, true,
    //             true, true, false, false, true, true, true, false, false, true, true, true, false,
    //             false, true, false, false, false, false, true, false, true, false, false, false,
    //             true, false, false, false, true, false, false, false, false, true, true, true,
    //             false, false, true, false, false, false, false, true, false, false, false, false,
    //             true, false, true, false, false, true, false, false, false, false, true, false,
    //             true, false, false, true, false, false, false, false, true, false, false, true,
    //             false, true, false, false, false, false, true, false, false, false, false, true,
    //             false, false, false, false, true, false, false, true, false, true, false, false,
    //             false, false, true, false, false, true, false, true, true, true, true, false,
    //             false, true, true, false, false, true, false, false, false, false, true, true,
    //             true, true, false, true, false, false, false, false
    //         ]
    //     );
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
