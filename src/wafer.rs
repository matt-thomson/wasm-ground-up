mod symbols;

use std::str::FromStr;

use pest::Parser as PestParser;
use pest::iterators::Pair;
use symbols::Symbols;

use crate::wasm::{Instruction, ValueType};

#[derive(pest_derive::Parser)]
#[grammar = "wafer.pest"]
struct Parser;

pub struct Function {
    pub name: String,
    pub parameters: Vec<ValueType>,
    pub locals: Vec<(usize, ValueType)>,
    pub instructions: Vec<Instruction>,
}

pub struct Wafer {
    pub functions: Vec<Function>,
}

fn to_instructions(input: Pair<Rule>, name: &str, symbols: &Symbols) -> Vec<Instruction> {
    fn inner(pair: Pair<Rule>, name: &str, symbols: &Symbols, instructions: &mut Vec<Instruction>) {
        match pair.as_rule() {
            Rule::block_expression => {
                for pair in pair.into_inner() {
                    inner(pair, name, symbols, instructions);
                }

                instructions.push(Instruction::End);
            }
            Rule::let_statement => {
                let mut pairs = pair.into_inner();

                let identifier = pairs.next().unwrap().as_str();
                let (r#type, index) = symbols.local(name, identifier);

                let expression = pairs.next().unwrap();

                inner(expression, name, symbols, instructions);

                match r#type {
                    ValueType::I32 => {
                        instructions.push(Instruction::LocalSetI32(index));
                    }
                }
            }
            Rule::expression_statement => {
                let expression = pair.into_inner().next().unwrap();
                inner(expression, name, symbols, instructions);

                instructions.push(Instruction::Drop);
            }
            Rule::assignment_expression => {
                let mut pairs = pair.into_inner();

                let identifier = pairs.next().unwrap().as_str();
                let (r#type, index) = symbols.local(name, identifier);

                let expression = pairs.next().unwrap();

                inner(expression, name, symbols, instructions);

                match r#type {
                    ValueType::I32 => {
                        instructions.push(Instruction::LocalTeeI32(index));
                    }
                }
            }
            Rule::arithmetic_expression => {
                let mut pairs = pair.into_inner();
                inner(pairs.next().unwrap(), name, symbols, instructions);

                while let Some(operation) = pairs.next() {
                    let operand = pairs.next().unwrap();

                    inner(operand, name, symbols, instructions);
                    inner(operation, name, symbols, instructions);
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
                let (r#type, index) = symbols.local(name, pair.as_str());

                match r#type {
                    ValueType::I32 => {
                        instructions.push(Instruction::LocalGetI32(index));
                    }
                }
            }
            Rule::number => {
                let number = i32::from_str(pair.as_str()).expect("failed to parse number");
                instructions.push(Instruction::ConstI32(number));
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    let mut instructions = vec![];
    inner(input, name, symbols, &mut instructions);

    instructions
}

impl Wafer {
    pub fn parse(input: &str) -> Self {
        let parsed = Parser::parse(Rule::module, input)
            .expect("failed to parse")
            .next()
            .unwrap();

        let symbols = Symbols::from(parsed.clone());
        let mut functions = vec![];

        for pair in parsed.into_inner() {
            match pair.as_rule() {
                Rule::function => {
                    let mut pairs = pair.into_inner();
                    let name = pairs.next().unwrap().as_str();
                    let _params = pairs.next().unwrap();
                    let body = pairs.next().unwrap();

                    let instructions = to_instructions(body, name, &symbols);

                    functions.push(Function {
                        name: name.to_string(),
                        parameters: symbols.parameters(name),
                        locals: symbols.locals(name),
                        instructions,
                    });
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        Self { functions }
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::{Instruction, ValueType};

    use super::Wafer;

    #[test]
    fn should_parse_numbers() {
        let wafer = Wafer::parse("func number() { 123 }");
        assert_eq!(wafer.functions.len(), 1);

        let function = &wafer.functions[0];
        assert_eq!(function.name, "number");
        assert_eq!(
            function.instructions,
            vec![Instruction::ConstI32(123), Instruction::End]
        );
    }

    #[test]
    fn should_handle_let_statement() {
        let wafer = Wafer::parse("func letstmt() { let x = 42; x * 2 }");
        let function = &wafer.functions[0];

        assert_eq!(function.locals, vec![(1, ValueType::I32)]);
        assert_eq!(
            function.instructions,
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

    #[test]
    fn should_handle_expression_statement() {
        let wafer = Wafer::parse("func exprstmt() { let x = 1; x := 2; 3 }");
        let function = &wafer.functions[0];

        assert_eq!(
            function.instructions,
            vec![
                Instruction::ConstI32(1),
                Instruction::LocalSetI32(0),
                Instruction::ConstI32(2),
                Instruction::LocalTeeI32(0),
                Instruction::Drop,
                Instruction::ConstI32(3),
                Instruction::End
            ]
        )
    }

    #[test]
    fn should_handle_multiple_functions() {
        let wafer = Wafer::parse("func one() { 1 } func two() { 2 }");

        assert_eq!(wafer.functions.len(), 2);
        assert_eq!(wafer.functions[0].name, "one");
        assert_eq!(wafer.functions[1].name, "two");
    }

    #[test]
    fn should_handle_function_with_parameters() {
        let wafer = Wafer::parse("func withparams(x, y) { x + y }");
        let function = &wafer.functions[0];

        assert_eq!(function.parameters, vec![ValueType::I32, ValueType::I32]);
    }
}
