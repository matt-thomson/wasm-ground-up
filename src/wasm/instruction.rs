use super::{ValueType, WasmEncodable};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Loop(Option<ValueType>),
    If(Option<ValueType>),
    Else,
    End,
    Break(usize),
    Call(usize),
    Drop,
    LocalGetI32(usize),
    LocalSetI32(usize),
    LocalTeeI32(usize),
    ConstI32(i32),
    EqualI32,
    NotEqualI32,
    LessThanSignedI32,
    GreaterThanSignedI32,
    LessThanOrEqualSignedI32,
    GreaterThanOrEqualSignedI32,
    AddI32,
    SubtractI32,
    MultiplyI32,
    DivideSignedI32,
    AndI32,
    OrI32,
}

impl WasmEncodable for Instruction {
    fn wasm_encode(&self) -> Vec<u8> {
        match self {
            Instruction::Loop(r#type) => [
                vec![0x03],
                r#type.map(|t| t.wasm_encode()).unwrap_or(vec![0x40]),
            ]
            .concat(),
            Instruction::If(r#type) => [
                vec![0x04],
                r#type.map(|t| t.wasm_encode()).unwrap_or(vec![0x40]),
            ]
            .concat(),
            Instruction::Else => vec![0x05],
            Instruction::End => vec![0x0b],
            Instruction::Break(index) => [vec![0x0c], index.wasm_encode()].concat(),
            Instruction::Call(index) => [vec![0x10], index.wasm_encode()].concat(),
            Instruction::Drop => vec![0x1a],
            Instruction::LocalGetI32(index) => [vec![0x20], index.wasm_encode()].concat(),
            Instruction::LocalSetI32(index) => [vec![0x21], index.wasm_encode()].concat(),
            Instruction::LocalTeeI32(index) => [vec![0x22], index.wasm_encode()].concat(),
            Instruction::ConstI32(value) => [vec![0x41], value.wasm_encode()].concat(),
            Instruction::EqualI32 => vec![0x46],
            Instruction::NotEqualI32 => vec![0x47],
            Instruction::LessThanSignedI32 => vec![0x48],
            Instruction::GreaterThanSignedI32 => vec![0x4a],
            Instruction::LessThanOrEqualSignedI32 => vec![0x4c],
            Instruction::GreaterThanOrEqualSignedI32 => vec![0x4e],
            Instruction::AddI32 => vec![0x6a],
            Instruction::SubtractI32 => vec![0x6b],
            Instruction::MultiplyI32 => vec![0x6c],
            Instruction::DivideSignedI32 => vec![0x6d],
            Instruction::AndI32 => vec![0x71],
            Instruction::OrI32 => vec![0x72],
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
