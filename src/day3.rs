use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::rucks::Rucks;

// todo add unit tests
pub struct Day3 {}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {}
    }

    pub fn get_first(&self, file_path: &str) -> u32 {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let rucks = Rucks::new();

        let mut sum = 0;

        reader.lines().for_each(|line| {
            sum += rucks.line_value(&line.unwrap());
        });

        sum
    }
}
