use std::str::FromStr;

use pest::Parser as PestParser;
use pest::iterators::Pair;

use crate::wasm::Instruction;

#[derive(pest_derive::Parser)]
#[grammar = "wafer.pest"]
struct Parser;

pub struct Wafer<'a>(Pair<'a, Rule>);

impl<'a> Wafer<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut parsed = Parser::parse(Rule::main, input).expect("failed to parse");

        Self(parsed.next().unwrap())
    }

    pub fn into_instructions(self) -> Vec<Instruction> {
        fn inner(pair: Pair<Rule>) -> Vec<Instruction> {
            match pair.as_rule() {
                Rule::main => {
                    let mut instructions = inner(pair.into_inner().next().unwrap());
                    instructions.push(Instruction::End);

                    instructions
                }
                Rule::expression => {
                    let mut pairs = pair.into_inner();
                    let mut instructions = inner(pairs.next().unwrap());

                    while let Some(operation) = pairs.next() {
                        let operand = pairs.next().unwrap();

                        instructions.extend(inner(operand));
                        instructions.extend(inner(operation));
                    }

                    instructions
                }
                Rule::operation => match pair.as_str() {
                    "+" => vec![Instruction::AddI32],
                    "-" => vec![Instruction::SubtractI32],
                    _ => unreachable!(),
                },
                Rule::number => {
                    let number = i32::from_str(pair.as_str()).expect("failed to parse number");
                    vec![Instruction::ConstI32(number)]
                }
                Rule::WHITESPACE => unreachable!(),
            }
        }

        inner(self.0)
    }
}

#[cfg(test)]
mod tests {

    use crate::wasm::Instruction;

    use super::Wafer;

    #[test]
    fn should_parse_numbers() {
        let wafer = Wafer::parse("123");
        assert_eq!(
            wafer.into_instructions(),
            vec![Instruction::ConstI32(123), Instruction::End]
        );
    }

    #[test]
    #[should_panic]
    fn should_fail_to_parse_non_numeric() {
        Wafer::parse("abc");
    }
}
