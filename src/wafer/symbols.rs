use std::collections::HashMap;

use itertools::Itertools;
use pest::iterators::Pair;

use crate::wasm::ValueType;

use super::Rule;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum SymbolKind {
    Parameter,
    LocalVariable,
}

pub struct Symbol {
    index: usize,
    r#type: ValueType,
    kind: SymbolKind,
}

pub struct Symbols(Vec<(String, HashMap<String, Symbol>)>);

fn param_symbols(pair: Pair<Rule>) -> impl Iterator<Item = (String, SymbolKind)> {
    pair.into_inner()
        .map(|p| (p.as_str().to_string(), SymbolKind::Parameter))
}

fn local_symbols(pair: Pair<Rule>) -> impl Iterator<Item = (String, SymbolKind)> {
    pair.into_inner()
        .flatten()
        .filter_map(|pair| match pair.as_rule() {
            Rule::let_statement => {
                let pair = pair.into_inner().next().unwrap();
                Some((pair.as_str().to_string(), SymbolKind::LocalVariable))
            }
            Rule::array_assignment_expression => {
                Some(("$temp".to_string(), SymbolKind::LocalVariable))
            }
            _ => None,
        })
        .unique()
}

fn function_symbols(pair: Pair<'_, Rule>) -> (String, HashMap<String, Symbol>) {
    let mut pairs = pair.into_inner();
    let name = pairs.next().unwrap();
    let params = pairs.next().unwrap();
    let body = pairs.next().unwrap();

    let symbols = param_symbols(params)
        .chain(local_symbols(body))
        .enumerate()
        .map(|(index, (name, kind))| {
            (
                name,
                Symbol {
                    index,
                    r#type: ValueType::I32,
                    kind,
                },
            )
        })
        .collect();

    (name.as_str().to_string(), symbols)
}

impl From<Pair<'_, Rule>> for Symbols {
    fn from(pair: Pair<Rule>) -> Self {
        let mut imports = vec![];
        let mut functions = vec![];

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::function => {
                    functions.push(function_symbols(pair));
                }
                Rule::public_function => {
                    functions.push(function_symbols(pair.into_inner().next().unwrap()));
                }
                Rule::external_function => {
                    let mut pairs = pair.into_inner();
                    let name = pairs.next().unwrap();
                    let params = pairs.next().unwrap();

                    let symbols = param_symbols(params)
                        .enumerate()
                        .map(|(index, (name, kind))| {
                            (
                                name,
                                Symbol {
                                    index,
                                    r#type: ValueType::I32,
                                    kind,
                                },
                            )
                        })
                        .collect();

                    imports.push((name.as_str().to_string(), symbols));
                }
                _ => (),
            }
        }

        let symbols = imports.into_iter().chain(functions).collect();
        Self(symbols)
    }
}

impl Symbols {
    pub fn local(&self, function_name: &str, local_name: &str) -> (ValueType, usize) {
        let symbol = self
            .symbols_for_function(function_name)
            .get(local_name)
            .expect("couldn't find symbol");

        (symbol.r#type, symbol.index)
    }

    pub fn locals(&self, function_name: &str) -> Vec<(usize, ValueType)> {
        let mut locals: HashMap<ValueType, usize> = HashMap::new();

        for symbol in self
            .symbols_for_function(function_name)
            .values()
            .filter(|symbol| symbol.kind == SymbolKind::LocalVariable)
        {
            *locals.entry(symbol.r#type).or_insert(0) += 1;
        }

        locals
            .into_iter()
            .map(|(r#type, count)| (count, r#type))
            .collect()
    }

    pub fn parameters(&self, function_name: &str) -> Vec<ValueType> {
        self.symbols_for_function(function_name)
            .values()
            .filter(|symbol| symbol.kind == SymbolKind::Parameter)
            .map(|symbol| symbol.r#type)
            .collect()
    }

    pub fn function(&self, function_name: &str) -> usize {
        self.0
            .iter()
            .position(|(name, _)| function_name == name)
            .expect("couldn't find function")
    }

    fn symbols_for_function(&self, function_name: &str) -> &HashMap<String, Symbol> {
        self.0
            .iter()
            .find(|(name, _)| name == function_name)
            .map(|(_, symbols)| symbols)
            .expect("couldn't find symbols")
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser as PestParser;

    use crate::wafer::{Parser, Rule};
    use crate::wasm::ValueType;

    use super::Symbols;

    const WAFER: &str = r"
        extern func import(a, b);
    
        func first(a) {
            if 0 {
                let x = 1;
            }
            while 0 {
                let y = 2;
            }
            a + 42
        }

        func second() {
            let y = 3;
            0
        }

        func third() {
            123
        }

        func fourth() {
            __mem[1] := 2;
            __mem[3] := 4;
            0
        }
    ";

    #[test]
    fn should_parse_symbols() {
        let pair = Parser::parse(Rule::module, WAFER).unwrap().next().unwrap();
        let symbols: Symbols = pair.into();

        assert_eq!(symbols.local("first", "a"), (ValueType::I32, 0));
        assert_eq!(symbols.local("first", "x"), (ValueType::I32, 1));
        assert_eq!(symbols.local("first", "y"), (ValueType::I32, 2));
        assert_eq!(symbols.local("second", "y"), (ValueType::I32, 0));
        assert_eq!(symbols.local("fourth", "$temp"), (ValueType::I32, 0));
    }

    #[test]
    fn should_get_locals() {
        let pair = Parser::parse(Rule::module, WAFER).unwrap().next().unwrap();
        let symbols: Symbols = pair.into();

        assert_eq!(symbols.locals("first"), vec![(2, ValueType::I32)]);
        assert_eq!(symbols.locals("second"), vec![(1, ValueType::I32)]);
        assert_eq!(symbols.locals("third"), vec![]);
        assert_eq!(symbols.locals("fourth"), vec![(1, ValueType::I32)]);
    }

    #[test]
    fn should_get_parameters() {
        let pair = Parser::parse(Rule::module, WAFER).unwrap().next().unwrap();
        let symbols: Symbols = pair.into();

        assert_eq!(
            symbols.parameters("import"),
            vec![ValueType::I32, ValueType::I32]
        );
        assert_eq!(symbols.parameters("first"), vec![ValueType::I32]);
        assert_eq!(symbols.parameters("second"), vec![]);
    }

    #[test]
    fn should_get_functions() {
        let pair = Parser::parse(Rule::module, WAFER).unwrap().next().unwrap();
        let symbols: Symbols = pair.into();

        assert_eq!(symbols.function("import"), 0);
        assert_eq!(symbols.function("first"), 1);
        assert_eq!(symbols.function("second"), 2);
        assert_eq!(symbols.function("third"), 3);
        assert_eq!(symbols.function("fourth"), 4);
    }
}
