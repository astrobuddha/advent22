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

    pub fn evaluate(&self, words: &str) -> u32 {
        let mut sum = 0;

        for c in words.chars() {
            if let Some(k) = MAP.get(&c) {
                sum += k;
            }
        }

        sum
    }

    pub fn char_eval(&self, input: char) -> u32 {
        let mut sum = 0;

        if let Some(k) = MAP.get(&c) {
            sum = k;
        }

        sum
    }

    pub fn split(&self, sack: &str) -> (str, str) {
        let mid = sack.chars().count() / 2;

        let (first, second) = sack.split_at(mid);

        (first, second)
    }

    pub fn find_char(&self, packs: (&str, &str)) -> char {
        for c in packs.0.chars() {
            if packs.1.contains(c) {
                return c;
            }
        }

        '0'
    }
}
