use std::collections::HashMap;

use pest::iterators::Pair;

use crate::wasm::ValueType;

use super::Rule;

pub struct Symbols(HashMap<String, HashMap<String, Symbol>>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Symbol {
    LocalVariable(ValueType, usize),
}

impl From<Pair<'_, Rule>> for Symbols {
    fn from(value: Pair<Rule>) -> Self {
        fn inner(pair: Pair<Rule>, symbols: &mut HashMap<String, HashMap<String, Symbol>>) {
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
                    let symbols = symbols.entry("main".to_string()).or_default();
                    symbols.insert(
                        pair.as_str().to_string(),
                        Symbol::LocalVariable(ValueType::I32, symbols.len()),
                    );
                }
                Rule::expression | Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        let mut symbols = HashMap::default();
        inner(value, &mut symbols);

        Self(symbols)
    }
}

impl Symbols {
    pub fn get(&self, function_name: &str, local_name: &str) -> Symbol {
        self.0
            .get(function_name)
            .and_then(|f| f.get(local_name))
            .copied()
            .expect("couldn't find symbol")
    }

    pub fn locals(&self, function_name: &str) -> Vec<ValueType> {
        let Some(symbols) = self.0.get(function_name) else {
            return vec![];
        };

        symbols
            .values()
            .map(|value| match value {
                Symbol::LocalVariable(value_type, _) => value_type,
            })
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser as PestParser;

    use crate::wafer::{Parser, Rule};
    use crate::wasm::ValueType;

    use super::{Symbol, Symbols};

    #[test]
    fn should_parse_symbols() {
        let pair = Parser::parse(Rule::main, "let x = 1; let y = 2; 42")
            .unwrap()
            .next()
            .unwrap();
        let symbols: Symbols = pair.into();

        assert_eq!(
            symbols.get("main", "x"),
            Symbol::LocalVariable(ValueType::I32, 0)
        );

        assert_eq!(
            symbols.get("main", "y"),
            Symbol::LocalVariable(ValueType::I32, 1)
        );
    }

    #[test]
    fn should_get_locals() {
        let pair = Parser::parse(Rule::main, "let x = 1; let y = 2; 42")
            .unwrap()
            .next()
            .unwrap();
        let symbols: Symbols = pair.into();

        assert_eq!(symbols.locals("main"), vec![ValueType::I32, ValueType::I32]);
        assert_eq!(symbols.locals("other"), vec![]);
    }
}
