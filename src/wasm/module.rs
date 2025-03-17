use super::section::{CodeSection, FunctionSection, TypeSection};
use super::{Instruction, WasmEncodable};

#[derive(Default)]
pub struct Module {
    r#type: TypeSection,
    function: FunctionSection,
    code: CodeSection,
}

const MAGIC: &[u8] = "\0asm".as_bytes();
const VERSION: &[u8] = &1u32.to_le_bytes();

impl WasmEncodable for Module {
    fn wasm_encode(&self) -> Vec<u8> {
        let mut result = vec![];

        result.extend(MAGIC);
        result.extend(VERSION);

        result.extend(self.r#type.wasm_encode());
        result.extend(self.function.wasm_encode());
        result.extend(self.code.wasm_encode());

        result
    }
}

impl Module {
    pub fn add_function(&mut self, instructions: Vec<Instruction>) {
        let r#type = self.r#type.add_function();
        self.function.add_function(r#type);
        self.code.add_function(instructions);
    }
}
