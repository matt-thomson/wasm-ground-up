mod section;

pub trait WasmEncodable {
    fn wasm_encode(&self) -> Vec<u8>;
}

impl WasmEncodable for u32 {
    fn wasm_encode(&self) -> Vec<u8> {
        if *self < 128 {
            vec![*self as u8]
        } else {
            unimplemented!()
        }
    }
}

impl WasmEncodable for usize {
    fn wasm_encode(&self) -> Vec<u8> {
        if *self < 128 {
            vec![*self as u8]
        } else {
            unimplemented!()
        }
    }
}

impl WasmEncodable for i32 {
    fn wasm_encode(&self) -> Vec<u8> {
        if *self >= 0 && *self < 64 {
            vec![*self as u8]
        } else {
            unimplemented!()
        }
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
