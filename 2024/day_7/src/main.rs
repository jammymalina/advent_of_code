use nom::{
    character::complete::{char, digit1, line_ending, space1},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};
use rayon::prelude::*;

struct Equation {
    result: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn evaluate(&self) -> bool {
        if self.operands.is_empty() {
            return false;
        }
        if self.operands.len() == 1 {
            return self.operands[0] == self.result;
        }

        Self::backtrack(self.result, self.operands[0], &self.operands[1..])
    }

    fn backtrack(result: u64, acc: u64, remaining_operands: &[u64]) -> bool {
        if remaining_operands.is_empty() {
            return acc == result;
        }

        let mut mul_result = false;
        let mut add_result = false;
        let mut concat_result = false;
        if acc * remaining_operands[0] <= result {
            mul_result = Self::backtrack(
                result,
                acc * remaining_operands[0],
                &remaining_operands[1..],
            );
        }
        if acc + remaining_operands[0] <= result {
            add_result = Self::backtrack(
                result,
                acc + remaining_operands[0],
                &remaining_operands[1..],
            );
        }
        let concatenated_number = acc
            * 10_u64.pow(remaining_operands[0].to_string().len().try_into().unwrap())
            + remaining_operands[0];
        if concatenated_number <= result {
            concat_result = Self::backtrack(result, concatenated_number, &remaining_operands[1..]);
        }

        mul_result || add_result || concat_result
    }
}

fn parse_integer(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    map(
        separated_pair(
            parse_integer,
            char(':'),
            many1(preceded(space1, parse_integer)),
        ),
        |(result, operands)| Equation { result, operands },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(line_ending, parse_equation)(input)
}

fn main() {
    let input = include_str!("input.txt");

    let (_, equations) = parse_input(input).unwrap();

    println!("Found equations {}", equations.len());

    let possible_equation_count: u64 = equations
        .into_par_iter()
        .filter(|eq| eq.evaluate())
        .map(|eq| eq.result)
        .sum();
    println!("Number of possible equations: {possible_equation_count}");
}
