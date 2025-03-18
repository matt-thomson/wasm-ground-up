use super::WasmEncodable;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ValueType {
    I32,
}

impl WasmEncodable for ValueType {
    fn wasm_encode(&self) -> Vec<u8> {
        match self {
            ValueType::I32 => vec![0x7f],
        }
    }
}
