use day_1::Day1;
use day_2::Day2;

pub mod day_1;
pub mod day_2;

fn main() {
    let day1_result1 = Day1::part1();
    let day1_result2 = Day1::part2();
    let day2_result1 = Day2::part1();
    let day2_result2 = Day2::part2();
    println!("{}", day2_result2.unwrap());
}
