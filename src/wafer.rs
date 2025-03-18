mod symbols;

use std::str::FromStr;

use pest::Parser as PestParser;
use pest::iterators::Pair;
use symbols::Symbols;

use crate::wasm::Instruction;

#[derive(pest_derive::Parser)]
#[grammar = "wafer.pest"]
struct Parser;

pub struct Wafer {
    pub instructions: Vec<Instruction>,
}

fn to_instructions(input: Pair<Rule>, symbols: &Symbols) -> Vec<Instruction> {
    fn inner(pair: Pair<Rule>, symbols: &Symbols) -> Vec<Instruction> {
        match pair.as_rule() {
            Rule::main => {
                let mut instructions = inner(pair.into_inner().next().unwrap(), symbols);
                instructions.push(Instruction::End);

                instructions
            }
            Rule::expression => {
                let mut pairs = pair.into_inner();
                let mut instructions = inner(pairs.next().unwrap(), symbols);

                while let Some(operation) = pairs.next() {
                    let operand = pairs.next().unwrap();

                    instructions.extend(inner(operand, symbols));
                    instructions.extend(inner(operation, symbols));
                }

                instructions
            }
            Rule::operation => match pair.as_str() {
                "+" => vec![Instruction::AddI32],
                "-" => vec![Instruction::SubtractI32],
                "*" => vec![Instruction::MultiplyI32],
                "/" => vec![Instruction::DivideSignedI32],
                _ => unreachable!(),
            },
            Rule::number => {
                let number = i32::from_str(pair.as_str()).expect("failed to parse number");
                vec![Instruction::ConstI32(number)]
            }
            _ => unreachable!(),
        }
    }

    inner(input, symbols)
}

impl Wafer {
    pub fn parse(input: &str) -> Self {
        let parsed = Parser::parse(Rule::main, input)
            .expect("failed to parse")
            .next()
            .unwrap();

        let symbols = Symbols::from(parsed.clone());
        let instructions = to_instructions(parsed, &symbols);

        Self { instructions }
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
            wafer.instructions,
            vec![Instruction::ConstI32(123), Instruction::End]
        );
    }
}
