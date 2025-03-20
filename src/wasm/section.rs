mod code;
mod export;
mod function;
mod import;
mod r#type;

pub use code::CodeSection;
pub use export::ExportSection;
pub use function::FunctionSection;
pub use r#type::TypeSection;

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
