pub fn solve_part1(inputs: &[String]) -> usize {
  let mut cosmic: Vec<Vec<char>> = inputs.to_vec().iter()
    .map(|x| x.chars().collect::<Vec<char>>()).collect();

  // cosmic expansion
  for i in (0..cosmic.len()).rev() {
    if !cosmic[i].contains(&'#') {
      let space = cosmic[i].clone();
      cosmic.insert(i + 1, space);
    }
  }
  for i in (0..cosmic[0].len()).rev() {
    let mut nogalaxy = true;
    for j in 0..cosmic.len() {
      if cosmic[j][i] == '#' {
        nogalaxy = false;
        break
      }
    }
    if nogalaxy {
      for j in 0..cosmic.len() {
        cosmic[j].insert(i + 1, '.');
      }
    }
  }

  // find galaxy
  let mut galaxies: Vec<(usize, usize)> = vec![];
  for (i, row) in cosmic.iter().enumerate() {
    for (j, c) in row.iter().enumerate() {
      if c == &'#' {
        galaxies.push((i, j));
      }
    }
  }

  // combinations
  let mut ret = 0;
  for (i, a) in galaxies.iter().enumerate() {
    for j in i..galaxies.len() {
      let b = galaxies.get(j).unwrap();
      let x = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
      let y = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
      ret += x + y;
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
      assert_eq!(result, 374);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 9648398);
    }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 2);
    // }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 977);
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
