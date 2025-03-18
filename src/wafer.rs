mod symbols;

use std::str::FromStr;

use pest::Parser as PestParser;
use pest::iterators::Pair;
use symbols::{Symbol, Symbols};

use crate::wasm::{Instruction, ValueType};

#[derive(pest_derive::Parser)]
#[grammar = "wafer.pest"]
struct Parser;

pub struct Wafer {
    pub locals: Vec<ValueType>,
    pub instructions: Vec<Instruction>,
}

fn to_instructions(input: Pair<Rule>, symbols: &Symbols) -> Vec<Instruction> {
    fn inner(pair: Pair<Rule>, symbols: &Symbols, instructions: &mut Vec<Instruction>) {
        match pair.as_rule() {
            Rule::main => {
                for pair in pair.into_inner() {
                    inner(pair, symbols, instructions);
                }
            }
            Rule::let_statement => {
                let mut pairs = pair.into_inner();

                let identifier = pairs.next().unwrap().as_str();
                let symbol = symbols.get("main", identifier);

                let expression = pairs.next().unwrap();

                inner(expression, symbols, instructions);

                match symbol {
                    Symbol::LocalVariable(ValueType::I32, index) => {
                        instructions.push(Instruction::LocalSetI32(index));
                    }
                }
            }
            Rule::expression => {
                let mut pairs = pair.into_inner();
                inner(pairs.next().unwrap(), symbols, instructions);

                while let Some(operation) = pairs.next() {
                    let operand = pairs.next().unwrap();

                    inner(operand, symbols, instructions);
                    inner(operation, symbols, instructions);
                }
            }
            Rule::operation => instructions.push(match pair.as_str() {
                "+" => Instruction::AddI32,
                "-" => Instruction::SubtractI32,
                "*" => Instruction::MultiplyI32,
                "/" => Instruction::DivideSignedI32,
                _ => unreachable!(),
            }),
            Rule::identifier => {
                let symbol = symbols.get("main", pair.as_str());

                match symbol {
                    Symbol::LocalVariable(ValueType::I32, index) => {
                        instructions.push(Instruction::LocalGetI32(index));
                    }
                }
            }
            Rule::number => {
                let number = i32::from_str(pair.as_str()).expect("failed to parse number");
                instructions.push(Instruction::ConstI32(number));
            }
            Rule::EOI => {
                instructions.push(Instruction::End);
            }
            _ => unreachable!(),
        }
    }

    let mut instructions = vec![];
    inner(input, symbols, &mut instructions);

    instructions
}

impl Wafer {
    pub fn parse(input: &str) -> Self {
        let parsed = Parser::parse(Rule::main, input)
            .expect("failed to parse")
            .next()
            .unwrap();

        let symbols = Symbols::from(parsed.clone());
        let instructions = to_instructions(parsed, &symbols);

        Self {
            locals: symbols.locals("main"),
            instructions,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::{Instruction, ValueType};

    use super::Wafer;

    #[test]
    fn should_parse_numbers() {
        let wafer = Wafer::parse("123");
        assert_eq!(
            wafer.instructions,
            vec![Instruction::ConstI32(123), Instruction::End]
        );
    }

    #[test]
    fn should_handle_let_statement() {
        let wafer = Wafer::parse("let x = 42; x * 2");

        assert_eq!(wafer.locals, vec![ValueType::I32]);
        assert_eq!(
            wafer.instructions,
            vec![
                Instruction::ConstI32(42),
                Instruction::LocalSetI32(0),
                Instruction::LocalGetI32(0),
                Instruction::ConstI32(2),
                Instruction::MultiplyI32,
                Instruction::End
            ]
        );
    }
}
