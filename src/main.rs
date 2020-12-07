mod fifth_day;
mod first_day;
mod fourth_day;
mod second_day;
mod sixth_day;
mod third_day;

use fifth_day::*;
use first_day::*;
use fourth_day::*;
use second_day::*;
use sixth_day::*;
use third_day::*;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    execute_first_day();
    execute_second_day();
    execute_third_day();
    execute_fourth_day();
    execute_fifth_day();
    execute_sixth_day()
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

fn execute_third_day() {
    let input = lines_from_file("inputs/day3.txt").expect("Could not load lines");
    let result = tree_map(input.clone(), 1, 3);
    println!("Number of trees: {}", result);
    let attempts: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let result = check_slots(input, attempts);
    println!("Number of tree posibilities: {}", result);
}

fn execute_fourth_day() {
    let input = lines_from_file("inputs/day4.txt").expect("Could not load lines");
    let result = validate_passport(input.clone());
    println!("Number of \"valid\" passports: {}", result);
    let result = validate_passport_content(input);
    println!("Number of valid passports: {}", result);
}

fn execute_fifth_day() {
    let input = lines_from_file("inputs/day5.txt").expect("Could not load lines");
    let result = highest_seat_id(input.clone());
    println!("Highest seat id: {}", result);
    let result = user_seat_id(input);
    println!("User seat id: {}", result);
}

fn execute_sixth_day() {
    let input = lines_from_file("inputs/day6.txt").expect("Could not load lines");
    let result = sum_answers(input.clone(), false);
    println!("The answers sum is: {}", result);
    let result = sum_answers(input, true);
    println!("The correct answers sum is: {}", result);
}
