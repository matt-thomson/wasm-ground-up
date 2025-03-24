use crate::wasm::{Instruction, WasmEncodable};

use super::Section;

pub struct Data {
    memory: usize,
    offset: usize,
    data: Vec<u8>,
}

impl WasmEncodable for Data {
    fn wasm_encode(&self) -> Vec<u8> {
        [
            self.memory.wasm_encode(),
            Instruction::ConstI32(self.offset as i32).wasm_encode(),
            Instruction::End.wasm_encode(),
            self.data.wasm_encode(),
        ]
        .concat()
    }
}

#[derive(Default)]
pub struct DataSection {
    data: Vec<Data>,
}

impl Section for DataSection {
    type Contents = Vec<Data>;

    const ID: u8 = 11;

    fn contents(&self) -> &Self::Contents {
        &self.data
    }
}

impl DataSection {
    pub fn add_segment(&mut self, memory: usize, offset: usize, data: Vec<u8>) {
        self.data.push(Data {
            memory,
            offset,
            data,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::WasmEncodable;

    use super::DataSection;

    #[test]
    fn should_encode_data_section_with_one_segment() {
        let mut section = DataSection::default();
        section.add_segment(0, 12, vec![0xde, 0xad, 0xbe, 0xef]);

        assert_eq!(
            section.wasm_encode(),
            vec![11, 10, 1, 0, 65, 12, 11, 4, 0xde, 0xad, 0xbe, 0xef]
        );
    }
}
