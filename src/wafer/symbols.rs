use std::collections::HashMap;

use pest::iterators::Pair;

use crate::wasm::ValueType;

use super::Rule;

pub struct Symbols(HashMap<String, HashMap<String, Symbol>>);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Symbol {
    LocalVariable(ValueType, usize),
}

fn param_symbols(pair: Pair<Rule>) -> impl Iterator<Item = String> {
    pair.into_inner().map(|p| p.as_str().to_string())
}

fn local_symbols(pair: Pair<Rule>) -> impl Iterator<Item = String> {
    pair.into_inner()
        .filter(|pair| pair.as_rule() == Rule::let_statement)
        .map(|pair| {
            let pair = pair.into_inner().next().unwrap();
            pair.as_str().to_string()
        })
}

impl From<Pair<'_, Rule>> for Symbols {
    fn from(pair: Pair<Rule>) -> Self {
        let symbols = pair
            .into_inner()
            .filter(|pair| pair.as_rule() == Rule::function)
            .map(|pair| {
                let mut pairs = pair.into_inner();
                let name = pairs.next().unwrap();
                let params = pairs.next().unwrap();
                let body = pairs.next().unwrap();

                let symbols = param_symbols(params)
                    .chain(local_symbols(body))
                    .enumerate()
                    .map(|(index, name)| (name, Symbol::LocalVariable(ValueType::I32, index)))
                    .collect();

                (name.as_str().to_string(), symbols)
            })
            .collect();

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

    pub fn locals(&self, function_name: &str) -> Vec<(usize, ValueType)> {
        let Some(symbols) = self.0.get(function_name) else {
            return vec![];
        };

        let mut locals: HashMap<ValueType, usize> = HashMap::new();

        for symbol in symbols.values() {
            match symbol {
                Symbol::LocalVariable(r#type, _) => {
                    *locals.entry(*r#type).or_insert(0) += 1;
                }
            }
        }

        locals
            .into_iter()
            .map(|(r#type, count)| (count, r#type))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser as PestParser;

    use crate::wafer::{Parser, Rule};
    use crate::wasm::ValueType;

    use super::{Symbol, Symbols};

    const WAFER: &str = r#"
       func first(a) {
           let x = 1;
           let y = 2;
           a + 42
       }

       func second() {
           let y = 3;
           0
       }

       func third() {
           123
       }
    "#;

    #[test]
    fn should_parse_symbols() {
        let pair = Parser::parse(Rule::module, WAFER).unwrap().next().unwrap();
        let symbols: Symbols = pair.into();

        assert_eq!(
            symbols.get("first", "a"),
            Symbol::LocalVariable(ValueType::I32, 0)
        );

        assert_eq!(
            symbols.get("first", "x"),
            Symbol::LocalVariable(ValueType::I32, 1)
        );

        assert_eq!(
            symbols.get("first", "y"),
            Symbol::LocalVariable(ValueType::I32, 2)
        );

        assert_eq!(
            symbols.get("second", "y"),
            Symbol::LocalVariable(ValueType::I32, 0)
        )
    }

    #[test]
    fn should_get_locals() {
        let pair = Parser::parse(Rule::module, WAFER).unwrap().next().unwrap();
        let symbols: Symbols = pair.into();

        assert_eq!(symbols.locals("first"), vec![(3, ValueType::I32)]);
        assert_eq!(symbols.locals("second"), vec![(1, ValueType::I32)]);
        assert_eq!(symbols.locals("third"), vec![]);
    }
}
