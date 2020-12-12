mod first_week;
mod second_week;

use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn main() {
    first_week::run_first_week();
    second_week::run_second_week();
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
