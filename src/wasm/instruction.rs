use super::{ValueType, WasmEncodable};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    If(Option<ValueType>),
    Else,
    End,
    Call(usize),
    Drop,
    ConstI32(i32),
    AddI32,
    SubtractI32,
    MultiplyI32,
    DivideSignedI32,
    LocalGetI32(usize),
    LocalSetI32(usize),
    LocalTeeI32(usize),
}

impl WasmEncodable for Instruction {
    fn wasm_encode(&self) -> Vec<u8> {
        match self {
            Instruction::If(r#type) => [
                vec![0x04],
                r#type.map(|t| t.wasm_encode()).unwrap_or(vec![0x40]),
            ]
            .concat(),
            Instruction::Else => vec![0x05],
            Instruction::End => vec![0x0b],
            Instruction::Call(index) => [vec![0x10], index.wasm_encode()].concat(),
            Instruction::Drop => vec![0x1a],
            Instruction::ConstI32(value) => [vec![0x41], value.wasm_encode()].concat(),
            Instruction::AddI32 => vec![0x6a],
            Instruction::SubtractI32 => vec![0x6b],
            Instruction::MultiplyI32 => vec![0x6c],
            Instruction::DivideSignedI32 => vec![0x6d],
            Instruction::LocalGetI32(index) => [vec![0x20], index.wasm_encode()].concat(),
            Instruction::LocalSetI32(index) => [vec![0x21], index.wasm_encode()].concat(),
            Instruction::LocalTeeI32(index) => [vec![0x22], index.wasm_encode()].concat(),
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
