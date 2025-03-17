const MAGIC: &[u8] = "\0asm".as_bytes();
const VERSION: &[u8] = &1u32.to_le_bytes();

fn compile_void_lang(input: &str) -> Vec<u8> {
    if !input.is_empty() {
        panic!("Expected empty code, got {}", input);
    }

    [MAGIC, VERSION].into_iter().flatten().copied().collect()
}

fn compile_nop_lang(input: &str) -> Vec<u8> {
    if !input.is_empty() {
        panic!("Expected empty code, got {}", input);
    }

    let type_section = [
        1, // section identifier
        4, // section size in bytes
        1, // number of entries that follow
        // entry 0
        0x60, // type "function"
        0,    // empty vector of parameters
        0,    // empty vector of return values
    ];

    let function_section = [
        3, // section identifier
        2, // section size in bytes
        1, // number of entries that follow
        // entry 0
        0, // index of the type section entry
    ];

    let code_section = [
        10, // section identifier
        4,  //section size in bytes
        1,  //number of entries that follow
        // entry 0
        2,  // entry size in bytes
        0,  // empty vector of local variables
        11, // "end" instruction
    ];

    [
        MAGIC,
        VERSION,
        &type_section,
        &function_section,
        &code_section,
    ]
    .into_iter()
    .flatten()
    .copied()
    .collect()
}

#[cfg(test)]
mod tests {
    use wasmi::{Engine, Instance, Module, Store};

    use super::{compile_nop_lang, compile_void_lang};

    fn create_wasmi_instance(wasm: &[u8]) -> Instance {
        let engine = Engine::default();
        let module = Module::new(&engine, wasm).expect("couldn't parse module");
        let store = Store::new(&engine, 0);

        Instance::new(store, &module, &[]).expect("couldn't build instance")
    }

    #[test]
    fn should_compile_void_lang() {
        let wasm = compile_void_lang("");

        create_wasmi_instance(&wasm);
    }

    #[test]
    fn should_compile_nop_lang() {
        let wasm = compile_nop_lang("");

        create_wasmi_instance(&wasm);
    }
}
