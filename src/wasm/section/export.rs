use crate::wasm::WasmEncodable;

use super::Section;

pub enum ExportDescription {
    Function(usize),
    Memory(usize),
}

impl WasmEncodable for ExportDescription {
    fn wasm_encode(&self) -> Vec<u8> {
        match self {
            ExportDescription::Function(index) => [vec![0x00], index.wasm_encode()].concat(),
            ExportDescription::Memory(index) => [vec![0x02], index.wasm_encode()].concat(),
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
        self.add(name, ExportDescription::Function(index));
    }

    pub fn add_memory(&mut self, name: &str, index: usize) {
        self.add(name, ExportDescription::Memory(index));
    }

    fn add(&mut self, name: &str, description: ExportDescription) {
        let export = Export {
            name: name.to_owned(),
            description,
        };
        self.exports.push(export);
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::WasmEncodable;

    use super::ExportSection;

    #[test]
    fn should_encode_export_section() {
        let mut section = ExportSection::default();
        section.add_function("main", 123);
        section.add_memory("mem", 101);

        let wasm = section.wasm_encode();

        assert_eq!(
            wasm,
            vec![
                7, 14, 2, 4, 0x6d, 0x61, 0x69, 0x6e, 0, 123, 3, 0x6d, 0x65, 0x6d, 2, 101
            ]
        );
    }
}
