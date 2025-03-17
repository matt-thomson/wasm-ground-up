use super::WasmEncodable;

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
