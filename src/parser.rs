#[derive(pest_derive::Parser)]
#[grammar = "wafer.pest"]
pub struct Parser;

#[cfg(test)]
mod tests {
    use super::{Parser, Rule};
    use pest::Parser as PestParser;

    #[test]
    fn should_parse_numbers() {
        Parser::parse(Rule::main, "123").expect("failed to parse 123");
        Parser::parse(Rule::main, "42").expect("failed to parse 42");
        Parser::parse(Rule::main, "abc").expect_err("failed to error when parsing abc");
    }
}
