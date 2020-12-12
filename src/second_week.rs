mod first_day;
mod second_day;
mod third_day;

use first_day::*;
use second_day::*;
use third_day::*;

use super::lines_from_file;

pub fn run_second_week() {
    run_first_day();
    run_second_day();
    run_third_day()
}

fn run_first_day() {
    let input = lines_from_file("inputs/second_week/day1.txt").expect("Could not load lines");
    let result = execute_bug_program(input.clone());
    println!("Last accumulator before breaks {}", result);
    let result = execute_and_fix_program(input);
    println!("Last accumulator fixed {}", result);
}

fn run_second_day() {
    let input = lines_from_file("inputs/second_week/day2.txt").expect("Could not load lines");
    if let Some(n) = find_number(input.clone(), 25) {
        println!("The vulnerability is {}", n);
    } else {
        println!("Not found");
    }
    if let Some(n) = find_number_in_range(input, 25) {
        println!("The vulnerability range sum is {}", n);
    } else {
        println!("Not found");
    }
}
fn run_third_day() {
    let input = lines_from_file("inputs/second_week/day3.txt").expect("Could not load lines");
    let result = mult_differece(input.clone());
    println!("The mult of differences is {}", result);
    let result = find_possibilities(input);
    println!("The possibilities are {}", result);
}
