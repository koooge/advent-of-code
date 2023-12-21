use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct State<'a> {
  t: u8,
  ins: Vec<bool>,
  out: bool,
  dsts: Vec<&'a str>,
}

pub fn solve_part1(inputs: &[String]) -> usize {
  let mut ret:(usize, usize) = (0, 0);
  let mut modules: HashMap<&str, State> = HashMap::new();

  for input in inputs {
    let (s, d) = input.split_once(" -> ").unwrap();
    let dsts: Vec<&str> = d.split(", ").collect();
    let (t, src) = if s.starts_with('%') {
      (1, &s[1..])
    } else if s.starts_with('&') {
      (2, &s[1..])
    } else {
      (0, s)
    };
    let state = State {
      t: t,
      ins: vec![false, false],
      out: false,
      dsts: dsts,
    };
    modules.insert(src, state);
  }

  for _ in 0..1 {
    ret.0 += 1; // "button" to "broadcaster"
    let mut changed: VecDeque<&str> = VecDeque::new();
    for dst in &modules.get("broadcaster").unwrap().dsts {
      ret.0 += 1;
      changed.push_front(dst);
    }

    while changed.len() > 0 {
      let name = changed.pop_back().unwrap();
      let module = modules.get(name).unwrap();
      let next = match module.t {
        1 => { // '%'
          let input = if module.ins.len() == 1 {
            module.ins[0]
          } else {
            let ins = &module.ins[1..];
            let mut new_module = module.clone();
            new_module.ins = ins.to_vec();
            modules.insert(name, new_module);
            changed.push_front(name);
            ins[0]
          };
          if !input { !module.out } else { module.out }
        },
        2 => { // '&'
          let input = if module.ins.len() == 1 {
            module.ins[0]
          } else {
            let ins0 = module.ins[0];
            module.ins.remove(0);
            modules.insert(name, module.clone());
            changed.push_front(name);
            ins0
          };
          if input && module.out { false } else { true }
        },
        _ => unreachable!(),
      };

      if next != module.out {
        for dst in &module.dsts {
          let dst_module = modules.get_mut(name).unwrap();
          changed.push_front(dst);
          ret = if next { (ret.0, ret.1 + 1) } else { (ret.0 + 1, ret.1) };
        }
      }
    }
  }

  println!("{:?}", ret);

  ret.0 * ret.1
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
      assert_eq!(result, 32000000);
    }

    // #[test]
    // fn part1_case2() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part1(&inputs);
    //   assert_eq!(result, 11687500);
    // }

    // #[test]
    // fn part1() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part1(&inputs);
    //   assert_eq!(result, 330820);
    // }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 167409079868000);
    // }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 8754);
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
