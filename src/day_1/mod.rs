use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub struct Day1 {}

impl Day1 {
    pub fn part1() -> Result<u32> {
        let file = File::open("src/day_1/input.txt")?;
        let reader = BufReader::new(file);

        let mut arr1: Vec<u32> = vec![];
        let mut arr2: Vec<u32> = vec![];

        for line in reader.lines() {
            let line = line.unwrap();
            let split = line.split("  ").collect::<Vec<&str>>();
            arr1.push(split[0].parse().unwrap());
            arr2.push(split[1].trim().parse().unwrap());
        }
        arr1.sort();
        arr2.sort();
        let mut sum: u32 = 0;
        for (i, value) in arr1.into_iter().enumerate() {
            sum += value.abs_diff(arr2[i]);
        }
        Ok(sum)
    }

    pub fn part2() -> Result<u32> {
        let file = File::open("src/day_1/input.txt")?;
        let reader = BufReader::new(file);

        let mut arr1: Vec<u32> = vec![];
        let mut arr2: Vec<u32> = vec![];

        for line in reader.lines() {
            let line = line.unwrap();
            let split = line.split("  ").collect::<Vec<&str>>();
            arr1.push(split[0].parse().unwrap());
            arr2.push(split[1].trim().parse().unwrap());
        }
        let mut similarity_score: u32 = 0;

        let mut occurences_map: HashMap<u32, u32> = HashMap::new();
        for value in arr2 {
            occurences_map
                .entry(value)
                .and_modify(|val| *val += 1)
                .or_insert(1);
        }
        for value in arr1 {
            let count = occurences_map.get(&value).unwrap_or(&0);
            similarity_score += value * count;
        }
        Ok(similarity_score)
    }
}
