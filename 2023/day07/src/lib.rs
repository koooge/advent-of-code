use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
enum HandType {
  FiveCard,
  FourCard,
  FullHouse,
  ThreeCard,
  TwoPair,
  OnePair,
  HighCard,
}

fn sort_cards(sets: &[(Vec<usize>, usize)]) -> Vec<(Vec<usize>, usize)> {
  let mut ret: Vec<(Vec<usize>, usize)> = sets.to_vec();
  ret.sort_by(|(cards1, _), (cards2, _)| {
    for i in 0..5 {
      if cards1[i] != cards2[i] {
        return cards1[i].cmp(&cards2[i]);
      }
    }
    unreachable!()
  });
  ret
}

pub fn solve_part1(inputs: &[String]) -> usize {
  let mut ret: usize = 0;
  let mut hands: HashMap<HandType, Vec<(Vec<usize>, usize)>> = HashMap::from([
    (HandType::FiveCard, vec![]),
    (HandType::FourCard, vec![]),
    (HandType::FullHouse, vec![]),
    (HandType::ThreeCard, vec![]),
    (HandType::TwoPair, vec![]),
    (HandType::OnePair, vec![]),
    (HandType::HighCard, vec![]),
  ]);

  for input in inputs {
    let (hand, b) = input.split_once(' ').unwrap();
    let mut cards: Vec<usize> = vec![];
    let mut map: HashMap<char, usize> = HashMap::new();
    for c in hand.chars() {
      match c {
        'A' => cards.push(14),
        'K' => cards.push(13),
        'Q' => cards.push(12),
        'J' => cards.push(11),
        'T' => cards.push(10),
        _ => cards.push(c.to_string().parse::<usize>().unwrap()),
      }
      let count = map.entry(c).or_insert(0);
      *count += 1;
    }
    let bid = b.parse::<usize>().unwrap();

    let mut values = map.into_iter().map(|(_k, v)| v).collect::<Vec<usize>>();
    values.sort();
    if values.len() == 1 {
      let sets = hands.entry(HandType::FiveCard).or_insert(vec![]);
      sets.push((cards, bid));
    } else if values.len() == 2 {
      if values.get(0) == Some(&1) {
        let sets = hands.entry(HandType::FourCard).or_insert(vec![]);
        sets.push((cards, bid));
      } else if values.get(0) == Some(&2) {
        let sets = hands.entry(HandType::FullHouse).or_insert(vec![]);
        sets.push((cards, bid));
      }
    } else if values.len() == 3 {
      if values.get(2) == Some(&3) {
        let sets = hands.entry(HandType::ThreeCard).or_insert(vec![]);
        sets.push((cards, bid));
      } else if values.get(2) == Some(&2) {
        let sets = hands.entry(HandType::TwoPair).or_insert(vec![]);
        sets.push((cards, bid));
      }
    } else if values.len() == 4 {
      let sets = hands.entry(HandType::OnePair).or_insert(vec![]);
      sets.push((cards, bid));
    } else if values.len() == 5 {
      let sets = hands.entry(HandType::HighCard).or_insert(vec![]);
      sets.push((cards, bid));
    }
  }

  let mut rank: usize = 1;
  let sorted_highcards = sort_cards(hands.get(&HandType::HighCard).unwrap());
  for bid in sorted_highcards {
    ret += bid.1 * rank;
    rank += 1;
  }
  let sorted_onepairs = sort_cards(hands.get(&HandType::OnePair).unwrap());
  for bid in sorted_onepairs {
    ret += bid.1 * rank;
    rank += 1;
  }
  let sorted_twopairs = sort_cards(hands.get(&HandType::TwoPair).unwrap());
  for bid in sorted_twopairs {
    ret += bid.1 * rank;
    rank += 1;
  }

  let sorted_threecards = sort_cards(hands.get(&HandType::ThreeCard).unwrap());
  for bid in sorted_threecards {
    ret += bid.1 * rank;
    rank += 1;
  }

  let sorted_fullhouses = sort_cards(hands.get(&HandType::FullHouse).unwrap());
  for bid in sorted_fullhouses {
    ret += bid.1 * rank;
    rank += 1;
  }

  let sorted_fourcards = sort_cards(hands.get(&HandType::FourCard).unwrap());
  for bid in sorted_fourcards {
    ret += bid.1 * rank;
    rank += 1;
  }

  let sourted_fivecards = sort_cards(hands.get(&HandType::FiveCard).unwrap());
  for bid in sourted_fivecards {
    ret += bid.1 * rank;
    rank += 1;
  }

  ret
}

// pub fn solve_part2(inputs: &[String]) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 6440);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 247961593);
    }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 71503);
    // }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 34788142);
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
