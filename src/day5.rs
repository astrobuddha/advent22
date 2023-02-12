use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::common;

pub struct Day5 {
    pub stacks: HashMap<i32, Vec<char>>,
    keys: Vec<i32>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Move {
    num_crates: i32,
    from: i32,
    to: i32,
}

#[derive(PartialEq, Eq, Debug)]
pub struct MoveError;

// we are not doing the text GUI, the crate stacks will be
// represented by logical stacks
impl Day5 {
    pub fn new() -> Day5 {
        let mut stacks: HashMap<i32, Vec<char>> = HashMap::new();
        let keys = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let stack1 = vec!['F', 'T', 'C', 'L', 'R', 'P', 'G', 'Q'];
        let stack2 = vec!['N', 'Q', 'H', 'W', 'R', 'F', 'S', 'J'];
        let stack3 = vec!['F', 'B', 'H', 'W', 'P', 'M', 'Q'];
        let stack4 = vec!['V', 'S', 'T', 'D', 'F'];
        let stack5 = vec!['Q', 'L', 'D', 'W', 'V', 'F', 'Z'];
        let stack6 = vec!['Z', 'C', 'L', 'S'];
        let stack7 = vec!['Z', 'B', 'M', 'V', 'D', 'F'];
        let stack8 = vec!['T', 'J', 'B'];
        let stack9 = vec!['Q', 'N', 'B', 'G', 'L', 'S', 'P', 'H'];

        stacks.insert(keys[0], stack1);
        stacks.insert(keys[1], stack2);
        stacks.insert(keys[2], stack3);
        stacks.insert(keys[3], stack4);
        stacks.insert(keys[4], stack5);
        stacks.insert(keys[5], stack6);
        stacks.insert(keys[6], stack7);
        stacks.insert(keys[7], stack8);
        stacks.insert(keys[8], stack9);

        Day5 { stacks, keys }
    }

    pub fn print_stacks(&mut self) {
        let ret: Vec<&Vec<char>> = self.stacks.values().into_iter().collect();

        ret.iter().for_each(|stack| {
            println!("{:?}", stack);
        });
    }

    pub fn print_in_order(&mut self) {
        let mut ret: Vec<&Vec<char>> = Vec::new();

        self.keys.iter().for_each(|key| {})
    }

    pub fn get_first(&mut self, file_path: &str) {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        reader.lines().for_each(|line| {
            let action = self.parse_move(&line.unwrap());

            if let Ok(act) = action {
                self.do_action(act);
            } else {
                panic!("cannot interpret action!");
            }
        });
    }

    pub fn parse_move(&self, line: &str) -> Result<Move, MoveError> {
        let parts: Vec<&str> = line.split(" ").collect();

        if parts.len() != 6 {
            return Err(MoveError);
        }

        let num_crates: i32;
        let from: i32;
        let to: i32;

        match common::parse_int(parts[1]) {
            Ok(n) => num_crates = n,
            Err(_) => return Err(MoveError),
        }

        match common::parse_int(parts[3]) {
            Ok(n) => from = n,
            Err(_) => return Err(MoveError),
        }

        match common::parse_int(parts[5]) {
            Ok(n) => to = n,
            Err(_) => return Err(MoveError),
        }

        Ok(Move {
            num_crates,
            from,
            to,
        })
    }

    // point of learning here: in order to pop, self has to be mut.
    pub fn do_move(&mut self, from: i32, to: i32) {
        let to_move = self.stacks.get_mut(&from).unwrap().pop().unwrap();

        self.stacks.get_mut(&to).unwrap().push(to_move);
    }

    pub fn do_action(&mut self, action: Move) {
        for _ in 0..action.num_crates {
            self.do_move(action.from, action.to);
        }
    }
}

#[cfg(test)]
mod day5_test {
    use crate::day5::Day5;
    use crate::day5::Move;
    use crate::day5::MoveError;

    #[test]
    fn test_constructor_stacks() {
        let myday5 = Day5::new();

        let expected = 9;
        let num_stacks = myday5.stacks.len();

        assert!(num_stacks == expected);
    }

    #[test]
    fn test_all_stacks() {
        let myday5 = Day5::new();

        let tops: Vec<char> = myday5
            .stacks
            .iter()
            .map(|stack| -> char { stack.1.last().expect("cannot get package!").clone() })
            .collect();

        let expected = vec!['Q', 'J', 'Q', 'F', 'Z', 'S', 'F', 'B', 'H'];

        tops.iter().for_each(|pack| {
            assert!(expected.contains(pack));
        })
    }

    #[test]
    fn parse_move_when_validmove_expect_ok() {
        let myday = Day5::new();

        let action = "move 1 from 8 to 1";

        let expected = Move {
            num_crates: 1,
            from: 8,
            to: 1,
        };

        let actual = myday.parse_move(action).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_move_when_incomplete_move_expect_move_error() {
        let myday = Day5::new();

        let action = "move 1 from 8 to ";

        let expected = Err(MoveError);

        let actual = myday.parse_move(action);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_move_when_too_much_info_expect_move_error() {
        let myday = Day5::new();

        let action = "move 1 from 8 to 9 and more";

        let expected = Err(MoveError);

        let actual = myday.parse_move(action);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_move_when_parse_issue_expect_move_error() {
        let myday = Day5::new();

        let action = "move one from 8 to 9";

        let expected = Err(MoveError);

        let actual = myday.parse_move(action);

        assert_eq!(actual, expected);
    }
}
