fn compile_void_lang(input: &str) -> Vec<u8> {
    if !input.is_empty() {
        panic!("Expected empty code, got {}", input);
    }

    vec![0, 97, 115, 109, 1, 0, 0, 0]
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
