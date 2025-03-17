#[derive(pest_derive::Parser)]
#[grammar = "wafer.pest"]
pub struct Parser;

#[cfg(test)]
mod tests {
    use super::{Parser, Rule};
    use pest::Parser as PestParser;

    #[test]
    fn should_parse_nop() {
        Parser::parse(Rule::main, "").expect("failed to parse");
    }
}
