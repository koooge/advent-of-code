use std::collections::HashSet;

pub fn solve_part1(inputs: &[String], step: usize) -> usize {
  let mut map: Vec<Vec<char>> = vec![];
  let mut s: (usize, usize) = (0, 0);

  for (i, input) in inputs.iter().enumerate() {
    let row: Vec<char> = input.chars().collect();
    let j = row.iter().position(|&c| c == 'S');
    if j != None {
      s = (i, j.unwrap());
    }
    map.push(row);
  }

  let mut state: HashSet<(usize, usize)> = HashSet::from([s]);
  for _ in 1..=step {
    let mut next: HashSet<(usize, usize)> = HashSet::new();
    for s in state {
      // top
      if s.1 > 0 && map[s.1-1][s.0] != '#' {
        next.insert((s.0, s.1 - 1));
      }
      // right
      if s.0 < map[0].len() - 1 && map[s.1][s.0+1] != '#' {
        next.insert((s.0 + 1, s.1));
      }
      // bottom
      if s.1 < map.len() - 1 && map[s.1+1][s.0] != '#' {
        next.insert((s.0, s.1 + 1));
      }
      // left
      if s.0 > 0 && map[s.1][s.0-1] != '#' {
        next.insert((s.0 - 1, s.1));
      }
    }
    state = next;
  }

  state.len()
}

pub fn solve_part2(inputs: &[String], step: usize) -> usize {
  let mut map: Vec<Vec<char>> = vec![];
  let mut s: (isize, isize) = (isize::MIN, isize::MIN);

  for (i, input) in inputs.iter().enumerate() {
    let row: Vec<char> = input.chars().collect();
    let j = row.iter().position(|&c| c == 'S');
    if j != None {
      s = (i as isize, j.unwrap() as isize);
    }
    map.push(row);
  }

  let mut state: HashSet<(isize, isize)> = HashSet::from([s]);
  for _ in 1..=step {
    let mut next: HashSet<(isize, isize)> = HashSet::new();
    for s in state {
      let remainder: (isize, isize) = (s.0 % (map[0].len() as isize), s.1 % (map.len() as isize));
      let pos: (usize, usize) = (
        if remainder.0 >= 0 { remainder.0  as usize } else { ((map[0].len() as isize) + remainder.0) as usize },
        if remainder.1 >= 0 { remainder.1  as usize } else { ((map.len() as isize) + remainder.1) as usize },
      );
      // top
      if (pos.1 > 0 && map[pos.1-1][pos.0] != '#') || (pos.1 == 0 && map[map.len()-1][pos.0] != '#') {
        next.insert((s.0, s.1 - 1));
      }
      // right
      if (pos.0 < map[pos.1].len() - 1 && map[pos.1][pos.0+1] != '#') || (pos.0 == map[pos.1].len() - 1 && map[pos.1][0] != '#') {
        next.insert((s.0 + 1, s.1));
      }
      // bottom
      if (pos.1 < map.len() - 1 && map[pos.1+1][pos.0] != '#') || (pos.1 == map.len() - 1 && map[0][pos.0] != '#') {
        next.insert((s.0, s.1 + 1));
      }
      // left
      if (pos.0 > 0 && map[pos.1][pos.0-1] != '#') || (pos.0 == 0 && map[pos.1][map[pos.1].len()-1] != '#') {
        next.insert((s.0 - 1, s.1));
      }
    }
    state = next;
  }

  state.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs, 1);
      assert_eq!(result, 2);
    }

    #[test]
    fn part1_case2() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs, 2);
      assert_eq!(result, 4);
    }

    #[test]
    fn part1_case3() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs, 3);
      assert_eq!(result, 6);
    }

    #[test]
    fn part1_case4() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs, 6);
      assert_eq!(result, 16);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs, 64);
      assert_eq!(result, 3642);
    }

    #[test]
    fn part2_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs, 6);
      assert_eq!(result, 16);
    }

    #[test]
    fn part2_case2() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs, 10);
      assert_eq!(result, 50);
    }

    #[test]
    fn part2_case3() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs, 50);
      assert_eq!(result, 1594);
    }

    #[test]
    fn part2_case4() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs, 100);
      assert_eq!(result, 6536);
    }

    #[test]
    fn part2_case5() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs, 500);
      assert_eq!(result, 167004);
    }

    #[test]
    fn part2_case6() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs, 1000);
      assert_eq!(result, 668697);
    }

    #[test]
    fn part2_case7() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs, 5000);
      assert_eq!(result, 16733044);
    }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs, 26501365);
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
