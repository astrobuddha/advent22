mod day3;
mod day4;

mod cleaning;
mod common;
mod rucks;

use crate::day3::Day3;

fn main() {
    // using OO for day 3
    let day_3 = Day3::new();

    let day_3_input = "input_3_1.txt";
    let day_4_input = "input_4_1.txt";

    let result = day_3.get_first(day_3_input);
    println!("day 3 part 1 is: {result}");

    let result = day_3.get_second(day_3_input);
    println!("day 3 part 2 is: {result}");

    // day 4, just functions
    let result = day4::Day4::get_first(day_4_input);

    println!("day 4 part 1 is: {result}");

    let result = day4::Day4::get_second(day_4_input);

    println!("day 4 part 2 is: {result}");
}
