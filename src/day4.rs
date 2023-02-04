use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::cleaning::Cleaning;

pub struct Day4 {}

impl Day4 {
    pub fn get_first(file_path: &str) -> u32 {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut sum = 0;

        reader.lines().for_each(|line| {
            if let Ok(l) = line {
                sum += Self::check_line(&l.as_str());
            }
        });

        sum
    }

    pub fn check_line(line: &str) -> u32 {
        let pairs = Cleaning::get_pairs(line);

        Cleaning::add_overlap(pairs)
    }
}

#[cfg(test)]
mod day4_tests {
    use crate::day4::Day4;

    #[test]
    fn test_input() {
        let file_path = "day4_test_input.txt";
        let actual = Day4::get_first(file_path);
        let expected = 2;

        assert!(actual == expected);
    }
}
