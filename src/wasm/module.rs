use super::WasmEncodable;
use super::section::{CodeSection, FunctionSection, TypeSection};

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
