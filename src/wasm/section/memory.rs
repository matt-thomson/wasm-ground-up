use crate::wasm::WasmEncodable;

use super::Section;

pub enum Memory {
    Minimum(usize),
    MinimumAndMaximum(usize, usize),
}

impl WasmEncodable for Memory {
    fn wasm_encode(&self) -> Vec<u8> {
        match self {
            Memory::Minimum(min) => [vec![0x00], min.wasm_encode()].concat(),
            Memory::MinimumAndMaximum(min, max) => {
                [vec![0x01], min.wasm_encode(), max.wasm_encode()].concat()
            }
        }
    }
}

#[derive(Default)]
pub struct MemorySection {
    memories: Vec<Memory>,
}

impl Section for MemorySection {
    type Contents = Vec<Memory>;

    const ID: u8 = 5;

    fn contents(&self) -> &Self::Contents {
        &self.memories
    }
}

impl MemorySection {
    pub fn add(&mut self, min: usize, max: Option<usize>) {
        let memory = if let Some(max) = max {
            Memory::MinimumAndMaximum(min, max)
        } else {
            Memory::Minimum(min)
        };

        self.memories.push(memory);
    }
}

#[cfg(test)]
mod tests {
    use crate::wasm::WasmEncodable;

    use super::MemorySection;

    #[test]
    fn should_encode_memory_section_with_minimum() {
        let mut section = MemorySection::default();
        section.add(32, None);

        let wasm = section.wasm_encode();

        assert_eq!(wasm, vec![5, 3, 1, 0, 32]);
    }

    #[test]
    fn should_encode_memory_section_with_minimum_and_maximum() {
        let mut section = MemorySection::default();
        section.add(32, Some(64));

        let wasm = section.wasm_encode();

        assert_eq!(wasm, vec![5, 4, 1, 1, 32, 64]);
    }
}
