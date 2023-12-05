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

pub fn solve_part2(inputs: &[String]) -> usize {
  let mut ret = 0;

  for (i, input) in inputs.iter().enumerate() {
    for (j, c) in input.chars().enumerate() {
      if c != '*' {
        continue
      }

      let mut parts: Vec<usize> = vec![];
      // let mut numbers = String::from("");

      // top
      if i > 0 {
        let t = &inputs[i-1];
        if t.chars().nth(j).unwrap().is_numeric() {
          if j < t.len() - 2 && t.chars().nth(j+1).unwrap().is_numeric() && t.chars().nth(j+2).unwrap().is_numeric(){
            let n = t.get(..=j+2).unwrap().split('.').last().unwrap().parse::<usize>().unwrap();
            parts.push(n);
          } else if j < t.len() - 1 && t.chars().nth(j+1).unwrap().is_numeric() {
            let n = t.get(..=j+1).unwrap().split('.').last().unwrap().parse::<usize>().unwrap();
            parts.push(n);
          } else if t.chars().nth(j).unwrap().is_numeric() {
            let n = t.get(..=j).unwrap().split('.').last().unwrap().parse::<usize>().unwrap();
            parts.push(n);
          }
        } else {
          if j > 0 && t.chars().nth(j-1).unwrap().is_numeric() {
            let n = t.get(..j).unwrap().split('.').last().unwrap().parse::<usize>().unwrap();
            parts.push(n);
          }
          if j < t.len() - 1 && t.chars().nth(j+1).unwrap().is_numeric() {
            let n = t.get(j+1..).unwrap().split('.').next().unwrap().parse::<usize>().unwrap();
            parts.push(n);
          }
        }
      }

      // right
      if j < input.len() - 1 && input.chars().nth(j+1).unwrap().is_numeric() {
        let n = input.get(j+1..).unwrap().split('.').next().unwrap().parse::<usize>().unwrap();
        parts.push(n);
      }

      // bottom
      if i < inputs.len() -1 {
        let b = &inputs[i+1];
        if b.chars().nth(j).unwrap().is_numeric() {
          if j < b.len() - 2 && b.chars().nth(j+1).unwrap().is_numeric() && b.chars().nth(j+2).unwrap().is_numeric(){
            let n = b.get(..=j+2).unwrap().split('.').last().unwrap().parse::<usize>().unwrap();
            parts.push(n);
          } else if j < b.len() - 1 && b.chars().nth(j+1).unwrap().is_numeric() {
            let n = b.get(..=j+1).unwrap().split('.').last().unwrap().parse::<usize>().unwrap();
            parts.push(n);
          } else if b.chars().nth(j).unwrap().is_numeric() {
            let n = b.get(..=j).unwrap().split('.').last().unwrap().parse::<usize>().unwrap();
            parts.push(n);
          }
        } else {
          if j > 0 && b.chars().nth(j-1).unwrap().is_numeric() {
            let n = b.get(..j).unwrap().split('.').last().unwrap().parse::<usize>().unwrap();
            parts.push(n);
          }
          if j < b.len() - 1 && b.chars().nth(j+1).unwrap().is_numeric() {
            let n = b.get(j+1..).unwrap().split('.').next().unwrap().parse::<usize>().unwrap();
            parts.push(n);
          }
        }
      }

      // left
      if j > 0 && input.chars().nth(j-1).unwrap().is_numeric() {
        let n = input.get(..j).unwrap().split('.').last().unwrap().parse::<usize>().unwrap();
        parts.push(n);
      }

      if parts.len() == 2 {
        ret += parts[0] * parts[1];
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
      let result = solve_part1(&inputs);
      assert_eq!(result, 4361);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 550934);
    }

    #[test]
    fn part2_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 467835);
    }

    #[test]
    fn part2() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 81997870);
    }

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
