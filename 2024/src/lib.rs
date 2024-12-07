pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

use std::fs;

pub fn read_file(file_path: &str) -> Vec<String> {
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
