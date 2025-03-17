use parser::Wafer;
use wasm::{Module, ValueType, WasmEncodable};

mod parser;
mod wasm;

pub fn compile(input: &str) -> Vec<u8> {
    let wafer = Wafer::parse(input);
    let instructions = wafer.into_instructions();

    let mut module = Module::default();
    let index = module.add_function(vec![], vec![ValueType::I32], instructions);
    module.export_function("main", index);

    module.wasm_encode()
}

#[cfg(test)]
mod tests {
    use wasmi::{Engine, Instance, Module, Store};

    use super::compile;

    fn create_wasmi_instance(wasm: &[u8]) -> (Store<u32>, Instance) {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm).expect("couldn't parse module");
        let mut store = Store::new(&engine, 0);

        let instance = Instance::new(&mut store, &module, &[]).expect("couldn't build instance");

        (store, instance)
    }

    #[test]
    fn should_compile_number() {
        let wasm = compile("123");
        let (mut store, instance) = create_wasmi_instance(&wasm);

        let func = instance
            .get_typed_func::<(), i32>(&mut store, "main")
            .expect("couldn't find function");

        let x = func.call(&mut store, ()).expect("couldn't call function");
        assert_eq!(x, 123);
    }

    #[test]
    fn should_compile_expression() {
        let wasm = compile("123 + 456");
        let (mut store, instance) = create_wasmi_instance(&wasm);

        let func = instance
            .get_typed_func::<(), i32>(&mut store, "main")
            .expect("couldn't find function");

        let x = func.call(&mut store, ()).expect("couldn't call function");
        assert_eq!(x, 579);
    }
}
