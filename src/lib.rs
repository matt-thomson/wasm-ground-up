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
    use rstest::rstest;
    use wasmi::{Engine, Instance, Module, Store};

    use super::compile;

    fn create_wasmi_instance(wasm: &[u8]) -> (Store<u32>, Instance) {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm).expect("couldn't parse module");
        let mut store = Store::new(&engine, 0);

        let instance = Instance::new(&mut store, &module, &[]).expect("couldn't build instance");

        (store, instance)
    }

    #[rstest]
    #[case("123", 123)]
    #[case("123 + 456", 579)]
    #[case("456 - 123", 333)]
    #[case("7 - 3 + 11", 15)]
    fn should_compile_correctly(#[case] input: &str, #[case] expected: i32) {
        let wasm = compile(input);
        let (mut store, instance) = create_wasmi_instance(&wasm);

        let func = instance
            .get_typed_func::<(), i32>(&mut store, "main")
            .expect("couldn't find function");

        let x = func.call(&mut store, ()).expect("couldn't call function");
        assert_eq!(x, expected);
    }
}
