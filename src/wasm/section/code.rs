use crate::wasm::{Instruction, ValueType, WasmEncodable};

use super::Section;

pub struct FunctionCode {
    locals: Vec<(usize, ValueType)>,
    instructions: Vec<Instruction>,
}

impl WasmEncodable for FunctionCode {
    fn wasm_encode(&self) -> Vec<u8> {
        let locals = self.locals.wasm_encode();
        let instructions = self
            .instructions
            .iter()
            .flat_map(WasmEncodable::wasm_encode)
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
    pub fn new(locals: Vec<(usize, ValueType)>, instructions: Vec<Instruction>) -> Self {
        Self {
            locals,
            instructions,
        }
    }
}

#[derive(Default)]
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
    pub fn add_function(
        &mut self,
        locals: Vec<(usize, ValueType)>,
        instructions: Vec<Instruction>,
    ) {
        let function = FunctionCode::new(locals, instructions);
        self.functions.push(function);
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::{ValueType, WasmEncodable};

    use super::{CodeSection, Instruction};

    #[test]
    fn should_encode_code_section_for_nop_function() {
        let mut section = CodeSection::default();
        section.add_function(vec![], vec![Instruction::End]);

        let wasm = section.wasm_encode();

        assert_eq!(wasm, vec![10, 4, 1, 2, 0, 0x0b]);
    }

    #[test]
    fn should_encode_locals() {
        let mut section = CodeSection::default();
        section.add_function(vec![(1, ValueType::I32)], vec![Instruction::End]);

        let wasm = section.wasm_encode();

        assert_eq!(wasm, vec![10, 6, 1, 4, 1, 1, 0x7f, 0x0b]);
    }
}
