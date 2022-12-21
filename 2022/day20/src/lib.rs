pub fn solve_part1(inputs: &[String]) -> i32 {
    let mut ret: Vec<i32> = Vec::new();
    let file: Vec<i32> = inputs.into_iter().map(|x| x.parse().unwrap()).collect();
    let mut positions: Vec<i32> = file.clone();

    for i in 0..3000 {
      let n: i32 = file[i % file.len()];
      if n != 0 {
        let i_n = positions.iter().position(|&x| x == n).unwrap();
        positions.remove(i_n);
        let d: i32 = i_n as i32 + n;
        let pos: usize = if d > 0 {
          (d % file.len() as i32) as usize + (d / file.len() as i32) as usize
        } else if d == 0 {
          file.len() - 1
        } else {
          file.len() - (d.abs() + 1) as usize
        };
        positions.insert(pos, n);
      }

      if i == 999 || i == 1999 || i == 2999 {
        let i_zero = positions.iter().position(|&x| x == 0).unwrap();
        ret.push(positions[(i_zero + 1) % positions.len()]);
        println!("{:?}", positions);
      }
    }

    println!("{:?}", ret);
    ret.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 3);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(&inputs);
        println!("{}", result);
        assert_eq!(result, 1624);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(&inputs[0..2]);
    //     assert_eq!(result, 62);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(&inputs[0..3]);
    //     assert_eq!(result, 62);
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
