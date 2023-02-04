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

                let str0 = line0.1.unwrap();
                let str1 = line1.1.unwrap();
                let str2 = line2.1.unwrap();

                let input = vec![str0, str1, str2];

                let badge = rucks.find_badge(&input);

                sum += rucks.char_eval(badge);
            } else {
                break;
            }
        }

        sum
    }
}

#[cfg(test)]
mod day3_tests {
    use crate::day3::Day3;

    #[test]
    fn test_test_input() {
        let myday3 = Day3::new();

        let test_input = "day3_test_input.txt";
        let expected = 70;

        let actual = myday3.get_second(test_input);

        assert_eq!(actual, expected);
    }
}
