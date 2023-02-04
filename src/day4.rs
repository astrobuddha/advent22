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

    pub fn get_second(file_path: &str) -> u32 {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut pairs: Vec<(u32, u32)> = Vec::new();

        reader.lines().for_each(|line| {
            if let Ok(l) = line {
                let line_pairs = Cleaning::get_pairs(&l);

                pairs.push(line_pairs.0);
                pairs.push(line_pairs.1);
            }
        });

        let result = Cleaning::get_all_overlap(&pairs);

        result
    }

    pub fn check_line(line: &str) -> u32 {
        let pairs = Cleaning::get_pairs(line);

        Cleaning::add_overlap(pairs.0, pairs.1)
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
