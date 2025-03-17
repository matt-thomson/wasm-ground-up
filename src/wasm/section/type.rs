use crate::wasm::WasmEncodable;

use super::Section;

pub enum ValueType {}

impl WasmEncodable for ValueType {
    fn wasm_encode(&self) -> Vec<u8> {
        todo!()
    }
}

pub struct FunctionType {
    parameters: Vec<ValueType>,
    returns: Vec<ValueType>,
}

impl WasmEncodable for FunctionType {
    fn wasm_encode(&self) -> Vec<u8> {
        let mut result = vec![0x60];

        result.extend(self.parameters.wasm_encode());
        result.extend(self.returns.wasm_encode());

        result
    }
}

impl FunctionType {
    pub fn new() -> Self {
        Self {
            parameters: vec![],
            returns: vec![],
        }
    }
}

#[derive(Default)]
pub struct TypeSection {
    functions: Vec<FunctionType>,
}

impl Section for TypeSection {
    type Contents = Vec<FunctionType>;

    const ID: u8 = 1;

    fn contents(&self) -> &Self::Contents {
        &self.functions
    }
}

impl TypeSection {
    pub fn new(functions: Vec<FunctionType>) -> Self {
        Self { functions }
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::WasmEncodable;

    use super::{FunctionType, TypeSection};

    #[test]
    fn should_encode_type_section_with_void_no_arg_function() {
        let function = FunctionType::new();
        let section = TypeSection::new(vec![function]);

        let wasm = section.wasm_encode();

        assert_eq!(wasm, vec![1, 4, 1, 0x60, 0, 0]);
    }
}
