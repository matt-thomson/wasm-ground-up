use super::section::{CodeSection, ExportSection, FunctionSection, ImportSection, TypeSection};
use super::{Instruction, ValueType, WasmEncodable};

#[derive(Default)]
pub struct Module {
    r#type: TypeSection,
    import: ImportSection,
    function: FunctionSection,
    export: ExportSection,
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
        result.extend(self.import.wasm_encode());
        result.extend(self.function.wasm_encode());
        result.extend(self.export.wasm_encode());
        result.extend(self.code.wasm_encode());

        result
    }
}

impl Module {
    pub fn add_import(&mut self, name: &str, parameters: Vec<ValueType>, returns: Vec<ValueType>) {
        let r#type = self.r#type.add_function(parameters, returns);
        self.import.add_function("waferImports", name, r#type);
    }

    pub fn add_function(
        &mut self,
        parameters: Vec<ValueType>,
        returns: Vec<ValueType>,
        locals: Vec<(usize, ValueType)>,
        instructions: Vec<Instruction>,
    ) -> usize {
        let r#type = self.r#type.add_function(parameters, returns);
        let index = self.function.add_function(r#type);
        self.code.add_function(locals, instructions);

        index
    }

    pub fn export_function(&mut self, name: &str, index: usize) {
        self.export.add_function(name, index);
    }
}
