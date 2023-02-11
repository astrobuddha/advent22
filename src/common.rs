use std::str::FromStr;

pub fn parse_uint(input: &str) -> Result<u32, <u32 as FromStr>::Err> {
    u32::from_str(input.trim())
}

pub fn parse_int(input: &str) -> Result<i32, <i32 as FromStr>::Err> {
    i32::from_str(input.trim())
}

// pub fn parse_usize(input: &str) -> Result<usize, <usize as FromStr>::Err> {
//     usize::from_str(input.trim())
// }

#[cfg(test)]
mod common_tests {

    use crate::common;

    #[test]
    fn parse_uint_when_validstr_expect_u32() {
        let input = "32";

        let expected = 32;
        let actual = common::parse_uint(input).unwrap();

        assert!(actual == expected);
    }

    #[test]
    fn parse_uint_when_negative_expect_err() {
        let input = "-32";

        let actual = common::parse_uint(input);

        if let Err(_) = actual {
            assert!(true);
        } else {
            panic!("failed to produce error");
        }
    }

    #[test]
    fn parse_uint_when_nonnumber_expect_err() {
        let input = "ten";

        let actual = common::parse_int(input);

        if let Err(_) = actual {
            assert!(true);
        } else {
            panic!("failed to produce error");
        }
    }

    #[test]
    fn parse_uint_when_whitespace_expect_uint() {
        let input = "   32    ";

        let expected = 32;
        let actual = common::parse_uint(input).unwrap();

        assert!(actual == expected);
    }

    #[test]
    fn parse_int_when_validstr_expect_u32() {
        let input = "32";

        let expected = 32;
        let actual = common::parse_int(input).unwrap();

        assert!(actual == expected);
    }

    #[test]
    fn parse_int_when_negative_expect_value() {
        let input = "-32";

        let expected = -32;
        let actual = common::parse_int(input).unwrap();

        assert!(actual == expected);
    }

    #[test]
    fn parse_int_when_nonnumber_expect_err() {
        let input = "ten";

        let actual = common::parse_int(input);

        if let Err(_) = actual {
            assert!(true);
        } else {
            panic!("failed to produce error");
        }
    }

    #[test]
    fn parse_int_when_whitespace_expect_uint() {
        let input = "   32    ";

        let expected = 32;
        let actual = common::parse_int(input).unwrap();

        assert!(actual == expected);
    }
}
