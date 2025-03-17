use pest::Parser as PestParser;
use pest::iterators::Pairs;

#[derive(pest_derive::Parser)]
#[grammar = "wafer.pest"]
struct Parser;

pub struct Wafer<'a>(Pairs<'a, Rule>);

impl<'a> Wafer<'a> {
    pub fn parse(input: &'a str) -> Self {
        let parsed = Parser::parse(Rule::main, input).expect("failed to parse");

        Self(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::Wafer;

    #[test]
    fn should_parse_numbers() {
        Wafer::parse("123");
    }

    #[test]
    #[should_panic]
    fn should_fail_to_parse_non_numeric() {
        Wafer::parse("abc");
    }
}
