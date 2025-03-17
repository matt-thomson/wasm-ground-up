use crate::wasm::WasmEncodable;

use super::Section;

#[derive(PartialEq)]
pub enum ValueType {}

impl WasmEncodable for ValueType {
    fn wasm_encode(&self) -> Vec<u8> {
        todo!()
    }
}

#[derive(PartialEq)]
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
    pub fn add_function(&mut self) -> u32 {
        let function = FunctionType::new();
        let index = self.functions.iter().position(|f| *f == function);

        if let Some(index) = index {
            index as u32
        } else {
            self.functions.push(function);
            (self.functions.len() - 1) as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::WasmEncodable;

    use super::TypeSection;

    #[test]
    fn should_encode_type_section_with_void_no_arg_function() {
        let mut section = TypeSection::default();
        section.add_function();

        let wasm = section.wasm_encode();

        assert_eq!(wasm, vec![1, 4, 1, 0x60, 0, 0]);
    }
}
