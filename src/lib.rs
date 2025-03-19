use wafer::Wafer;
use wasm::{Module, ValueType, WasmEncodable};

mod wafer;
mod wasm;

pub fn compile(input: &str) -> Vec<u8> {
    let wafer = Wafer::parse(input);
    let mut module = Module::default();

    for function in wafer.functions {
        let index = module.add_function(
            function.parameters,
            vec![ValueType::I32],
            function.locals,
            function.instructions,
        );

        module.export_function(&function.name, index);
    }

    module.wasm_encode()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

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
    #[case("12 * 3", 36)]
    #[case("37 / 4", 9)]
    #[case("6 / (2 * 1)", 3)]
    #[case("1 + (2 * 4) / 3", 3)]
    #[case("let x = 123; let y = 456; 702", 702)]
    #[case("let a = 13; let b = 15; a := 10; a + b", 25)]
    fn should_compile_simple_cases_correctly(#[case] input: &str, #[case] expected: i32) {
        let input = format!("func main() {{ {input} }}");
        let wasm = compile(&input);
        let (mut store, instance) = create_wasmi_instance(&wasm);

        let func = instance
            .get_typed_func::<(), i32>(&mut store, "main")
            .expect("couldn't find function");

        let result = func.call(&mut store, ()).expect("couldn't call function");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("add", 579)]
    fn should_compile_fixtures_correctly(#[case] fixture_name: &str, #[case] expected: i32) {
        let input = read_to_string(format!("fixtures/{fixture_name}.wafer")).unwrap();
        let wasm = compile(&input);
        let (mut store, instance) = create_wasmi_instance(&wasm);

        let func = instance
            .get_typed_func::<(), i32>(&mut store, "main")
            .expect("couldn't find function");

        let result = func.call(&mut store, ()).expect("couldn't call function");
        assert_eq!(result, expected);
    }
}
