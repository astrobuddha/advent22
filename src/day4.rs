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
                sum += Self::check_line(&l.as_str(), false);
            }
        });

        sum
    }

    pub fn get_second(file_path: &str) -> u32 {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut sum = 0;

        reader.lines().for_each(|line| {
            if let Ok(l) = line {
                sum += Self::check_line(&l.as_str(), true);
            }
        });

        sum
    }

    pub fn check_line(line: &str, any: bool) -> u32 {
        let pairs = Cleaning::get_pairs(line);

        if any {
            return Cleaning::any_overlap(pairs.0, pairs.1);
        }

        Cleaning::all_overlap(pairs.0, pairs.1)
    }
}

#[cfg(test)]
mod day4_tests {
    use crate::day4::Day4;

    #[test]
    fn first_test_input() {
        let file_path = "day4_test_input.txt";
        let actual = Day4::get_first(file_path);
        let expected = 2;

        assert!(actual == expected);
    }

    #[test]
    fn second_test_input() {
        let file_path = "day4_test_input.txt";
        let actual = Day4::get_second(file_path);
        let expected = 4;

        println!("actual: {actual}");

        assert!(actual == expected);
    }
}
