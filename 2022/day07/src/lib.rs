use std::collections::HashMap;

pub fn solve_part1(inputs: Vec<String>) -> u32 {
ir).or_insert(0) += filesize;
    }
  }

  println!("{:?}", files);

  let mut ret: u32 = 0;
  for (k, v) in files {
    if v < 100000 {
      continue;
    }

    ret += v;
    for c in k.chars() {
      if c == '/' {
        ret += v;
      }
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
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 95437);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 1181094);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(inputs);
    //     println!("{}", result);
    //     assert_eq!(result, 19);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(inputs);
    //     println!("{}", result);
    //     assert_eq!(result, 3495);
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
