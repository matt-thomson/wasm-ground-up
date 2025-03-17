use std::env::args;
use std::fs;
use std::path::PathBuf;

use wasm_ground_up::compile;

pub fn main() {
    let input_path = args().nth(1).expect("usage: wasm_ground_up <input path>");
    let input = fs::read_to_string(&input_path).expect("failed to read file");

    let wasm = compile(&input);

    let output_path = PathBuf::from(&input_path).with_extension("wasm");
    fs::write(output_path, wasm).expect("failed to write WASM");
}
