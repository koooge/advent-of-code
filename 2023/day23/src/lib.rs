#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos(usize, usize);

#[derive(Clone, Copy, Debug)]
struct State {
  pos: Pos,
  steps: usize,
  from: u8,
}

#[derive(Clone, Debug)]
struct StateWithPath {
  pos: Pos,
  steps: usize,
  path: Vec<Pos>,
}

pub fn solve_part1(inputs: &[String]) -> usize {
  let mut map: Vec<Vec<char>> = vec![];
  for input in inputs {
    let mut row: Vec<char> = vec![];
    for c in input.chars() {
      row.push(c);
    }
    map.push(row);
  }

  let mut longests: Vec<Vec<isize>> = vec![vec![isize::MIN; map[0].len()]; map.len()];
  longests[0][1] = 0;
  let init: State = State {
    pos: Pos(1, 0),
    steps: 0,
    from: 0,
  };
  let mut stack: Vec<State> = vec![init];
  let (goal_x, goal_y) = (map[map.len()-1].len() - 2, map.len() - 1);
  while stack.len() > 0 {
    let cur = stack.pop().unwrap();

    // upward
    if cur.pos.1 > 0 && cur.from != 0 {
      let (x, y) = (cur.pos.0, cur.pos.1 - 1);
      if (map[y][x] == '.' || map[y][x] == '^') && (cur.steps as isize) + 1 > longests[y][x] {
        let next = State {
          pos: Pos(x, y),
          steps: cur.steps + 1,
          from: 2,
        };
        stack.push(next);
        longests[y][x] = next.steps as isize;
      }
    }
    // rightward
    if cur.pos.0 < map[cur.pos.1].len() - 1 && cur.from != 1 {
      let (x, y) = (cur.pos.0 + 1, cur.pos.1);
      if (map[y][x] == '.' || map[y][x] == '>') && (cur.steps as isize) + 1 > longests[y][x] {
        let next = State {
          pos: Pos(x, y),
          steps: cur.steps + 1,
          from: 3,
        };
        stack.push(next);
        longests[y][x] = next.steps as isize;
      }
    }
    // downward
    if cur.pos.1 < map.len() - 1 && cur.from != 2 {
      let (x, y) = (cur.pos.0, cur.pos.1 + 1);
      if (map[y][x] == '.' || map[y][x] == 'v') && (cur.steps as isize) + 1 > longests[y][x] {
        let next = State {
          pos: Pos(x, y),
          steps: cur.steps + 1,
          from: 0,
        };
        stack.push(next);
        longests[y][x] = next.steps as isize;
      }
    }
    // leftward
    if cur.pos.0 > 0 && cur.from != 3 {
      let (x, y) = (cur.pos.0 - 1, cur.pos.1);
      if (map[y][x] == '.' || map[y][x] == '<') && (cur.steps as isize) + 1 > longests[y][x] {
        let next = State {
          pos: Pos(x, y),
          steps: cur.steps + 1,
          from: 1,
        };
        stack.push(next);
        longests[y][x] = next.steps as isize;
      }
    }
  }

  longests[goal_y][goal_x] as usize
}

pub fn solve_part2(inputs: &[String]) -> usize {
  let mut map: Vec<Vec<char>> = vec![];
  for input in inputs {
    let mut row: Vec<char> = vec![];
    for c in input.chars() {
      row.push(c);
    }
    map.push(row);
  }

  let init = StateWithPath {
    pos: Pos(1, 0),
    steps: 0,
    path: vec![Pos(0, 0)],
  };
  let mut stack: Vec<StateWithPath> = vec![init];
  let goal = Pos(map[map.len()-1].len() - 2, map.len() - 1);
  let mut paths: Vec<StateWithPath> = vec![];
  while stack.len() > 0 {
    let cur = stack.pop().unwrap();

    if cur.pos == goal {
      paths.push(cur);
      continue;
    }

    // upward
    if cur.pos.1 > 0 && !cur.path.contains(&Pos(cur.pos.0, cur.pos.1 - 1)) {
      let (x, y) = (cur.pos.0, cur.pos.1 - 1);
      if map[y][x] != '#' {
        let mut path = cur.path.to_vec();
        path.push(Pos(x, y));
        let next = StateWithPath {
          pos: Pos(x, y),
          steps: cur.steps + 1,
          path: path,
        };
        stack.push(next);
      }
    }
    // rightward
    if cur.pos.0 < map[cur.pos.1].len() - 1 &&  !cur.path.contains(&Pos(cur.pos.0 + 1, cur.pos.1)) {
      let (x, y) = (cur.pos.0 + 1, cur.pos.1);
      if map[y][x] != '#' {
        let mut path = cur.path.to_vec();
        path.push(Pos(x, y));
        let next = StateWithPath {
          pos: Pos(x, y),
          steps: cur.steps + 1,
          path: path,
        };
        stack.push(next);
      }
    }
    // downward
    if cur.pos.1 < map.len() - 1 &&  !cur.path.contains(&Pos(cur.pos.0, cur.pos.1 + 1)) {
      let (x, y) = (cur.pos.0, cur.pos.1 + 1);
      if map[y][x] != '#' {
        let mut path = cur.path.to_vec();
        path.push(Pos(x, y));
        let next = StateWithPath {
          pos: Pos(x, y),
          steps: cur.steps + 1,
          path: path,
        };
        stack.push(next);
      }
    }
    // leftward
    if cur.pos.0 > 0 &&  !cur.path.contains(&Pos(cur.pos.0 - 1, cur.pos.1)) {
      let (x, y) = (cur.pos.0 - 1, cur.pos.1);
      if map[y][x] != '#' {
        let mut path = cur.path.to_vec();
        path.push(Pos(x, y));
        let next = StateWithPath {
          pos: Pos(x, y),
          steps: cur.steps + 1,
          path: path,
        };
        stack.push(next);
      }
    }
  }

  let mut ret = 0;
  for path in paths {
    ret = ret.max(path.steps);
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
      assert_eq!(result, 94);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 2210);
    }

    #[test]
    fn part2_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 154);
    }

    #[test]
    fn part2() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 70702);
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
