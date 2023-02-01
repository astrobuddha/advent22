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

    pub fn get_second(&self, file_path: &str) -> u32 {
        let mut sum = 0;

        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let rucks = Rucks::new();

        let mut lines = reader.lines().enumerate();

        loop {
            if let Some(line0) = lines.next() {
                let line1 = lines.next().unwrap();
                let line2 = lines.next().unwrap();

                let str0 = Result::unwrap(line0.1).as_str();
                let str1 = line1.1.unwrap().as_str();
                let str2 = line2.1.unwrap().as_str();

                let input = vec![str0, str1, str2];

                let badge = rucks.find_badge(&input);
            } else {
                break;
            }
        }

        sum
    }
}
