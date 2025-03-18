mod symbols;

use std::str::FromStr;

use pest::Parser as PestParser;
use pest::iterators::Pair;
use symbols::Symbols;

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

    pub fn to_instructions(&self) -> Vec<Instruction> {
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

        inner(self.0.clone())
    }

    pub fn symbols(&self) -> Symbols {
        fn inner(pair: Pair<Rule>, symbols: &mut Symbols) {
            match pair.as_rule() {
                Rule::main => {
                    let pairs = pair.into_inner();

                    for pair in pairs {
                        inner(pair, symbols);
                    }
                }
                Rule::let_statement => {
                    let pair = pair.into_inner().next().unwrap();
                    inner(pair, symbols);
                }
                Rule::identifier => {
                    symbols.add("main", pair.as_str());
                }
                Rule::expression => (),
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        let mut symbols = Symbols::default();
        inner(self.0.clone(), &mut symbols);

        symbols
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
            wafer.to_instructions(),
            vec![Instruction::ConstI32(123), Instruction::End]
        );
    }

    #[test]
    fn should_parse_symbols() {
        let wafer = Wafer::parse("let x = 1; let y = 2; 42");
        let symbols = wafer.symbols();

        assert_eq!(symbols.get("main", "x"), Some(0));
        assert_eq!(symbols.get("main", "y"), Some(1));
    }
}
