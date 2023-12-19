pub fn solve_part1(inputs: &[String]) -> usize {
  let mut map: Vec<Vec<char>> = vec![];
  let mut energized: Vec<Vec<(bool, bool, bool, bool)>> = vec![vec![(false, false, false, false); inputs[0].len()]; inputs.len()];

  for input in inputs {
    let row = input.chars().collect::<Vec<char>>();
    map.push(row);
  }
  energized[0][0] = match map[0][0] {
    '.' => (false, true, false, false),
    '/' => (true, false, false, false),
    '\\' => (false, false, true, false),
    '|' => (true, true, true, false),
    '-' => (false, true, false, false),
    _ => unreachable!(),
  };

  loop {
    let mut is_new = false;

    for i in 0..energized.len() {
      for j in 0..energized[i].len() {
        // upward
        if energized[i][j].0 {
          for k in (0..i).rev() {
            match map[k][j] {
              '/' => {
                if !energized[k][j].1 {
                  is_new = true;
                }
                energized[k][j] = (energized[k][j].0, true, energized[k][j].2, energized[k][j].3);
                break;
              },
              '\\' => {
                if !energized[k][j].3 {
                  is_new = true;
                }
                energized[k][j] = (energized[k][j].0, energized[k][j].1, energized[k][j].2, true);
                break;
              },
              '-' => {
                if !energized[k][j].1 || !energized[k][j].3 {
                  is_new = true;
                }
                energized[k][j] = (energized[k][j].0, true, energized[k][j].2, true);
                break;
              },
              '.' | '|' => {
                if energized[k][j].0 {
                  break;
                }
                energized[k][j] = (true, energized[k][j].1, energized[k][j].2, energized[k][j].3);
                is_new = true;
              },
              _ => unreachable!(),
            }
          }
        }
        // rightward
        if energized[i][j].1 {
          for k in j+1..energized[i].len() {
            match map[i][k] {
              '/' => {
                if !energized[i][k].0 {
                  is_new = true;
                }
                energized[i][k] = (true, energized[i][k].1, energized[i][k].2, energized[i][k].3);
                break;
              },
              '\\' => {
                if !energized[i][k].2 {
                  is_new = true;
                }
                energized[i][k] = (energized[i][k].0, energized[i][k].1, true, energized[i][k].3);
                break;
              },
              '|' => {
                if !energized[i][k].0 || !energized[i][k].2 {
                  is_new = true;
                }
                energized[i][k] = (true, energized[i][k].1, true, energized[i][k].3);
                break;
              },
              '.' | '-' => {
                if energized[i][k].1 {
                  break;
                }
                energized[i][k] = (energized[i][k].0, true, energized[i][k].2, energized[i][k].3);
                is_new = true;
              },
              _ => unreachable!(),
            }
          }
        }
        // downward
        if energized[i][j].2 {
          for k in i+1..energized.len() {
            match map[k][j] {
              '/' => {
                if !energized[k][j].3 {
                  is_new = true;
                }
                energized[k][j] = (energized[k][j].0, energized[k][j].1, energized[k][j].2, true);
                break;
              },
              '\\' => {
                if !energized[k][j].1 {
                  is_new = true;
                }
                energized[k][j] = (energized[k][j].0, true, energized[k][j].2, energized[k][j].3);
                break;
              },
              '-' => {
                if !energized[k][j].1 || !energized[k][j].3 {
                  is_new = true;
                }
                energized[k][j] = (energized[k][j].0, true, energized[k][j].2, true);
                break;
              },
              '.' | '|' => {
                if energized[k][j].2 {
                  break;
                }
                energized[k][j] = (energized[k][j].0, energized[k][j].1, true, energized[k][j].3);
                is_new = true;
              },
              _ => unreachable!(),
            }
          }
        }
        // leftward
        if energized[i][j].3 {
          for k in (0..j).rev() {
            match map[i][k] {
              '/' => {
                if !energized[i][k].2 {
                  is_new = true;
                }
                energized[i][k] = (energized[i][k].0, energized[i][k].1, true, energized[i][k].3);
                break;
              },
              '\\' => {
                if !energized[i][k].0 {
                  is_new = true;
                }
                energized[i][k] = (true, energized[i][k].1, energized[i][k].2, energized[i][k].3);
                break;
              },
              '|' => {
                if !energized[i][k].0 || !energized[i][k].2 {
                  is_new = true;
                }
                energized[i][k] = (true, energized[i][k].1, true, energized[i][k].3);
                break;
              },
              '.' | '-' => {
                if energized[i][k].3 {
                  break;
                }
                energized[i][k] = (energized[i][k].0, energized[i][k].1, energized[i][k].2, true);
                is_new = true;
              },
              _ => unreachable!(),
            }
          }
        }
      }
    }

    if !is_new {
      break;
    }
  }

  let mut ret = 0;
  for row in energized {
    for tile in row {
      if tile.0 || tile.1 || tile.2 || tile.3 {
        ret += 1;
      }
    }
  }
  ret
}

// pub fn solve_part2(inputs: &[String]) -> usize {
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
      assert_eq!(result, 46);
    }

    #[test]
    fn part1_case2() {
      let inputs = read_file("./src/test2.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 21);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 8551);
    }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 145);
    // }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 261505);
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
