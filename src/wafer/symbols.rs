use std::collections::HashMap;

#[derive(Default)]
pub struct Symbols(HashMap<String, HashMap<String, usize>>);

impl Symbols {
    pub fn add(&mut self, function_name: &str, local_name: &str) {
        let symbols = self.0.entry(function_name.to_string()).or_default();
        symbols.insert(local_name.to_string(), symbols.len());
    }

    pub fn get(&self, function_name: &str, local_name: &str) -> Option<usize> {
        self.0
            .get(function_name)
            .and_then(|f| f.get(local_name))
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::Symbols;

    #[test]
    fn should_build_symbols() {
        let mut symbols = Symbols::default();

        symbols.add("main", "y");
        symbols.add("other", "x");
        symbols.add("main", "x");

        assert_eq!(symbols.get("main", "x"), Some(1));
        assert_eq!(symbols.get("main", "y"), Some(0));
        assert_eq!(symbols.get("other", "x"), Some(0));
        assert_eq!(symbols.get("other", "y"), None);
    }
}
