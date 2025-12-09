use std::collections::HashSet;

type Coordinate = (usize, usize, usize);

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut coordinates: Vec<Coordinate> = vec![];

    for line in inputs {
        let mut it = line.split(',').map(|s| s.parse::<usize>().unwrap());
        coordinates.push((it.next().unwrap(), it.next().unwrap(), it.next().unwrap()));
    }

    let mut distances: Vec<(f64, Coordinate, Coordinate)> = vec![];
    for i in 0..coordinates.len() - 1 {
        for j in i + 1..coordinates.len() {
            let dx = coordinates[j].0 as f64 - coordinates[i].0 as f64;
            let dy = coordinates[j].1 as f64 - coordinates[i].1 as f64;
            let dz = coordinates[j].2 as f64 - coordinates[i].2 as f64;
            let d = (dx * dx + dy * dy + dz * dz).sqrt();
            distances.push((d, coordinates[i], coordinates[j]));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Less));

    let mut circuits: Vec<HashSet<Coordinate>> = vec![];
    for d in distances.iter().take(10) {
        println!("{:?}", d);
        let idx = circuits
            .iter()
            .position(|c| c.contains(&d.1) || c.contains(&d.2));
        if let Some(idx) = idx {
            circuits[idx].insert(d.1);
            circuits[idx].insert(d.2);
        } else {
            circuits.push(HashSet::from([d.1, d.2]));
        }
    }
    println!("{:?}", circuits);

    let mut lens: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    lens.sort_by(|a, b| b.cmp(a));
    println!("{:?}", lens);
    lens.iter().take(3).product()
}

// pub fn solve_part2(inputs: &[String]) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn day08_part1_case1() {
        let inputs = read_file("./src/day08/test1.txt");
        let result = solve_part1(&inputs);
        // assert_eq!(result, 40);
        assert_eq!(result, 32);
    }

    // #[test]
    // fn part1() {
    //     let inputs = read_file("./src/day08/input1.txt");
    //     let result = solve_part1(&inputs);
    //     assert_eq!(result, 8);
    // }
}
