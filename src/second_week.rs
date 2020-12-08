mod first_day;

use first_day::*;

use super::lines_from_file;

pub fn run_second_week() {
    run_first_day();
}

fn run_first_day() {
    let input = lines_from_file("inputs/second_week/day1.txt").expect("Could not load lines");
    let result = execute_bug_program(input.clone());
    println!("Last accumulator before breaks {}", result);
    let result = execute_and_fix_program(input);
    println!("Last accumulator fixed {}", result);
}
