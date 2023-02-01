mod day3;
mod rucks;

use crate::day3::Day3;

fn main() {
    // let ruck = Rucks::new();
    // let result = ruck.line_value("vJrwpWtwJgWrhcsFMMfFFhFp");

    let day_3 = Day3::new();
    let day_3_input = "input_3_1.txt";
    let result = day_3.get_first(day_3_input);
    println!("the value is: {result}")
}
