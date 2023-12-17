fn is_valid(conditions: &[char], pattern: &[char]) -> bool {
  let mut ret = true;

  for (i, c) in pattern.iter().enumerate() {
    if c == &'.' && conditions[i] == '#' {
      ret = false;
      break;
    } else if c == &'#' && conditions[i] == '.' {
      ret = false;
      break
    }
  }

  ret
}

pub fn solve_part1(inputs: &[String]) -> usize {
  let mut ret: Vec<usize> = vec![0; inputs.len()];

  for (ind, input) in inputs.iter().enumerate() {
    let (cond, cont) = input.split_once(' ').unwrap();
    let conditions: Vec<char> = cond.chars().collect();
    let damaged: Vec<usize> = cont.split(',').filter_map(|c| c.parse::<usize>().ok()).collect();
    let mut damaged_indexes: Vec<usize> = vec![];
    let mut damaged_i = 0;
    for d in &damaged {
      damaged_indexes.push(damaged_i);
      damaged_i += d + 1;
    }
    let end: usize = conditions.len() - damaged.iter().sum::<usize>() - (damaged.len() - 1);

    while damaged_indexes[0] <= end {
      let mut pattern: Vec<char> = ".".repeat(conditions.len()).chars().collect();
      for i in 0..damaged.len() {
        for j in 0..damaged[i] {
          pattern[damaged_indexes[i] + j] = '#';
        }
      }

      // println!("{:?} {}", conditions, conditions.len());
      println!("{:?} {}", pattern, pattern.len());
      if is_valid(&conditions, &pattern) {
        ret[ind] += 1;
      }

      // increment
      for i in (0..damaged_indexes.len()).rev() {
        if (i == damaged_indexes.len() - 1 && damaged_indexes[i] + damaged[i] < conditions.len())
         || (i < damaged_indexes.len() - 1 && (damaged_indexes[i] + damaged[i] + 1 < damaged_indexes[i+1])) {
          damaged_indexes[i] += 1;
          break
        }
        if damaged_indexes[0] == end {
          damaged_indexes[0] += 1;
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
      assert_eq!(result, 21);
    }

    // #[test]
    // fn part1() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part1(&inputs);
    //   assert_eq!(result, 9648398);
    // }

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
