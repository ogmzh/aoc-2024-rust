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

        // let mut arr1: Vec<u32> = vec![];
        // let mut arr2: Vec<u32> = vec![];

        // for line in reader.lines() {
        //     let line = line.unwrap();
        //     let split = line.split("  ").collect::<Vec<&str>>();
        //     arr1.push(split[0].parse().unwrap());
        //     arr2.push(split[1].trim().parse().unwrap());
        // }

        let (mut arr1, mut arr2): (Vec<u32>, Vec<u32>) = reader
            .lines()
            .map_while(Result::ok)
            // .inspect(|line| println!("LINE: {line}"))
            .filter_map(|line| {
                let split: Vec<&str> = line.split_whitespace().collect();
                Some((split[0].parse::<u32>().ok()?, split[1].parse::<u32>().ok()?))
            })
            .unzip();

        arr1.sort();
        arr2.sort();
        // let mut sum: u32 = 0;
        // for (i, value) in arr1.into_iter().enumerate() {
        //     sum += value.abs_diff(arr2[i]);
        // }
        let sum = arr1.iter().zip(&arr2).map(|(a, b)| a.abs_diff(*b)).sum();
        Ok(sum)
    }

    pub fn part2() -> Result<u32> {
        let file = File::open("src/day_1/input.txt")?;
        let reader = BufReader::new(file);

        let (arr1, arr2): (Vec<u32>, Vec<u32>) = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| {
                let split: Vec<&str> = line.split_whitespace().collect();
                Some((split[0].parse::<u32>().ok()?, split[1].parse::<u32>().ok()?))
            })
            .unzip();

        // let mut similarity_score: u32 = 0;
        // let mut occurences_map: HashMap<u32, u32> = HashMap::new();
        // for value in arr2 {
        //     occurences_map
        //         .entry(value)
        //         .and_modify(|val| *val += 1)
        //         .or_insert(1);
        // }
        // for value in arr1 {
        //     let count = occurences_map.get(&value).unwrap_or(&0);
        //     similarity_score += value * count;
        // }
        let occurences_map: HashMap<u32, u32> = // Default::default() is an idiomatic replacement for HashMap::new()
            arr2.iter().fold(Default::default(), |mut map, &val| {
                *map.entry(val).or_default() += 1;
                map
            });
        let similarity_score = arr1
            .iter()
            .map(|val| val * occurences_map.get(val).unwrap_or(&0))
            .sum();
        Ok(similarity_score)
    }
}
