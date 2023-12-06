use nom::{IResult, multi::separated_list1, character::complete::{multispace1, digit1}, combinator::recognize, sequence::pair, bytes::complete::tag};
use nom::combinator::map;

type Race = (u32, u32);
type R<'a,T> = IResult<&'a str, T>;

fn parse_u32(input: &str) -> R<Vec<u32>> {
    separated_list1(multispace1, map(digit1, |s: &str| s.parse::<u32>().unwrap()))(input)
}

fn parse_races(input: &str) -> R<Vec<Race>> {
    let (input, _) = recognize(pair(tag("Time:"), multispace1))(input)?;
    let (input, times) = parse_u32(input)?;
    let (input, _) = recognize(pair(tag("Distance:"), multispace1))(input.trim())?;
    let (input, distances) = parse_u32(input)?;

    Ok((input, times.into_iter().zip(distances.into_iter()).collect()))
}

fn calculate_distance(button_time: u32, record_time: u32) -> u32 {
    (record_time - button_time) * button_time
}

fn calculate_record_count(input: &Race) -> u32 {
    let (time, record) = input;
    calculate_distance(*time, *record)
}

fn part1(input: &str) -> u32 { 
    let races =  {
        match parse_races(input) {
            Ok((_,r)) => r,
            Err(e) => panic!("Failed to parse input: {:?}", e)
        }
    };

    for (time, distance) in races {
        let mut ok = 0;
        for t in 1..time {
            let d = calculate_distance(t, distance);
            if d > distance {
                ok += 1;
            }
        }
        println!("{} {} {}", time, distance, ok);
    }
    0 
}

fn part2(input: &str) -> u32 { 0 }

fn main() {
    // aoc2023::utils::run(INPUT, part1, INPUT_PART_1);
    // aoc2023::utils::run(INPUT, part2, INPUT_PART_2);
    aoc2023::utils::run(EXAMPLE, part1, EXAMPLE_PART_1);
}

const EXAMPLE_PART_1: Option<u32> = Some(288);
const EXAMPLE_PART_2: Option<u32> = Some(46);
const INPUT_PART_2: Option<u32> = Some(136096660);
const INPUT_PART_1: Option<u32> = Some(825516882);

const EXAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

const INPUT: &str = r#"Time:        54     94     65     92
Distance:   302   1476   1029   1404"#;
