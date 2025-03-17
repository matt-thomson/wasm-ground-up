use super::WasmEncodable;

#[derive(PartialEq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

impl WasmEncodable for ValueType {
    fn wasm_encode(&self) -> Vec<u8> {
        match self {
            ValueType::I32 => vec![0x7f],
            ValueType::I64 => vec![0x7e],
            ValueType::F32 => vec![0x7d],
            ValueType::F64 => vec![0x7c],
        }
    }
}
