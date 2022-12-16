use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    rate: u32,
    lead_to: Vec<String>,
    is_open: bool,
}

fn act(valves: &mut HashMap<String, Valve>, current: &str, released: u32, time: u32) -> u32 {
  if time == 30 { return released; }

  let new_time: u32 = time + 1;
  let mut new_released: u32 = released;
  for (_, v) in valves.iter() {
    if v.is_open {
      new_released += v.rate;
    }
  };

  let mut binding = valves.clone();
  let mut valve = binding.get_mut(current).unwrap();
  let mut results: Vec<u32> = Vec::new();
  if !valve.is_open {
    valve.is_open = true;
  }
  for dist in &valve.lead_to {
    results.push(act(&mut binding, dist, new_released, new_time));
  }

  *results.iter().max().unwrap()
}

pub fn solve_part1(inputs: &[String]) -> u32 {
    let mut valves: HashMap<String, Valve> = HashMap::new();

    for input in inputs {
        let name = String::from(&input[6..=7]);
        let rate: u32 = input.split(';').collect::<Vec<&str>>()[0]
            .split('=')
            .collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let lead_to: Vec<String> = input.split(' ').collect::<Vec<&str>>()[9..]
            .iter()
            .map(|x| x.split(',').collect::<Vec<&str>>()[0].to_string())
            .collect();
        valves.insert(
            name.clone(),
            Valve {
                name,
                rate,
                lead_to,
                is_open: false,
            },
        );
    }

    act(&mut valves, "AA", 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs);
        println!("{}", result);
        assert_eq!(result, 1651);
    }

    // #[test]
    // fn part1() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part1(&inputs, 2000000);
    //     println!("{}", result);
    //     assert_eq!(result, 5166077);
    // }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 56000011);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 13071206703981);
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
