mod day3;
mod day4;
mod day5;

mod cleaning;
mod common;
mod rucks;

use crate::day3::Day3;
use crate::day5::Day5;

// TODO: replace all .unwrap() in production code. Unit tests are fine
fn main() {
    // using OO for day 3
    let day_3 = Day3::new();

    let day_3_input = "input_3_1.txt";
    let day_4_input = "input_4_1.txt";
    let day_5_input = "input_5_1.txt";

    let result = day_3.get_first(day_3_input);
    println!("day 3 part 1 is: {result}");

    let result = day_3.get_second(day_3_input);
    println!("day 3 part 2 is: {result}");

    // day 4, just functions
    let result = day4::Day4::get_first(day_4_input);

    println!("day 4 part 1 is: {result}");

    let result = day4::Day4::get_second(day_4_input);

    println!("day 4 part 2 is: {result}");

    // going OO for this one.
    let mut day_5 = Day5::new();

    println!("before actions:");
    //day_5.print_tops_in_order();

    let tops = day_5.get_tops_in_order();
    println!("{:?}", tops);

    day_5.get_first(day_5_input);

    println!("after actions:");
    //day_5.print_tops_in_order();
    let tops = day_5.get_tops_in_order();
    println!("{:?}", tops);
}
