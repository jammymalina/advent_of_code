use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    combinator::map_res,
    error::{ErrorKind, ParseError},
    multi::fold_many0,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
enum Expr {
    Noop,
    Mul(i32, i32),
    Do,
    Dont,
}

struct Calculator {
    enabled_default: bool,
}

impl Calculator {
    const fn new() -> Self {
        Self {
            enabled_default: true,
        }
    }

    fn exec(&self, expressions: &[Expr]) -> i32 {
        let mut enabled = self.enabled_default;

        expressions
            .iter()
            .map(|expr| match expr {
                Expr::Do => {
                    enabled = true;
                    0
                }
                Expr::Dont => {
                    enabled = false;
                    0
                }
                Expr::Mul(x, y) => {
                    if enabled {
                        x * y
                    } else {
                        0
                    }
                }
                Expr::Noop => 0,
            })
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct EndOfParsing;

impl<I> ParseError<I> for EndOfParsing {
    fn from_error_kind(_input: I, _kind: ErrorKind) -> Self {
        Self
    }

    fn append(_: I, _: ErrorKind, _: Self) -> Self {
        Self
    }
}

fn parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}

fn parse_mul_expr(input: &str) -> IResult<&str, Expr> {
    let parser = tuple((
        tag("mul("),
        parse_integer,
        tag(","),
        parse_integer,
        tag(")"),
    ));

    map(parser, |(_, x, _, y, _)| Expr::Mul(x, y))(input)
}

fn parse_do(input: &str) -> IResult<&str, Expr> {
    map(tag("do()"), |_| Expr::Do)(input)
}

fn parse_dont(input: &str) -> IResult<&str, Expr> {
    map(tag("don't()"), |_| Expr::Dont)(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_mul_expr, parse_do, parse_dont))(input)
}

fn parse_input(input: &str) -> Vec<Expr> {
    let (_remaining, results) = fold_many0(
        |input| -> IResult<&str, Expr, EndOfParsing> {
            if let Ok((rest, expr)) = parse_expr(input) {
                Ok((rest, expr))
            } else {
                if input.is_empty() {
                    return Err(nom::Err::Error(EndOfParsing));
                }
                Ok((&input[1..], Expr::Noop)) // Skip one character
            }
        },
        Vec::new,
        |mut acc, item| {
            if !matches!(item, Expr::Noop) {
                acc.push(item);
            }
            acc
        },
    )(input)
    .unwrap_or((input, vec![]));

    results
}

fn main() {
    let input = include_str!("input.txt");

    let expressions = parse_input(input);
    let calc = Calculator::new();
    let result = calc.exec(&expressions);

    println!("Sum of all multiplications: {result}");
}
