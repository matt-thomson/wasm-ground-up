mod code;
mod function;
mod r#type;

use super::WasmEncodable;

pub trait Section {
    type Contents: WasmEncodable;

    const ID: u8;

    fn contents(&self) -> &Self::Contents;
}

impl<T: Section> WasmEncodable for T {
    fn wasm_encode(&self) -> Vec<u8> {
        let contents = self.contents().wasm_encode();

        let mut result = vec![Self::ID];
        result.extend(contents.len().wasm_encode());
        result.extend(contents);

        result
    }
}
