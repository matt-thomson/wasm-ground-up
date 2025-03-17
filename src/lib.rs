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

    fn create_wasmi_instance(wasm: &[u8]) -> (Store<u32>, Instance) {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm).expect("couldn't parse module");
        let mut store = Store::new(&engine, 0);

        let instance = Instance::new(&mut store, &module, &[]).expect("couldn't build instance");

        (store, instance)
    }

    #[test]
    fn should_compile_nop_lang() {
        let wasm = compile_nop_lang("");
        let (mut store, instance) = create_wasmi_instance(&wasm);

        let func = instance
            .get_typed_func::<(), ()>(&mut store, "main")
            .expect("couldn't find function");

        func.call(&mut store, ()).expect("couldn't call function");
    }
}
