mod instruction;
mod module;
pub mod section;

pub use instruction::Instruction;
pub use module::Module;

pub trait WasmEncodable {
    fn wasm_encode(&self) -> Vec<u8>;
}

impl WasmEncodable for u32 {
    fn wasm_encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        leb128::write::unsigned(&mut buffer, *self as u64).expect("failed to write LEB128");

        buffer
    }
}

impl WasmEncodable for usize {
    fn wasm_encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        leb128::write::unsigned(&mut buffer, *self as u64).expect("failed to write LEB128");

        buffer
    }
}

impl WasmEncodable for i32 {
    fn wasm_encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        leb128::write::signed(&mut buffer, *self as i64).expect("failed to write LEB128");

        buffer
    }
}

impl WasmEncodable for u8 {
    fn wasm_encode(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl<T> WasmEncodable for Vec<T>
where
    T: WasmEncodable,
{
    fn wasm_encode(&self) -> Vec<u8> {
        self.len()
            .wasm_encode()
            .into_iter()
            .chain(self.iter().flat_map(|x| x.wasm_encode()))
            .collect()
    }
}

impl WasmEncodable for String {
    fn wasm_encode(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self.bytes().collect();

        bytes.wasm_encode()
    }
}
