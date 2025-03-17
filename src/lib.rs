use wasm::{Instruction, Module, WasmEncodable};

mod wasm;

fn compile_nop_lang(input: &str) -> Vec<u8> {
    if !input.is_empty() {
        panic!("Expected empty code, got {}", input);
    }

    let mut module = Module::default();
    let index = module.add_function(vec![Instruction::End]);
    module.export_function("main", index);

    module.wasm_encode()
}

#[cfg(test)]
mod tests {
    use wasmi::{Engine, Instance, Module, Store};

    use super::compile_nop_lang;

    fn create_wasmi_instance(wasm: &[u8]) -> Instance {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm).expect("couldn't parse module");
        let store = Store::new(&engine, 0);

        Instance::new(store, &module, &[]).expect("couldn't build instance")
    }

    #[test]
    fn should_compile_nop_lang() {
        let wasm = compile_nop_lang("");

        create_wasmi_instance(&wasm);
    }
}
