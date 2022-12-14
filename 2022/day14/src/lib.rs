fn pour_sand(x: usize, y: usize, cave: &mut [Vec<bool>]) -> Result<(), ()> {
    if x == cave.len() - 1 || y == cave[0].len() - 1 {
        Err(())
    } else if cave[x][y + 1] == false {
        pour_sand(x, y + 1, cave)
    } else if cave[x - 1][y + 1] == false {
        pour_sand(x - 1, y + 1, cave)
    } else if cave[x + 1][y + 1] == false {
        pour_sand(x + 1, y + 1, cave)
    } else {
        cave[x][y] = true;
        Ok(())
    }
}

pub fn solve_part1(inputs: &[String]) -> u32 {
    let mut ret: u32 = 0;
    let offset: usize = 400;
    let mut cave: Vec<Vec<bool>> = vec![vec![false; 200]; 200];

    for input in inputs {
        let points: Vec<Vec<u32>> = input
            .split(" -> ")
            .map(|x| x.split(',').map(|x| x.parse().unwrap()).collect())
            .collect();
        for i in 0..points.len() - 1 {
            let dx: (u32, u32) = if points[i][0] > points[i + 1][0] {
                (points[i + 1][0], points[i][0])
            } else {
                (points[i][0], points[i + 1][0])
            };
            let dy: (u32, u32) = if points[i][1] > points[i + 1][1] {
                (points[i + 1][1], points[i][1])
            } else {
                (points[i][1], points[i + 1][1])
            };
            for x in dx.0..=dx.1 {
                for y in dy.0..=dy.1 {
                    cave[x as usize - offset][y as usize] = true;
                }
            }
        }
    }

    loop {
        match pour_sand(500 - offset, 0, &mut cave) {
            Ok(()) => ret += 1,
            Err(()) => break,
        }
    }

    ret
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
        assert_eq!(result, 24);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(&inputs);
        println!("{}", result);
        assert_eq!(result, 858);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(inputs);
    //     assert_eq!(result, 29);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(inputs);
    //     assert_eq!(result, 354);
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
