use std::{cmp::Ordering, collections::HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

type PageUpdates = Vec<Vec<i32>>;

struct PageOrderingRules {
    rules: HashSet<(i32, i32)>,
}

impl PageOrderingRules {
    fn new(rules: Vec<(i32, i32)>) -> Self {
        Self {
            rules: rules.into_iter().collect(),
        }
    }

    fn check_page_update_order(&self, page_update: &[i32]) -> bool {
        page_update
            .windows(2)
            .all(|w| !self.rules.contains(&(w[1], w[0])))
    }

    fn sort_page_numbers(&self, a: i32, b: i32) -> Ordering {
        if self.rules.contains(&(a, b)) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

fn parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_pipe_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parse_integer, tag("|"), parse_integer)(input)
}

fn parse_integer_pairs(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(line_ending, parse_pipe_pair)(input)
}

fn parse_comma_list(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(char(','), parse_integer)(input)
}

fn parse_input(input: &str) -> IResult<&str, (PageOrderingRules, PageUpdates)> {
    let (input, pairs) = parse_integer_pairs(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, lists) = separated_list1(line_ending, parse_comma_list)(input)?;

    Ok((input, (PageOrderingRules::new(pairs), lists)))
}

fn main() {
    let input = include_str!("input.txt");

    let (_, (ordering_rules, page_updates)) = parse_input(input).unwrap();

    let middle_page_sum: i32 = page_updates
        .iter()
        .filter(|page_update| {
            page_update.len() >= 3 && ordering_rules.check_page_update_order(page_update)
        })
        .map(|page_update| page_update[page_update.len() / 2])
        .sum();

    println!("Sum of middles of correctly sorted pages: {middle_page_sum}");

    let incorrectly_sorted_pages: PageUpdates = page_updates
        .into_iter()
        .filter(|page_update| {
            page_update.len() >= 3 && !ordering_rules.check_page_update_order(page_update)
        })
        .collect();

    let mut middle_page_sum: i32 = 0;
    for mut page_update in incorrectly_sorted_pages {
        page_update.sort_by(|&a, &b| ordering_rules.sort_page_numbers(a, b));
        middle_page_sum += page_update[page_update.len() / 2];
    }

    println!(
        "Sum of middles of originally incorrectly sorted pages after sorting: {middle_page_sum}"
    );
}
