use std::str::FromStr;

pub fn parse_uint(input: &str) -> Result<u32, <u32 as FromStr>::Err> {
    u32::from_str(input)
}

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

        let actual = common::parse_uint(input);

        if let Err(_) = actual {
            assert!(true);
        } else {
            panic!("failed to produce error");
        }
    }
}
