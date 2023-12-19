pub fn solve_part1(inputs: &[String]) -> usize {
  let mut ret: Vec<usize> = vec![];

  for step in inputs[0].split(',') {
    let mut result: usize = 0;
    for c in step.chars() {
      let ascii = c as usize;
      result = ((result + ascii) * 17) % 256;
    }
    ret.push(result);
  }

  ret.iter().sum()
}

pub fn solve_part2(inputs: &[String]) -> usize {
  let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

  for step in inputs[0].split(',') {
    let (label, op) = if step.contains('=') {
      let cs: Vec<&str> = step.split('=').collect();
      (cs[0], '=')
    } else {
      let cs: Vec<&str> = step.split('-').collect();
      (cs[0], '-')
    };
    let mut box_no: usize = 0;
    for c in label.chars() {
      let ascii: usize = c as usize;
      box_no = ((box_no + ascii) * 17) % 256;
    }

    let i = boxes[box_no].iter().position(|&(s, _)| s == label);
    match op {
      '=' => {
        let focul = step.split('=').last().unwrap().parse::<usize>().unwrap();
        if i == None {
          boxes[box_no].push((label, focul));
        } else {
          boxes[box_no][i.unwrap()] = (label, focul);
        }
      },
      '-' => {
        if i != None {
          boxes[box_no].remove(i.unwrap());
        }
      },
      _ => unreachable!(),
    }
  }

  let mut ret = 0;
  for (i, b) in boxes.iter().enumerate() {
    for (j, (_, focul)) in b.iter().enumerate() {
      ret += (i+1) * (j+1) * focul;
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
      assert_eq!(result, 1320);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 503487);
    }

    #[test]
    fn part2_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 145);
    }

    #[test]
    fn part2() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 261505);
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
