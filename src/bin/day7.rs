use std::cmp::Ordering;

use nom::{IResult, multi::separated_list1, character::{complete::{multispace1, digit1, u64, space1}, is_alphanumeric}, combinator::recognize, sequence::{pair, tuple}, bytes::complete::{tag, self, take_until, take_while}};

type Race = (u64, u64);
type R<'a,T> = IResult<&'a str, T>;

fn part1(input: &str) -> u32 {
    let mut things = parse_many(input).unwrap().1;

    things.sort_by(|a, b| {
        let (str_a, _, hand_a) = a;
        let (str_b, _, hand_b) = b;

        match str_a.cmp(str_b) {
            std::cmp::Ordering::Equal => {
                hand_a.cmp(hand_b)
            }
            other => other,
        }
    });

    for (strength, bid, hand) in things {
        println!("[{}]: {}, bid: {}", hand, strength, bid );
    }

    0
}

fn compare_hand(a: &str, b: &str) -> Ordering {
}

const LOOKUP

const ALLOWED: &str = "AKQJT98765432";

// fn parse_many(input: &str) -> R<Vec<([u8;13], u16, &str)>> {
//     separated_list1(multispace1, parse)(input)
// }

fn parse_many(input: &str) -> R<Vec<(u8, u16, &str)>> {
    separated_list1(multispace1, parse)(input)
}

fn parse(input: &str) -> R<(u8, u16, &str)> {
    let (rest, (hand, _, bid)) = tuple((
        take_while(|c: char| c.is_alphanumeric()),
        space1,
        nom::character::complete::u16,
    ))(input)?;

    let mut cards = [0u8;13];

    for c in hand.chars() {
        let idx = match c {
            'A' => 0,
            'K' => 1,
            'Q' => 2,
            'J' => 3,
            'T' => 4,
            '9' => 5,
            '8' => 6,
            '7' => 7,
            '6' => 8,
            '5' => 9,
            '4' => 10,
            '3' => 11,
            '2' => 12,
            _ => unreachable!(),
        };
        cards[idx] += 1;
    };

    let mut strength = 1;

    for i in 0..13 {
        match cards[i] {
            0 => continue,
            1 => continue,
            2 => strength *= 2,
            3 => strength *= 3,
            4 => strength *= 5,
            5 => strength *= 7,
            _ => unreachable!(),
        }
    }

    let strength = match strength {
        7 => 10, // five of a kind
        5 => 5,  // four of a kind
        6 => 4,  // full house
        3 => 3,  // three of a kind
        4 => 2,  // two pair
        2 => 1,  // pair
        1 => 0, 
        _ => unreachable!(),
    };

    Ok((rest, (strength, bid, hand)))
}

// fn parse(input: &str) -> R<([u8;13], u16, &str)> {
//     let (rest, (hand, _, bid)) = tuple((
//         take_while(|c: char| c.is_alphanumeric()),
//         space1,
//         nom::character::complete::u16,
//     ))(input)?;
//
//     let mut cards = [0u8;13];
//
//     for c in hand.chars() {
//         let idx = match c {
//             'A' => 0,
//             'K' => 1,
//             'Q' => 2,
//             'J' => 3,
//             'T' => 4,
//             '9' => 5,
//             '8' => 6,
//             '7' => 7,
//             '6' => 8,
//             '5' => 9,
//             '4' => 10,
//             '3' => 11,
//             '2' => 12,
//             _ => unreachable!(),
//         };
//         cards[idx] += 1;
//     }
//     cards.sort();
//     Ok((rest, (cards, bid, hand)))
// }

fn find_hand(hand: &[u8]) -> &str {
    match hand {
        [1,1,1,1,1] => {
            "high card"
            // println!("high card: {:?}", hand);
        },
        [0,0,0,1,4] => {
            "four of a kind"
            // println!("four of a kind: {:?}", hand);
        },
        [0,0,1,2,3] => {
            "full house"
            // println!("full house: {:?}", hand);
        },
        [0,0,1,1,3] => {
            "three of a kind"
            // println!("three of a kind: {:?}", hand);
        },
        [0,0,1,2,2] => {
            "two pair"
            // println!("two pair: {:?}", hand);
        },
        [0,1,1,1,2] => {
            "pair"
            // println!("pair: {:?}", hand);
        }
        [..] => {
            "n/a"
            // println!("any: {:?}", hand);
        },
    }
}

fn main() {
    // aoc2023::utils::run(INPUT, part1, INPUT_PART_1);
    // aoc2023::utils::run(EXAMPLE_2, part1, EXAMPLE_PART_2);
    aoc2023::utils::run(EXAMPLE, part1, EXAMPLE_PART_1);
    // aoc2023::utils::run(INPUT, part2, INPUT_PART_2);
    // aoc2023::utils::run(EXAMPLE, part1, EXAMPLE_PART_1);
}

#[allow(dead_code)]
const EXAMPLE_PART_2: Option<u32> = Some(71503);
#[allow(dead_code)]
const INPUT_PART_2: Option<u32> = Some(42550411);
#[allow(dead_code)]
const INPUT_PART_1: Option<u32> = Some(1195150);

#[allow(dead_code)]
const EXAMPLE_PART_1: Option<u32> = None;

#[allow(dead_code)]
const EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

#[allow(dead_code)]
const EXAMPLE_2: &str = r#"Time:      71530
Distance:  940200"#;

#[allow(dead_code)]
const INPUT: &str = r#"Time:        54     94     65     92
Distance:   302   1476   1029   1404"#;

#[allow(dead_code)]
const INPUT_2: &str = r#"Time: 54946592
Distance:   302147610291404"#;
