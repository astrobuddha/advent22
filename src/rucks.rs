use lazy_static::lazy_static;
use std::{cell::RefCell, collections::HashMap};

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

// todo add unit tests
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
        let mut counts = HashMap::new();

        for line in lines {
            for c in line.chars() {
                if !counts.contains_key(&c) {
                    counts.insert(c.to_owned(), RefCell::new(1));
                } else {
                    let value = counts.get(&c).unwrap();
                    *value.borrow_mut() += 1;
                }
            }
        }

        for (key, value) in counts.iter() {
            if *value == 3.into() {
                return *key;
            }
        }

        panic!("badge not found");
    }
}

#[cfg(test)]
mod rucks_tests {
    use crate::rucks::Rucks;

    #[test]
    fn test_find_badge() {
        let myruck = Rucks::new();

        let val = myruck.char_eval('a');

        assert!(val == 1);
    }
}
