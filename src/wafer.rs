mod symbols;

use std::str::FromStr;

use pest::Parser as PestParser;
use pest::iterators::Pair;
use symbols::Symbols;

use crate::wasm::{Instruction, ValueType};

#[derive(pest_derive::Parser)]
#[grammar = "wafer.pest"]
struct Parser;

pub struct Wafer<'a>(Pair<'a, Rule>);

impl<'a> Wafer<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut parsed = Parser::parse(Rule::main, input).expect("failed to parse");

        Self(parsed.next().unwrap())
    }

    pub fn to_instructions(&self) -> Vec<Instruction> {
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

        let symbols = self.symbols();
        inner(self.0.clone(), &symbols)
    }

    fn symbols(&self) -> Symbols {
        self.0.clone().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::wafer::symbols::Symbol;
    use crate::wasm::{Instruction, ValueType};

    use super::Wafer;

    #[test]
    fn should_parse_numbers() {
        let wafer = Wafer::parse("123");
        assert_eq!(
            wafer.to_instructions(),
            vec![Instruction::ConstI32(123), Instruction::End]
        );
    }

    #[test]
    fn should_parse_symbols() {
        let wafer = Wafer::parse("let x = 1; let y = 2; 42");
        let symbols = wafer.symbols();

        assert_eq!(
            symbols.get("main", "x"),
            Some(Symbol::LocalVariable(ValueType::I32, 0))
        );

        assert_eq!(
            symbols.get("main", "y"),
            Some(Symbol::LocalVariable(ValueType::I32, 1))
        );
    }
}
