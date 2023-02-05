pub struct Day5 {
    pub stacks: Vec<Vec<char>>,
}

// we are not doing the text GUI, the crate stacks will be
// represented by logical stacks
impl Day5 {
    pub fn new() -> Day5 {
        let mut stacks: Vec<Vec<char>> = Vec::new();

        let stack0 = vec!['F', 'T', 'C', 'L', 'R', 'P', 'G', 'Q'];
        let stack1 = vec!['N', 'Q', 'H', 'W', 'R', 'F', 'S', 'J'];
        let stack2 = vec!['F', 'B', 'H', 'W', 'P', 'M', 'Q'];
        let stack3 = vec!['V', 'S', 'T', 'D', 'F'];
        let stack4 = vec!['Q', 'L', 'D', 'W', 'V', 'F', 'Z'];
        let stack5 = vec!['Z', 'C', 'L', 'S'];
        let stack6 = vec!['Z', 'B', 'M', 'V', 'D', 'F'];
        let stack7 = vec!['T', 'J', 'B'];
        let stack8 = vec!['Q', 'N', 'B', 'G', 'L', 'S', 'P', 'H'];

        stacks.push(stack0);
        stacks.push(stack1);
        stacks.push(stack2);
        stacks.push(stack3);
        stacks.push(stack4);
        stacks.push(stack5);
        stacks.push(stack6);
        stacks.push(stack7);
        stacks.push(stack8);

        Day5 { stacks }
    }
}

#[cfg(test)]
mod day5_test {
    use crate::day5::Day5;

    #[test]
    fn test_constructor_stacks() {
        let myday5 = Day5::new();

        let top_s0 = myday5.stacks[0].last().unwrap().clone();

        assert_eq!(top_s0, 'Q');
    }
}
