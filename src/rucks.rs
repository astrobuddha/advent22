use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MAP: HashMap<char, u32> = {
        let data = vec![
            ('a', 1),
            ('b', 2),
            ('c', 3),
            ('d', 4),
            ('e', 5),
            ('f', 6),
            ('g', 7),
            ('h', 8),
            ('i', 9),
            ('j', 10),
            ('k', 11),
            ('l', 12),
            ('m', 13),
            ('n', 14),
            ('o', 15),
            ('p', 16),
            ('q', 17),
            ('r', 18),
            ('s', 19),
            ('t', 20),
            ('u', 21),
            ('v', 22),
            ('w', 23),
            ('x', 24),
            ('y', 25),
            ('z', 26),
            ('A', 27),
            ('B', 28),
            ('C', 29),
            ('D', 30),
            ('E', 31),
            ('F', 32),
            ('G', 33),
            ('H', 34),
            ('I', 35),
            ('J', 36),
            ('K', 37),
            ('L', 38),
            ('M', 39),
            ('N', 40),
            ('O', 41),
            ('P', 42),
            ('Q', 43),
            ('R', 44),
            ('S', 45),
            ('T', 46),
            ('U', 47),
            ('V', 48),
            ('W', 49),
            ('X', 50),
            ('Y', 51),
            ('Z', 52),
        ];

        data.into_iter().collect()
    };
}

pub struct Rucks;

impl Rucks {
    pub fn new() -> Rucks {
        Rucks
    }

    pub fn line_value(&self, line: &str) -> u32 {
        self.char_eval(self.find_char(self.split(line)))
    }

    pub fn char_eval(&self, input: char) -> u32 {
        let mut sum = 0;

        if let Some(k) = MAP.get(&input) {
            sum = *k;
        }

        sum
    }

    fn split<'a>(&self, sack: &'a str) -> (&'a str, &'a str) {
        let mid = sack.chars().count() / 2;

        let first = &sack[0..mid];
        let second = &sack[mid..];

        (first, second)
    }

    fn find_char(&self, packs: (&str, &str)) -> char {
        for c in packs.0.chars() {
            if packs.1.contains(c) {
                return c;
            }
        }

        '0'
    }

    pub fn find_badge(&self, lines: &[String]) -> char {
        if lines.len() < 3 {
            panic!("Not enough elves!");
        }

        if lines.len() > 3 {
            panic!("Too many elves!");
        }

        let first = lines[0].chars();

        for c in first {
            if lines[1].contains(c) && lines[2].contains(c) {
                return c;
            }
        }

        panic!("badge not found");
    }
}

#[cfg(test)]
mod rucks_tests {
    use crate::rucks::Rucks;

    #[test]
    fn test_char_eval() {
        let myruck = Rucks::new();

        let val = myruck.char_eval('a');

        assert!(val == 1);
    }

    #[test]
    fn test_find_badge() {
        let myruck = Rucks::new();

        let lines = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
        ];

        let expected = 'r';

        let actual = myruck.find_badge(&lines);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_badge_sum() {
        let myruck = Rucks::new();

        let lines = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
        ];

        let expected = 'r';

        let actual = myruck.find_badge(&lines);

        assert_eq!(actual, expected);

        let lines2 = vec![
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        let expected2 = 'Z';

        let actual2 = myruck.find_badge(&lines2);

        assert_eq!(actual2, expected2);

        let mut sum = 0;

        sum += myruck.char_eval(actual);
        sum += myruck.char_eval(actual2);

        let expected_sum = 70;

        assert_eq!(sum, expected_sum);
    }
}
