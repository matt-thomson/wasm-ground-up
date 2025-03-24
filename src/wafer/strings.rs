use std::collections::HashMap;

use pest::iterators::Pair;

use super::Rule;

pub struct Strings {
    offsets: HashMap<String, usize>,
    data: Vec<u8>,
}

impl From<Pair<'_, Rule>> for Strings {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut offsets = HashMap::new();
        let mut data = vec![];

        for pair in pair
            .into_inner()
            .flatten()
            .filter(|pair| pair.as_rule() == Rule::string_literal)
        {
            let value = pair.as_str();
            offsets.insert(value.to_owned(), data.len());

            data.extend((value.len() as i32).to_le_bytes());
            data.extend(value.chars().flat_map(|c| (c as i32).to_le_bytes()));
        }

        Self { offsets, data }
    }
}

impl Strings {
    pub fn offset(&self, string: &str) -> i32 {
        self.offsets[string] as i32
    }

    pub fn len(&self) -> i32 {
        self.data.len() as i32
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser as PestParser;

    use crate::wafer::{Parser, Rule};

    use super::Strings;

    #[test]
    fn should_collect_strings() {
        let pair = Parser::parse(
            Rule::module,
            r#"func main() { let a = "foo"; let b = "bar"; 0 }"#,
        )
        .unwrap()
        .next()
        .unwrap();

        let strings = Strings::from(pair);

        assert_eq!(strings.offset("foo"), 0);
        assert_eq!(strings.offset("bar"), 16);
        assert_eq!(strings.len(), 32);

        let bytes = strings.into_bytes();

        assert_eq!(bytes[0], 3);
        assert_eq!(bytes[4], 0x66);
        assert_eq!(bytes[8], 0x6f);
        assert_eq!(bytes[12], 0x6f);

        assert_eq!(bytes[16], 3);
        assert_eq!(bytes[20], 0x62);
        assert_eq!(bytes[24], 0x61);
        assert_eq!(bytes[28], 0x72);
    }
}
