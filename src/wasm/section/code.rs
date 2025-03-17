use crate::wasm::WasmEncodable;

use super::Section;

pub enum Instruction {
    End,
}

impl WasmEncodable for Instruction {
    fn wasm_encode(&self) -> Vec<u8> {
        match self {
            Instruction::End => vec![0x0b],
        }
    }
}

pub struct FunctionCode {
    locals: Vec<u32>,
    instructions: Vec<Instruction>,
}

impl WasmEncodable for FunctionCode {
    fn wasm_encode(&self) -> Vec<u8> {
        let locals = self.locals.wasm_encode();
        let instructions = self
            .instructions
            .iter()
            .flat_map(|i| i.wasm_encode())
            .collect::<Vec<_>>();

        [
            (locals.len() + instructions.len()).wasm_encode(),
            locals,
            instructions,
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl FunctionCode {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            locals: vec![],
            instructions,
        }
    }
}

pub struct CodeSection {
    functions: Vec<FunctionCode>,
}

impl Section for CodeSection {
    type Contents = Vec<FunctionCode>;

    const ID: u8 = 10;

    fn contents(&self) -> &Self::Contents {
        &self.functions
    }
}

impl CodeSection {
    pub fn new(functions: Vec<FunctionCode>) -> Self {
        Self { functions }
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::WasmEncodable;

    use super::{CodeSection, FunctionCode, Instruction};

    #[test]
    fn should_encode_code_section_for_nop_function() {
        let function = FunctionCode::new(vec![Instruction::End]);
        let section = CodeSection::new(vec![function]);

        let wasm = section.wasm_encode();

        assert_eq!(wasm, vec![10, 4, 1, 2, 0, 11]);
    }
}
