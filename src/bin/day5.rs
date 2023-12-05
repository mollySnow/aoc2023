use std::{str::FromStr, collections::HashMap};

use nom::IResult;

#[derive(Debug)]
struct Map {
    source: &'static str,
    target: &'static str,
    map: HashMap<u32, u32>,
}

type R<TOut> = IResult<&'static str, TOut>;

fn part1(input: &str) -> u32 {
    0
}

fn part2(input: &str) -> u32 {
    0
}
    

fn main() {
    aoc2023::utils::run(EXAMPLE, part1, EXAMPLE_PART_1);
}

const EXAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

const EXAMPLE_PART_1: Option<u32> = Some(35);
const EXAMPLE_PART_2: Option<u32> = None;

const INPUT_PART_2: Option<u32> = None;
const INPUT_PART_1: Option<u32> = None;
