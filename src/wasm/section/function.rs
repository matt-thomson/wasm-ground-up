use super::Section;

#[derive(Default)]
pub struct FunctionSection {
    types: Vec<u32>,
}

impl Section for FunctionSection {
    type Contents = Vec<u32>;

    const ID: u8 = 3;

    fn contents(&self) -> &Self::Contents {
        &self.types
    }
}

impl FunctionSection {
    pub fn new(types: Vec<u32>) -> Self {
        Self { types }
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::WasmEncodable;

    use super::FunctionSection;

    #[test]
    fn should_encode_function_section_with_single_type_index() {
        let section = FunctionSection::new(vec![0]);

        let wasm = section.wasm_encode();

        assert_eq!(wasm, vec![3, 2, 1, 0]);
    }
}
