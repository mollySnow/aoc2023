use nom::{IResult, multi::separated_list1, character::complete::{multispace1, digit1, u64}, combinator::recognize, sequence::pair, bytes::complete::{tag, self, take_until, take_while}};

type Race = (u64, u64);
type R<'a,T> = IResult<&'a str, T>;

fn numbers(input: &str) -> R<Vec<u64>> {
    let (input, _) = take_while(|s: char| !s.is_numeric())(input)?;
    separated_list1(multispace1, u64)(input)
}

fn parse_races(input: &str) -> R<Vec<Race>> {
    let (input, times) = numbers(input)?;
    let (input, distances) = numbers(input)?;

    Ok((input, times.into_iter().zip(distances.into_iter()).collect()))
}

fn calculate_distance_from(input: &Race) -> u64 {
    (1..input.0).map(|t| (input.0 - t) * t).filter(|d| {
        d > &input.1
    }).count() as u64
}

fn part1(input: &str) -> u32 { 
    let races =  {
        match parse_races(input) {
            Ok((_,r)) => r,
            Err(e) => panic!("Failed to parse input: {:?}", e)
        }
    };

    races.into_iter().fold(1, |mut acc, r| {acc *= calculate_distance_from(&r); acc }) as u32
}

fn main() {
    // aoc2023::utils::run(INPUT, part1, INPUT_PART_1);
    // aoc2023::utils::run(EXAMPLE_2, part1, EXAMPLE_PART_2);
    aoc2023::utils::run(INPUT_2, part1, INPUT_PART_2);
    // aoc2023::utils::run(INPUT, part2, INPUT_PART_2);
    // aoc2023::utils::run(EXAMPLE, part1, EXAMPLE_PART_1);
}

const EXAMPLE_PART_1: Option<u32> = Some(288);
const EXAMPLE_PART_2: Option<u32> = Some(71503);
const INPUT_PART_2: Option<u32> = Some(42550411);
const INPUT_PART_1: Option<u32> = Some(1195150);

const EXAMPLE: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

const EXAMPLE_2: &str = r#"Time:      71530
Distance:  940200"#;

const INPUT: &str = r#"Time:        54     94     65     92
Distance:   302   1476   1029   1404"#;

const INPUT_2: &str = r#"Time: 54946592
Distance:   302147610291404"#;
