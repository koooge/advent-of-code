fn is_symbol(val: Option<char>) -> bool {
  val != None && !val.unwrap().is_numeric() && val.unwrap() != '.'
}

pub fn solve_part1(inputs: &[String]) -> usize {
  let mut ret = 0;

  for (i, input) in inputs.iter().enumerate() {
    let mut adj_symbol = false;
    let mut numbers = String::from("");
    for (j, c) in input.chars().enumerate() {
      if !adj_symbol {
        if i > 0 {
          let t = &inputs[i-1];
          if (j > 0 && is_symbol(t.chars().nth(j-1)))
          || is_symbol(t.chars().nth(j))
          || (j < input.len() - 1 && is_symbol(t.chars().nth(j+1))) {
            adj_symbol = true;
          }
        }

        if j < input.len() - 1 && is_symbol(input.chars().nth(j+1)) {
          adj_symbol = true;
        }

        if i < inputs.len() -1 {
          let b = &inputs[i+1];
          if (j > 0 && is_symbol(b.chars().nth(j-1)))
          || is_symbol(b.chars().nth(j))
          || (j < input.len() - 1 && is_symbol(b.chars().nth(j+1))) {
            adj_symbol = true;
          }
        }

        if j > 0 && is_symbol(input.chars().nth(j-1)) {
          adj_symbol = true;
        }
      }

      if c.is_numeric() {
        numbers = format!("{}{}", numbers, c.to_string());
        let right = input.chars().nth(j+1);
        if right == None || !right.unwrap().is_numeric() {
          if adj_symbol {
            let num = numbers.parse::<usize>().unwrap();
            ret += num;
          }
          numbers = String::from("");
          adj_symbol = false;
        }
      } else {
        adj_symbol = false;
      }
    }
  }

  ret
}

// pub fn solve_part2(inputs: &[String]) -> usize {
//   let mut ret = 0;

//   ret
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 4361);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 550934);
    }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 2286);
    // }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 55593);
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
