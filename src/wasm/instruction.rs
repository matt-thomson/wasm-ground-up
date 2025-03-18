use super::WasmEncodable;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    End,
    ConstI32(i32),
    AddI32,
    SubtractI32,
    MultiplyI32,
    DivideSignedI32,
    LocalSetI32(usize),
}

impl WasmEncodable for Instruction {
    fn wasm_encode(&self) -> Vec<u8> {
        match self {
            Instruction::End => vec![0x0b],
            Instruction::ConstI32(value) => [vec![0x41], value.wasm_encode()].concat(),
            Instruction::AddI32 => vec![0x6a],
            Instruction::SubtractI32 => vec![0x6b],
            Instruction::MultiplyI32 => vec![0x6c],
            Instruction::DivideSignedI32 => vec![0x6d],
            Instruction::LocalSetI32(index) => [vec![0x21], index.wasm_encode()].concat(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::WasmEncodable;

    use super::Instruction;

    #[test]
    fn should_encode_const_i32() {
        let instruction = Instruction::ConstI32(42);

        let wasm = instruction.wasm_encode();

        assert_eq!(wasm, vec![65, 42]);
    }
}
