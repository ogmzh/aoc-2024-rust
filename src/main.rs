use day_1::Day1;
use day_2::Day2;

pub mod day_1;
pub mod day_2;

fn main() {
    // let result = Day1::part1();
    // let result = Day1::part2();
    // let result = Day2::part1();
    let result = Day2::part2();
    println!("{}", result.unwrap());
}
