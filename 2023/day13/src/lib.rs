pub fn solve_part1(inputs: &[String]) -> usize {
  let mut ret: Vec<usize> = vec![];
  let mut patterns: Vec<Vec<Vec<char>>> = vec![];

  // parse
  let mut patterns_i = 0;
  for input in inputs {
    if input != "" {
      let line: Vec<char> = input.chars().collect();
      if patterns.get(patterns_i) == None {
        patterns.push(vec![]);
      }
      patterns[patterns_i].push(line);
    } else {
      patterns_i += 1;
    }
  }

  for pattern in patterns {
    for i in 0..pattern.len()-1 {
      let mut is_horizontal = true;
      if pattern[i].iter().collect::<String>() == pattern[i+1].iter().collect::<String>() {
        for j in (0..i).rev() {
          if pattern.get(i+1+(i-j)) == None {
            break;
          }
          if pattern[j].iter().collect::<String>() != pattern[i+1+(i-j)].iter().collect::<String>() {
            is_horizontal = false;
            break;
          }
        }
        if is_horizontal {
          ret.push((i+1) * 100);
        }
      }
    }

    for i in 0..pattern[0].len()-1 {
      let mut is_vertical = true;
      let mut a: Vec<char> = vec![];
      let mut b: Vec<char> = vec![];
      for j in 0..pattern.len() {
        a.push(pattern[j][i]);
        b.push(pattern[j][i+1]);
      }
      if a.iter().collect::<String>() == b.iter().collect::<String>() {
        for j in (0..i).rev() {
          if pattern[0].get(i+1+(i-j)) == None {
            break;
          }
          let mut a: Vec<char> = vec![];
          let mut b: Vec<char> = vec![];
          for k in 0..pattern.len() {
            a.push(pattern[k][j]);
            b.push(pattern[k][i+1+(i-j)]);
          }
          if a.iter().collect::<String>() != b.iter().collect::<String>() {
            is_vertical = false;
            break;
          }
        }
        if is_vertical {
          ret.push(i+1);
        }
      }
    }
  }

  ret.iter().sum()
}

// pub fn solve_part2(inputs: &[String], expand_scale: usize) -> usize {
//   0
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 405);
    }

    #[test]
    fn part1_case2() {
      let inputs = read_file("./src/test2.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 1300);
    }

    #[test]
    fn part1_case3() {
      let inputs = read_file("./src/test3.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 1);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 42974);
    }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 1030);
    // }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 0);
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
