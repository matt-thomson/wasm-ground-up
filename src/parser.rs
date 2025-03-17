use std::str::FromStr;

use pest::Parser as PestParser;
use pest::iterators::Pair;

#[derive(pest_derive::Parser)]
#[grammar = "wafer.pest"]
struct Parser;

pub struct Wafer<'a>(Pair<'a, Rule>);

impl<'a> Wafer<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut parsed = Parser::parse(Rule::main, input).expect("failed to parse");

        Self(parsed.next().unwrap())
    }

    fn js_value(self) -> u32 {
        fn inner(pair: Pair<Rule>) -> u32 {
            match pair.as_rule() {
                Rule::main => inner(pair.into_inner().next().unwrap()),
                Rule::number => u32::from_str(pair.as_str()).expect("failed to parse number"),
                Rule::WHITESPACE => unreachable!(),
            }
        }

        inner(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Wafer;

    #[test]
    fn should_parse_numbers() {
        let wafer = Wafer::parse("123");
        assert_eq!(wafer.js_value(), 123);
    }

    #[test]
    #[should_panic]
    fn should_fail_to_parse_non_numeric() {
        Wafer::parse("abc");
    }
}
