use crate::common;

pub struct Cleaning;

impl Cleaning {
    pub fn get_pairs(line: &str) -> ((u32, u32), (u32, u32)) {
        let pairs: Vec<&str> = line.split(",").collect();

        if pairs.len() != 2 {
            panic!("incorrect number of pairs");
        }

        let first: Vec<&str> = pairs[0].split("-").collect();
        let second: Vec<&str> = pairs[1].split("-").collect();

        if first.len() != 2 || second.len() != 2 {
            panic!("incorrect number of pairs");
        }

        let mut as_nums: Vec<u32> = Vec::new();

        for num in first {
            if let Ok(n) = common::parse_uint(num) {
                as_nums.push(n);
            } else {
                panic!("could not parse nums in first pair.");
            }
        }

        for num in second {
            if let Ok(n) = common::parse_uint(num) {
                as_nums.push(n);
            } else {
                panic!("could not parse nums in second pair.");
            }
        }

        let pair1 = (as_nums[0], as_nums[1]);
        let pair2 = (as_nums[2], as_nums[3]);

        (pair1, pair2)
    }

    pub fn all_overlap(pair1: (u32, u32), pair2: (u32, u32)) -> u32 {
        if pair1.0 <= pair2.0 && pair1.1 >= pair2.1 {
            return 1;
        }

        if pair1.0 >= pair2.0 && pair1.1 <= pair2.1 {
            return 1;
        }

        0
    }

    pub fn any_overlap(pair1: (u32, u32), pair2: (u32, u32)) -> u32 {
        if pair2.0 <= pair1.1 && pair1.0 <= pair2.0 {
            return 1;
        }

        if pair1.0 <= pair2.1 && pair2.0 <= pair1.0 {
            return 1;
        }

        0
    }
}

#[cfg(test)]
mod cleaning_tests {
    use crate::cleaning::Cleaning;

    #[test]
    fn get_pairs_when_validpairs_expect_u32s() {
        let line = "2-4,6-8";

        let expected = ((2, 4), (6, 8));

        let actual = Cleaning::get_pairs(line);

        assert_eq!(expected, actual);
    }

    #[test]
    fn add_overlap_tests() {
        let input = ((2, 4), (6, 8));

        let expected = 0;
        let actual = Cleaning::all_overlap(input.0, input.1);

        assert!(actual == expected);

        let input = ((2, 8), (3, 7));
        let expected = 1;
        let actual = Cleaning::all_overlap(input.0, input.1);

        assert!(actual == expected);

        let input = ((6, 6), (4, 6));
        let expected = 1;
        let actual = Cleaning::all_overlap(input.0, input.1);

        assert!(actual == expected);
    }
}
