use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub struct Day2 {}

impl Day2 {
    fn test_sequence(numbers: &[i32]) -> Option<usize> {
        let first_diff = numbers[1] - numbers[0];
        if first_diff.abs() > 3 {
            return Some(0);
        }

        for (index, pair) in numbers.windows(2).enumerate() {
            let diff = pair[1] - pair[0];
            if diff.abs() < 1 || diff.abs() > 3 || diff.signum() != first_diff.signum() {
                return Some(index); // failed at the specific index
            }
        }
        None
    }

    pub fn part1() -> Result<i32> {
        let file = File::open("src/day_2/input.txt")?;
        let reader = BufReader::new(file);
        let mut safe_count = 0;

        for line in reader.lines() {
            let numbers: Vec<i32> = line?
                .split(' ')
                .map(|val| val.parse::<i32>().unwrap())
                .collect();

            if Self::test_sequence(&numbers).is_none() {
                safe_count += 1;
            }
        }
        Ok(safe_count)
    }

    pub fn part2() -> Result<u32> {
        let file = File::open("src/day_2/input.txt")?;
        let reader = BufReader::new(file);

        let mut safe_count: u32 = 0;

        for line in reader.lines() {
            let line = line?;

            let numbers: Vec<i32> = line
                .split(" ")
                .map(|val| val.parse::<i32>().unwrap())
                .collect();

            if Self::test_sequence(&numbers).is_none() {
                safe_count += 1;
                continue;
            }

            // brute force without all possible values because i'm stupid
            for i in 0..numbers.len() {
                let test_sequence: Vec<_> = numbers
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, &x)| x)
                    .collect();

                if Self::test_sequence(&test_sequence).is_none() {
                    safe_count += 1;
                    break;
                }
            }
        }
        Ok(safe_count)
    }
}
