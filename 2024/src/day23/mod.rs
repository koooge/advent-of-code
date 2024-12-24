use std::collections::{HashMap, HashSet};

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    for input in inputs {
        let kv: Vec<&str> = input.split('-').collect();
        connections.entry(kv[0]).or_default().push(kv[1]);
        connections.entry(kv[1]).or_default().push(kv[0]);
    }
    let mut ret: HashSet<String> = HashSet::new();
    for (a, b_s) in &connections {
        if !a.starts_with('t') {
            continue;
        }
        for b in b_s {
            if let Some(c_s) = connections.get(b) {
                for c in c_s {
                    if let Some(a_s) = connections.get(c) {
                        for d in a_s {
                            if d == a {
                                let mut set_v: Vec<&str> = vec![a, b, c];
                                set_v.sort();
                                let r: String = set_v.join(",");
                                ret.insert(r);
                            }
                        }
                    }
                }
            }
        }
    }

    ret.len()
}

pub fn solve_part2(inputs: &[String]) -> String {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    for input in inputs {
        let kv: Vec<&str> = input.split('-').collect();
        connections.entry(kv[0]).or_default().push(kv[1]);
        connections.entry(kv[1]).or_default().push(kv[0]);
    }

    let mut largest: Vec<&str> = vec![];
    let mut parties: Vec<Vec<&str>> = vec![];
    for con in connections.keys() {
        parties.push(vec![con]);
    }

    let is_party = |party: &[&str], com: &str| {
        for p in party {
            let vals = connections.get(p).unwrap();
            if !vals.contains(&com) {
                return false;
            }
        }
        true
    };

    while let Some(mut party) = parties.pop() {
        let key = party.last().unwrap();
        let values = connections.get(key).unwrap();
        for v in values {
            if is_party(&party, v) {
                party.push(v);
                parties.push(party.clone());
                let new_length = party.len();
                if new_length > largest.len() {
                    largest = party.clone();
                }
            }
        }
    }
    largest.sort();
    largest.join(",").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day23/test1.txt");

        let result = solve_part1(&inputs);
        assert_eq!(result, 7);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day23/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 1378);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day23/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, String::from("co,de,ka,ta"));
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day23/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(
            result,
            String::from("bs,ey,fq,fy,he,ii,lh,ol,tc,uu,wl,xq,xv")
        );
    }
}
