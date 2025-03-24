mod instruction;
mod module;
pub mod section;
mod value;

pub use instruction::Instruction;
pub use module::Module;
pub use value::ValueType;

pub trait WasmEncodable {
    fn wasm_encode(&self) -> Vec<u8>;
}

impl WasmEncodable for usize {
    fn wasm_encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        leb128::write::unsigned(&mut buffer, *self as u64).expect("failed to write LEB128");

        buffer
    }
}

impl WasmEncodable for u8 {
    fn wasm_encode(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl WasmEncodable for i32 {
    fn wasm_encode(&self) -> Vec<u8> {
        let mut buffer = vec![];
        leb128::write::signed(&mut buffer, i64::from(*self)).expect("failed to write LEB128");

        buffer
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
            .chain(self.iter().flat_map(WasmEncodable::wasm_encode))
            .collect()
    }
}

impl WasmEncodable for String {
    fn wasm_encode(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self.bytes().collect();
        [bytes.len().wasm_encode(), bytes].concat()
    }
}

impl<A, B> WasmEncodable for (A, B)
where
    A: WasmEncodable,
    B: WasmEncodable,
{
    fn wasm_encode(&self) -> Vec<u8> {
        let (a, b) = self;

        [a.wasm_encode(), b.wasm_encode()].concat()
    }
}
