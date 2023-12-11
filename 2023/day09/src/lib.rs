pub fn solve_part1(inputs: &[String]) -> isize {
  let mut ret = 0;

  for input in inputs {
    let values: Vec<isize> = input.split(' ').filter_map(|n| n.parse::<isize>().ok()).collect();
    let mut seqs: Vec<Vec<isize>> = vec![];
    seqs.push(values);
    loop {
      let last_seq = seqs.last().unwrap();
      if last_seq.iter().all(|&n| n == 0) {
        let mut num = 0;
        for seq in seqs.iter().rev() {
          let n = seq.last().unwrap();
          num += n;
        }
        ret += num;
        break;
      }
      let mut seq: Vec<isize> = vec![];
      for i in 0..last_seq.len() - 1 {
        seq.push(last_seq[i+1] - last_seq[i]);
      }
      seqs.push(seq);
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
      assert_eq!(result, 114);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 1980437560);
    }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test3.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 6);
    // }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 248750699);
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
