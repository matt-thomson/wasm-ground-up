mod strings;
mod symbols;

use std::str::FromStr;

use pest::Parser as PestParser;
use pest::iterators::Pair;
use strings::Strings;
use symbols::Symbols;

use crate::wasm::{Instruction, ValueType};

#[derive(pest_derive::Parser)]
#[grammar = "src/wafer.pest"]
struct Parser;

pub struct Import {
    pub name: String,
    pub parameters: Vec<ValueType>,
}

pub struct Function {
    pub name: String,
    pub public: bool,
    pub parameters: Vec<ValueType>,
    pub locals: Vec<(usize, ValueType)>,
    pub instructions: Vec<Instruction>,
}

pub struct Wafer {
    pub imports: Vec<Import>,
    pub functions: Vec<Function>,
    pub data: Vec<u8>,
}

struct InstructionCollector<'a> {
    name: &'a str,
    symbols: &'a Symbols,
    strings: &'a Strings,
    instructions: Vec<Instruction>,
}

impl<'a> InstructionCollector<'a> {
    fn new(name: &'a str, symbols: &'a Symbols, strings: &'a Strings) -> Self {
        Self {
            name,
            symbols,
            strings,
            instructions: vec![],
        }
    }

    fn collect_inner(&mut self, pair: Pair<Rule>) {
        match pair.as_rule() {
            Rule::block_expression | Rule::block_statements => {
                for pair in pair.into_inner() {
                    self.collect_inner(pair);
                }
            }
            Rule::let_statement => {
                let mut pairs = pair.into_inner();

                let identifier = pairs.next().unwrap().as_str();
                let (r#type, index) = self.symbols.local(self.name, identifier);

                let expression = pairs.next().unwrap();

                self.collect_inner(expression);

                match r#type {
                    ValueType::I32 => {
                        self.instructions.push(Instruction::LocalSetI32(index));
                    }
                }
            }
            Rule::if_statement => {
                let mut pairs = pair.into_inner();

                let condition = pairs.next().unwrap();
                self.collect_inner(condition);
                self.instructions.push(Instruction::If(None));

                let then_block = pairs.next().unwrap();
                self.collect_inner(then_block);

                if let Some(else_block) = pairs.next() {
                    self.instructions.push(Instruction::Else);
                    self.collect_inner(else_block);
                }

                self.instructions.push(Instruction::End);
            }
            Rule::while_statement => {
                self.instructions.push(Instruction::Loop(None));

                let mut pairs = pair.into_inner();

                let condition = pairs.next().unwrap();
                self.collect_inner(condition);

                self.instructions.push(Instruction::If(None));

                let body = pairs.next().unwrap();
                self.collect_inner(body);

                self.instructions.push(Instruction::Break(1));
                self.instructions.push(Instruction::End);
                self.instructions.push(Instruction::End);
            }
            Rule::expression_statement => {
                let expression = pair.into_inner().next().unwrap();
                self.collect_inner(expression);

                self.instructions.push(Instruction::Drop);
            }
            Rule::variable_assignment_expression => {
                let mut pairs = pair.into_inner();

                let identifier = pairs.next().unwrap().as_str();
                let (r#type, index) = self.symbols.local(self.name, identifier);

                let expression = pairs.next().unwrap();

                self.collect_inner(expression);

                match r#type {
                    ValueType::I32 => {
                        self.instructions.push(Instruction::LocalTeeI32(index));
                    }
                }
            }
            Rule::array_assignment_expression => {
                let mut pairs = pair.into_inner();

                let mut array = pairs.next().unwrap().into_inner();
                let identifier = array.next().unwrap().as_str();
                let index = array.next().unwrap();
                let expression = pairs.next().unwrap();

                if identifier == "__mem" {
                    self.collect_inner(index);
                    self.collect_inner(expression);

                    let (r#type, temp_index) = self.symbols.local(self.name, "$temp");

                    match r#type {
                        ValueType::I32 => {
                            self.instructions.push(Instruction::LocalTeeI32(temp_index));
                        }
                    }

                    self.instructions.push(Instruction::StoreI32(2, 0));

                    match r#type {
                        ValueType::I32 => {
                            self.instructions.push(Instruction::LocalGetI32(temp_index));
                        }
                    }
                } else {
                    let (r#type, ident_index) = self.symbols.local(self.name, identifier);

                    match r#type {
                        ValueType::I32 => {
                            self.instructions
                                .push(Instruction::LocalGetI32(ident_index));
                        }
                    }

                    self.collect_inner(index);
                    self.collect_inner(expression);

                    let function_index = self.symbols.function("__writeInt32Array");
                    self.instructions.push(Instruction::Call(function_index));
                }
            }
            Rule::binary_expression => {
                let mut pairs = pair.into_inner();
                self.collect_inner(pairs.next().unwrap());

                while let Some(operation) = pairs.next() {
                    let operand = pairs.next().unwrap();

                    self.collect_inner(operand);
                    self.collect_inner(operation);
                }
            }
            Rule::call_expression => {
                let mut pairs = pair.into_inner();

                let identifier = pairs.next().unwrap().as_str();

                if identifier == "__trap" {
                    self.instructions.push(Instruction::Unreachable);
                } else {
                    let index = self.symbols.function(identifier);

                    let args = pairs.next().unwrap();

                    for expression in args.into_inner() {
                        self.collect_inner(expression);
                    }

                    self.instructions.push(Instruction::Call(index));
                }
            }
            Rule::if_expression => {
                let mut pairs = pair.into_inner();

                let condition = pairs.next().unwrap();
                self.collect_inner(condition);

                self.instructions
                    .push(Instruction::If(Some(ValueType::I32)));

                let then_block = pairs.next().unwrap();
                self.collect_inner(then_block);

                self.instructions.push(Instruction::Else);

                let else_block = pairs.next().unwrap();
                self.collect_inner(else_block);

                self.instructions.push(Instruction::End);
            }
            Rule::binary_operation => self.instructions.push(match pair.as_str() {
                "+" => Instruction::AddI32,
                "-" => Instruction::SubtractI32,
                "*" => Instruction::MultiplyI32,
                "/" => Instruction::DivideSignedI32,
                "==" => Instruction::EqualI32,
                "!=" => Instruction::NotEqualI32,
                "<=" => Instruction::LessThanOrEqualSignedI32,
                "<" => Instruction::LessThanSignedI32,
                ">=" => Instruction::GreaterThanOrEqualSignedI32,
                ">" => Instruction::GreaterThanSignedI32,
                "and" => Instruction::AndI32,
                "or" => Instruction::OrI32,
                _ => unreachable!(),
            }),
            Rule::array_index => {
                let mut pairs = pair.into_inner();

                let identifier = pairs.next().unwrap().as_str();
                let index = pairs.next().unwrap();

                if identifier == "__mem" {
                    self.collect_inner(index);

                    self.instructions.push(Instruction::LoadI32(2, 0));
                } else {
                    let (r#type, ident_index) = self.symbols.local(self.name, identifier);

                    match r#type {
                        ValueType::I32 => {
                            self.instructions
                                .push(Instruction::LocalGetI32(ident_index));
                        }
                    }

                    self.collect_inner(index);

                    let function_index = self.symbols.function("__readInt32Array");
                    self.instructions.push(Instruction::Call(function_index));
                }
            }
            Rule::identifier => {
                let identifier = pair.as_str();

                if identifier == "__heap_base" {
                    self.instructions
                        .push(Instruction::ConstI32(self.strings.len()))
                } else {
                    let (r#type, index) = self.symbols.local(self.name, identifier);

                    match r#type {
                        ValueType::I32 => {
                            self.instructions.push(Instruction::LocalGetI32(index));
                        }
                    }
                }
            }
            Rule::number => {
                let number = i32::from_str(pair.as_str()).expect("failed to parse number");
                self.instructions.push(Instruction::ConstI32(number));
            }
            Rule::string_literal => {
                let value = pair.as_str();
                let offset = self.strings.offset(value);

                self.instructions.push(Instruction::ConstI32(offset));
            }
            Rule::EOI => (),
            _ => unreachable!("{:#?}", pair),
        }
    }

    fn collect(&mut self, pair: Pair<Rule>) {
        self.collect_inner(pair);
        self.instructions.push(Instruction::End);
    }
}

fn parse_function(
    pair: Pair<Rule>,
    public: bool,
    symbols: &Symbols,
    strings: &Strings,
) -> Function {
    let mut pairs = pair.into_inner();
    let name = pairs.next().unwrap().as_str();
    let _params = pairs.next().unwrap();
    let body = pairs.next().unwrap();

    let mut collector = InstructionCollector::new(name, &symbols, &strings);
    collector.collect(body);

    Function {
        name: name.to_string(),
        public,
        parameters: symbols.parameters(name),
        locals: symbols.locals(name),
        instructions: collector.instructions,
    }
}

impl Wafer {
    pub fn parse(input: &str) -> Self {
        let parsed = Parser::parse(Rule::module, input)
            .expect("failed to parse")
            .next()
            .unwrap();

        let symbols = Symbols::from(parsed.clone());
        let strings = Strings::from(parsed.clone());

        let mut imports = vec![];
        let mut functions = vec![];

        for pair in parsed.into_inner() {
            match pair.as_rule() {
                Rule::external_function => {
                    let name = pair.into_inner().next().unwrap().as_str();

                    imports.push(Import {
                        name: name.to_string(),
                        parameters: symbols.parameters(name),
                    });
                }
                Rule::public_function => {
                    functions.push(parse_function(
                        pair.into_inner().next().unwrap(),
                        true,
                        &symbols,
                        &strings,
                    ));
                }
                Rule::function => {
                    functions.push(parse_function(pair, false, &symbols, &strings));
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        let data = strings.into_bytes();

        Self {
            imports,
            functions,
            data,
        }
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
        );
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

    #[test]
    fn should_handle_function_call() {
        let wafer = Wafer::parse("func one() { 1 } func caller() { one() + 2 }");
        let function = &wafer.functions[1];

        assert_eq!(
            function.instructions,
            vec![
                Instruction::Call(0),
                Instruction::ConstI32(2),
                Instruction::AddI32,
                Instruction::End
            ]
        );
    }

    #[test]
    fn should_handle_function_call_with_parameters() {
        let wafer = Wafer::parse("func add(x, y) { x + y } func caller() { add(3, 4 + 5) }");
        let function = &wafer.functions[1];

        assert_eq!(
            function.instructions,
            vec![
                Instruction::ConstI32(3),
                Instruction::ConstI32(4),
                Instruction::ConstI32(5),
                Instruction::AddI32,
                Instruction::Call(0),
                Instruction::End
            ]
        );
    }

    #[test]
    fn should_handle_if_expression() {
        let wafer = Wafer::parse("func iffy() { if 0 { 1 } else { 2 } }");
        let function = &wafer.functions[0];

        assert_eq!(
            function.instructions,
            vec![
                Instruction::ConstI32(0),
                Instruction::If(Some(ValueType::I32)),
                Instruction::ConstI32(1),
                Instruction::Else,
                Instruction::ConstI32(2),
                Instruction::End,
                Instruction::End
            ]
        );
    }

    #[test]
    fn should_handle_if_statement() {
        let wafer = Wafer::parse("func iffy() { if 0 { 1; } if 2 { 3; } else { 4; } 5 }");
        let function = &wafer.functions[0];

        assert_eq!(
            function.instructions,
            vec![
                Instruction::ConstI32(0),
                Instruction::If(None),
                Instruction::ConstI32(1),
                Instruction::Drop,
                Instruction::End,
                Instruction::ConstI32(2),
                Instruction::If(None),
                Instruction::ConstI32(3),
                Instruction::Drop,
                Instruction::Else,
                Instruction::ConstI32(4),
                Instruction::Drop,
                Instruction::End,
                Instruction::ConstI32(5),
                Instruction::End
            ]
        );
    }

    #[test]
    fn should_handle_while() {
        let wafer = Wafer::parse("func until() { while 0 { 1; } 2 }");
        let function = &wafer.functions[0];

        assert_eq!(
            function.instructions,
            vec![
                Instruction::Loop(None),
                Instruction::ConstI32(0),
                Instruction::If(None),
                Instruction::ConstI32(1),
                Instruction::Drop,
                Instruction::Break(1),
                Instruction::End,
                Instruction::End,
                Instruction::ConstI32(2),
                Instruction::End,
            ]
        );
    }

    #[test]
    fn should_handle_imports() {
        let wafer = Wafer::parse("extern func add(a, b);");
        let import = &wafer.imports[0];

        assert_eq!(import.name, "add");
        assert_eq!(import.parameters, vec![ValueType::I32, ValueType::I32]);
    }

    #[test]
    fn should_handle_memory_operations() {
        let wafer = Wafer::parse("func memory() { __mem[1] := 2; __mem[3] }");
        let function = &wafer.functions[0];

        assert_eq!(
            function.instructions,
            vec![
                Instruction::ConstI32(1),
                Instruction::ConstI32(2),
                Instruction::LocalTeeI32(0),
                Instruction::StoreI32(2, 0),
                Instruction::LocalGetI32(0),
                Instruction::Drop,
                Instruction::ConstI32(3),
                Instruction::LoadI32(2, 0),
                Instruction::End
            ]
        );
    }

    #[test]
    fn should_handle_array_operations() {
        let wafer = Wafer::parse(
            r"
                func __writeInt32Array() {
                    0
                }

                func __readInt32Array() {
                    0
                }

                func array() {
                    let x = 0;
                    x[1] := 2;
                    x[3]
                }",
        );
        let function = &wafer.functions[2];

        assert_eq!(
            function.instructions,
            vec![
                Instruction::ConstI32(0),
                Instruction::LocalSetI32(0),
                Instruction::LocalGetI32(0),
                Instruction::ConstI32(1),
                Instruction::ConstI32(2),
                Instruction::Call(0),
                Instruction::Drop,
                Instruction::LocalGetI32(0),
                Instruction::ConstI32(3),
                Instruction::Call(1),
                Instruction::End
            ]
        );
    }

    #[test]
    fn should_handle_strings() {
        let wafer = Wafer::parse(
            r#"
            func newInt32Array() {
                0
            }
                
            func string() {
                let a = "hello";
                let b = "world";
                0
            }
        "#,
        );
        let function = &wafer.functions[1];

        assert_eq!(
            function.instructions,
            vec![
                Instruction::ConstI32(0),
                Instruction::LocalSetI32(0),
                Instruction::ConstI32(24),
                Instruction::LocalSetI32(1),
                Instruction::ConstI32(0),
                Instruction::End
            ]
        );
    }

    #[test]
    fn should_handle_public_functions() {
        let wafer = Wafer::parse(
            r"
                func a() { 0 }
                public func b() { 0 }
            ",
        );

        assert_eq!(wafer.functions[0].public, false);
        assert_eq!(wafer.functions[1].public, true);
    }
}
