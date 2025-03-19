use std::collections::HashMap;

use pest::iterators::Pair;

use crate::wasm::ValueType;

use super::Rule;

#[derive(PartialEq)]
pub enum SymbolKind {
    Parameter,
    LocalVariable,
}

pub struct Symbol {
    index: usize,
    r#type: ValueType,
    kind: SymbolKind,
}

pub struct Symbols(HashMap<String, HashMap<String, Symbol>>);

fn param_symbols(pair: Pair<Rule>) -> impl Iterator<Item = (String, SymbolKind)> {
    pair.into_inner()
        .map(|p| (p.as_str().to_string(), SymbolKind::Parameter))
}

fn local_symbols(pair: Pair<Rule>) -> impl Iterator<Item = (String, SymbolKind)> {
    pair.into_inner()
        .filter(|pair| pair.as_rule() == Rule::let_statement)
        .map(|pair| {
            let pair = pair.into_inner().next().unwrap();
            (pair.as_str().to_string(), SymbolKind::LocalVariable)
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
            })
            .collect();

        Self(symbols)
    }
}

impl Symbols {
    pub fn get(&self, function_name: &str, local_name: &str) -> (ValueType, usize) {
        let symbol = self
            .0
            .get(function_name)
            .and_then(|f| f.get(local_name))
            .expect("couldn't find symbol");

        (symbol.r#type, symbol.index)
    }

    pub fn locals(&self, function_name: &str) -> Vec<(usize, ValueType)> {
        let Some(symbols) = self.0.get(function_name) else {
            return vec![];
        };

        let mut locals: HashMap<ValueType, usize> = HashMap::new();

        for symbol in symbols
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
        self.0
            .get(function_name)
            .expect("couldn't find symbols")
            .values()
            .filter(|symbol| symbol.kind == SymbolKind::Parameter)
            .map(|symbol| symbol.r#type)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser as PestParser;

    use crate::wafer::{Parser, Rule};
    use crate::wasm::ValueType;

    use super::Symbols;

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

        assert_eq!(symbols.get("first", "a"), (ValueType::I32, 0));
        assert_eq!(symbols.get("first", "x"), (ValueType::I32, 1));
        assert_eq!(symbols.get("first", "y"), (ValueType::I32, 2));
        assert_eq!(symbols.get("second", "y"), (ValueType::I32, 0))
    }

    #[test]
    fn should_get_locals() {
        let pair = Parser::parse(Rule::module, WAFER).unwrap().next().unwrap();
        let symbols: Symbols = pair.into();

        assert_eq!(symbols.locals("first"), vec![(2, ValueType::I32)]);
        assert_eq!(symbols.locals("second"), vec![(1, ValueType::I32)]);
        assert_eq!(symbols.locals("third"), vec![]);
    }

    #[test]
    fn should_get_parameters() {
        let pair = Parser::parse(Rule::module, WAFER).unwrap().next().unwrap();
        let symbols: Symbols = pair.into();

        assert_eq!(symbols.parameters("first"), vec![ValueType::I32]);
        assert_eq!(symbols.parameters("second"), vec![]);
    }
}
