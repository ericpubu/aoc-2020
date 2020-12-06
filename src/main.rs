mod first_day;
mod second_day;

use first_day::*;
use second_day::*;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    execute_first_day();
    execute_second_day();
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
fn execute_first_day() {
    let list = lines_from_file("inputs/day1.txt").expect("Could not load lines");
    let input = list
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let result = expenses(input.clone());
    println!("Result of first multiplication is: {}", result);
    let result = expenses2(input);
    println!("Result of second multiplication is: {}", result);
}

fn execute_second_day() {
    let list = lines_from_file("inputs/day2.txt").expect("Could not load lines");
    let result = incorrect_passwords(list.clone());
    println!("Incorrect passwords: {}", result);
    let result = correct_passwords(list);
    println!("Correct passwords: {}", result);
}
