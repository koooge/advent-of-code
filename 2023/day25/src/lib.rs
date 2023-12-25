use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve_part1(inputs: &[String]) -> usize {
  let mut connections: Vec<(&str, &str)> = vec![];
  let mut components: HashSet<&str> = HashSet::new();
  let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();

  for input in inputs {
    let (a, s) = input.split_once(": ").unwrap();
    let bs: Vec<&str> = s.split_whitespace().collect();
    components.insert(a);
    for b in bs {
      components.insert(b);
      connections.push((a, b));
      map.entry(a).or_insert(HashSet::from([b])).insert(b);
      map.entry(b).or_insert(HashSet::from([a])).insert(a);
    }
  }

  for i in 0..connections.len()-2 {
    for j in i+1..connections.len()-1 {
      for k in j+1..connections.len() {
        let mut disconnected = connections.clone();
        disconnected.remove(i);
        disconnected.remove(j-1);
        disconnected.remove(k-2);
        let mut stack = vec![disconnected[0].0];
        let mut group: HashSet<&str> = HashSet::from([disconnected[0].0, disconnected[0].1]);
        let mut checked = vec![];
        while stack.len() > 0 {
          let key = stack.pop().unwrap();
          if checked.contains(&key) {
            continue;
          }
          checked.push(key);
          let values = map.get(key).unwrap();
          for v in values {
            if !disconnected.contains(&(key, v)) && !disconnected.contains(&(v, key)){
              continue;
            }
            group.insert(v);
            if !stack.contains(v) {
              stack.push(v);
            }
          }
        }
        if group.len() < components.len() {
          let mut another_group = HashSet::new();
          for (a, b) in connections {
            if !group.contains(a) && !group.contains(b) {
              another_group.insert(a);
              another_group.insert(b);
            }
          }
          return group.len() * another_group.len();
        }
      }
    }
  }
  unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 54);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 0);
    }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 47);
    // }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 70702);
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
