use std::collections::HashMap;

use pest::iterators::Pair;

use crate::wasm::ValueType;

use super::Rule;

#[derive(Default)]
pub struct Symbols(HashMap<String, HashMap<String, Symbol>>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Symbol {
    LocalVariable(ValueType, usize),
}

impl From<Pair<'_, Rule>> for Symbols {
    fn from(value: Pair<Rule>) -> Self {
        let mut symbols = HashMap::default();

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

        inner(value, &mut symbols);

        Self(symbols)
    }
}

impl Symbols {
    pub fn add_local(&mut self, function_name: &str, local_name: &str, r#type: ValueType) {
        let symbols = self.0.entry(function_name.to_string()).or_default();
        symbols.insert(
            local_name.to_string(),
            Symbol::LocalVariable(r#type, symbols.len()),
        );
    }

    pub fn get(&self, function_name: &str, local_name: &str) -> Option<Symbol> {
        self.0
            .get(function_name)
            .and_then(|f| f.get(local_name))
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::ValueType;

    use super::{Symbol, Symbols};

    #[test]
    fn should_build_symbols() {
        let mut symbols = Symbols::default();

        symbols.add_local("main", "y", ValueType::I32);
        symbols.add_local("other", "x", ValueType::I32);
        symbols.add_local("main", "x", ValueType::I32);

        assert_eq!(
            symbols.get("main", "x"),
            Some(Symbol::LocalVariable(ValueType::I32, 1))
        );
        assert_eq!(
            symbols.get("main", "y"),
            Some(Symbol::LocalVariable(ValueType::I32, 0))
        );
        assert_eq!(
            symbols.get("other", "x"),
            Some(Symbol::LocalVariable(ValueType::I32, 0))
        );
        assert_eq!(symbols.get("other", "y"), None);
    }
}
