use crate::wasm::WasmEncodable;

use super::Section;

pub enum ImportDescription {
    Function(usize),
}

impl WasmEncodable for ImportDescription {
    fn wasm_encode(&self) -> Vec<u8> {
        match self {
            ImportDescription::Function(index) => [vec![0], index.wasm_encode()].concat(),
        }
    }
}

pub struct Import {
    module_name: String,
    function_name: String,
    description: ImportDescription,
}

impl WasmEncodable for Import {
    fn wasm_encode(&self) -> Vec<u8> {
        [
            self.module_name.wasm_encode(),
            self.function_name.wasm_encode(),
            self.description.wasm_encode(),
        ]
        .concat()
    }
}

#[derive(Default)]
pub struct ImportSection {
    imports: Vec<Import>,
}

impl Section for ImportSection {
    type Contents = Vec<Import>;

    const ID: u8 = 2;

    fn contents(&self) -> &Self::Contents {
        &self.imports
    }
}

impl ImportSection {
    pub fn add_function(&mut self, module_name: &str, function_name: &str, index: usize) {
        let import = Import {
            module_name: module_name.to_string(),
            function_name: function_name.to_string(),
            description: ImportDescription::Function(index),
        };

        self.imports.push(import);
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::WasmEncodable;

    use super::ImportSection;

    #[test]
    fn should_encode_import_section_with_one_import() {
        let mut section = ImportSection::default();
        section.add_function("mod", "add", 123);

        assert_eq!(
            section.wasm_encode(),
            vec![2, 11, 1, 3, 0x6d, 0x6f, 0x64, 3, 0x61, 0x64, 0x64, 0, 123]
        );
    }
}
