use crate::wasm::WasmEncodable;

use super::Section;

pub enum ExportDescription {
    Function(usize),
}

impl WasmEncodable for ExportDescription {
    fn wasm_encode(&self) -> Vec<u8> {
        match self {
            ExportDescription::Function(index) => [vec![0], index.wasm_encode()].concat(),
        }
    }
}

pub struct Export {
    name: String,
    description: ExportDescription,
}

impl WasmEncodable for Export {
    fn wasm_encode(&self) -> Vec<u8> {
        [self.name.wasm_encode(), self.description.wasm_encode()].concat()
    }
}

#[derive(Default)]
pub struct ExportSection {
    exports: Vec<Export>,
}

impl Section for ExportSection {
    type Contents = Vec<Export>;

    const ID: u8 = 7;

    fn contents(&self) -> &Self::Contents {
        &self.exports
    }
}

impl ExportSection {
    pub fn add_function(&mut self, name: &str, index: usize) {
        let export = Export {
            name: name.to_owned(),
            description: ExportDescription::Function(index),
        };

        self.exports.push(export);
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::WasmEncodable;

    use super::ExportSection;

    #[test]
    fn should_encode_export_section_with_one_export() {
        let mut section = ExportSection::default();
        section.add_function("main", 123);

        let wasm = section.wasm_encode();

        assert_eq!(wasm, vec![7, 8, 1, 4, 0x6d, 0x61, 0x69, 0x6e, 0, 123]);
    }
}
