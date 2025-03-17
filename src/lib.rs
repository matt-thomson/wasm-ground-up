const MAGIC: &[u8] = "\0asm".as_bytes();
const VERSION: &[u8] = &1u32.to_le_bytes();

fn compile_void_lang(input: &str) -> Vec<u8> {
    if !input.is_empty() {
        panic!("Expected empty code, got {}", input);
    }

    [MAGIC, VERSION].into_iter().flatten().copied().collect()
}

#[cfg(test)]
mod tests {
    use wasmi::{Engine, Instance, Module, Store};

    use super::compile_void_lang;

    #[test]
    fn should_compile_void_lang() {
        let wasm = compile_void_lang("");

        let engine = Engine::default();
        let module = Module::new(&engine, wasm).expect("couldn't parse module");
        let store = Store::new(&engine, 0);

        Instance::new(store, &module, &[]).expect("couldn't build instance");
    }
}
